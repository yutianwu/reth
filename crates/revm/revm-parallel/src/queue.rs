//! The lists representing the order of execution of the block.

use crate::rw_set::{BlockRWSet, TransitionRWSet};
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use reth_primitives::{BlockNumber, TransitionId};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs,
    ops::{IndexMut, RangeInclusive},
    path::PathBuf,
};

/// The batch of transition ids that can be executed in parallel.
#[derive(
    Deref, DerefMut, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Default, Debug,
)]
pub struct TransitionBatch {
    /// Collection of transition ids.
    #[deref]
    #[deref_mut]
    pub transitions: Vec<TransitionId>,
    /// The gas used by this batch.
    pub gas_used: u128,
}

impl TransitionBatch {
    /// Create new transition batch.
    pub fn new(transitions: Vec<TransitionId>, gas_used: u128) -> Self {
        Self { transitions, gas_used }
    }
}

impl From<(Vec<TransitionId>, u128)> for TransitionBatch {
    fn from((transitions, gas_used): (Vec<TransitionId>, u128)) -> Self {
        Self::new(transitions, gas_used)
    }
}

/// The queue of transition lists that represent the order of execution for the block.
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Debug)]
pub struct TransitionQueue {
    range: RangeInclusive<BlockNumber>,
    batches: Vec<TransitionBatch>,
}

impl TransitionQueue {
    /// Create new transition queue for block range.
    pub fn new(range: RangeInclusive<BlockNumber>) -> Self {
        Self { range, batches: Vec::new() }
    }

    /// Set transitions batches.
    pub fn with_batches(
        mut self,
        batches: impl IntoIterator<Item = (Vec<TransitionId>, u128)>,
    ) -> Self {
        self.batches = batches.into_iter().map(TransitionBatch::from).collect();
        self
    }

    /// Resolve transition queue from an ordered list of block transition rw sets.
    pub fn resolve(
        range: RangeInclusive<BlockNumber>,
        sets: HashMap<BlockNumber, BlockRWSet>,
    ) -> Self {
        let mut this = Self::new(range);
        let mut batch_rw_sets = HashMap::<usize, TransitionRWSet>::default();

        for (block_number, block_rw_set) in
            sets.into_iter().sorted_unstable_by_key(|(block, _)| *block)
        {
            tracing::trace!(target: "evm::parallel::resolve", block_number, "Resolving dependencies");
            for (id, rw_set) in block_rw_set.into_transitions(block_number) {
                let depth =
                    this.find_highest_dependency(&rw_set, &batch_rw_sets).map_or(0, |dep| dep + 1);

                if depth >= this.batches.len() {
                    this.append_transition(id, rw_set.gas_used);
                } else {
                    let batch = this.batches.index_mut(depth);
                    batch.push(id);
                    batch.gas_used += rw_set.gas_used as u128;
                }

                batch_rw_sets.entry(depth).or_default().extend(rw_set);
            }
        }

        this
    }

    /// Return transaction batches.
    pub fn batches(&self) -> &[TransitionBatch] {
        &self.batches
    }

    /// Find dependency with the highest index in the queue.
    /// Returns [None] if transition is independent.
    ///
    ///
    /// # Panics
    ///
    /// - If the queue contains a transition id that has no corresponding rw set.
    pub fn find_highest_dependency(
        &self,
        target: &TransitionRWSet,
        batch_rw_sets: &HashMap<usize, TransitionRWSet>,
    ) -> Option<usize> {
        // Iterate over the list in reverse to find dependency with the highest index.
        for (queue_depth, _transition_list) in self.batches.iter().enumerate().rev() {
            let batch_set = batch_rw_sets.get(&queue_depth).unwrap();
            // The dependency check has to be bidirectional since the target
            // transition might modify the state in a way that affects the reads
            // of the transition we are currently checking.
            if target.depends_on(batch_set) || batch_set.depends_on(target) {
                return Some(queue_depth)
            }
        }

        None
    }

    /// Appends transition as a separate batch to the queue.
    pub fn append_transition(&mut self, id: TransitionId, gas_used: u64) {
        self.batches.push(TransitionBatch::new(Vec::from([id]), gas_used as u128))
    }

    /// Appends transition batch to the queue.
    pub fn append_batch(&mut self, batch: TransitionBatch) {
        self.batches.push(batch);
    }
}

/// The collection of transitions queues by block number.
#[derive(Debug)]
pub struct TransitionQueueStore {
    dir: PathBuf,
}

impl TransitionQueueStore {
    /// Create new store at a given path.
    pub fn new(dir: PathBuf) -> Self {
        Self { dir }
    }

    /// Load transition queue.
    pub fn load(
        &self,
        range: RangeInclusive<BlockNumber>,
    ) -> eyre::Result<Option<TransitionQueue>> {
        let mut matching = Vec::new();
        for path in fs::read_dir(&self.dir)? {
            let path = path?;
            if !path.metadata()?.is_dir() {
                let parsed = parse_block_range(&path.file_name());
                tracing::trace!(
                    target: "evm::parallel::store",
                    path = ?path.file_name(),
                    ?parsed,
                    "Parsing transition file"
                );
                if let Some(file_range) = parse_block_range(&path.file_name()) {
                    if range.contains(file_range.start()) || range.contains(file_range.end()) {
                        matching.push((file_range, path.path()));
                    }
                }
            }
        }

        if matching.is_empty() {
            tracing::trace!(target: "evm::parallel::store", dir = %self.dir.display(), "No transition queue files found");
            return Ok(None)
        }

        matching.sort_by_key(|(range, _)| *range.start());

        // Check that files cover the requested range.
        let full_file_range = matching
            .first()
            .zip(matching.last())
            .map(|((first, _), (last, _))| *first.start()..=*last.end())
            .unwrap();
        if !full_file_range.contains(range.start()) || !full_file_range.contains(range.end()) {
            tracing::trace!(
                target: "evm::parallel::store",
                requested_range = ?range,
                file_range = ?full_file_range,
                "Transition queue files do not cover the requested range"
            );
            return Ok(None)
        }

        let mut queue = TransitionQueue::new(range.clone());
        for (file_range, path) in matching {
            let mut loaded: TransitionQueue = serde_json::from_str(&fs::read_to_string(path)?)?;
            debug_assert_eq!(loaded.range, file_range);
            if range.contains(file_range.start()) && range.contains(file_range.end()) {
                queue.batches.append(&mut loaded.batches);
            } else {
                for batch in loaded.batches {
                    let transitions = batch
                        .transitions
                        .into_iter()
                        .filter(|id| range.contains(&id.0))
                        .collect::<Vec<_>>();
                    if !transitions.is_empty() {
                        queue.append_batch(TransitionBatch::new(
                            transitions,
                            batch.gas_used, // TODO: invalid
                        ));
                    }
                }
            }
        }
        Ok(Some(queue))
    }

    /// Save a queue to the queue store.
    pub fn save(&self, queue: TransitionQueue) -> eyre::Result<()> {
        let filename = format!("parallel-{}-{}.json", queue.range.start(), queue.range.end());
        let path = self.dir.join(filename);
        tracing::trace!(target: "evm::parallel::store", range = ?queue.range, path = %path.display(), "Saving transition queue");
        fs::write(path, serde_json::to_string(&queue)?)?;
        Ok(())
    }
}

fn parse_block_range(filename: &OsStr) -> Option<RangeInclusive<BlockNumber>> {
    let filename = filename.to_str()?;
    let range = filename.strip_prefix("parallel-")?;
    let range = range.strip_suffix(".json")?;
    let mut range = range.split('-');
    let start = range.next()?;
    let end = range.next()?;
    Some(start.parse::<u64>().ok()?..=end.parse::<u64>().ok()?)
}

// TODO: fix tests
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::rw_set::{RevmAccessSet, RevmAccountDataKey, RevmKey};
//     use reth_primitives::Address;

//     #[test]
//     #[should_panic]
//     fn highest_dependency_target_out_of_bounds() {
//         let id = TransitionId::transaction(0, 0);
//         assert_eq!(
//             TransitionQueue::default().find_highest_dependency(id, &BTreeMap::default()),
//             None
//         );
//     }

//     #[test]
//     #[should_panic]
//     fn highest_dependency_queue_item_out_of_bounds() {
//         let mut queue = TransitionQueue::default();
//         queue.append_transition(TransitionId::transaction(0, 1));
//         assert_eq!(
//             queue.find_highest_dependency(
//                 TransitionId::transaction(0, 0),
//                 &BTreeMap::from_iter([(0, BlockRWSet::default())])
//             ),
//             None
//         );
//     }

//     #[test]
//     fn highest_dependency() {
//         let queue = TransitionQueue::default();
//         assert_eq!(queue.find_highest_dependency(0, &[TransitionRWSet::default()]), None);

//         let account_balance_key = RevmKey::Account(Address::random(),
// RevmAccountDataKey::Balance);         let sets = Vec::from([
//
// TransitionRWSet::default().with_write_set(RevmAccessSet::from([account_balance_key])),
//
// TransitionRWSet::default().with_read_set(RevmAccessSet::from([account_balance_key])),
//         ]);
//         let queue = TransitionQueue::from([vec![0]]);
//         assert_eq!(queue.find_highest_dependency(1, &sets), Some(0));
//     }

//     #[test]
//     fn resolve() {
//         let account_balance_key = RevmKey::Account(Address::random(),
// RevmAccountDataKey::Balance);         let account_nonce_key =
// RevmKey::Account(Address::random(), RevmAccountDataKey::Nonce);         let tx_sets =
// Vec::from([             // 0: first hence independent
//
// TransitionRWSet::default().with_write_set(RevmAccessSet::from([account_balance_key])),
//             // 1: independent
//             TransitionRWSet::default(),
//             // 2: depends on 0
//             TransitionRWSet::default()
//                 .with_read_set(RevmAccessSet::from([account_balance_key]))
//                 .with_write_set(RevmAccessSet::from([account_nonce_key])),
//             // 3: independent
//             TransitionRWSet::default(),
//             // 4: depends on 0
//
// TransitionRWSet::default().with_read_set(RevmAccessSet::from([account_nonce_key])),
//             // 5: depends on 0, 2
//             TransitionRWSet::default()
//                 .with_read_set(RevmAccessSet::from([account_balance_key,
// account_nonce_key])),         ]);

//         // [0, 1, 3]
//         // [2]
//         // [4, 5]
//         let expected_queue = TransitionQueue::from([vec![0, 1, 3], vec![2], vec![4, 5]]);
//         assert_eq!(TransitionQueue::resolve(&tx_sets), expected_queue);
//     }
// }
