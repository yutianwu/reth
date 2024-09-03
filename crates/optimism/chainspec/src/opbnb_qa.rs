//! Chain specification for the Opbnb QA network.

#[cfg(not(feature = "std"))]
use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Arc;

use alloy_chains::Chain;
use alloy_primitives::{b256, U256};
use once_cell::sync::Lazy;
use reth_chainspec::{BaseFeeParams, BaseFeeParamsKind, ChainSpec};
use reth_ethereum_forks::{EthereumHardfork, OptimismHardfork};

use crate::OpChainSpec;

/// The opbnb qa spec
pub static OPBNB_QA: Lazy<Arc<OpChainSpec>> = Lazy::new(|| {
    OpChainSpec {
        inner: ChainSpec {
            chain: Chain::from_id(4530),
            genesis: serde_json::from_str(include_str!("../res/genesis/opbnb_qa.json"))
                .expect("Can't deserialize opBNB qa genesis json"),
            genesis_hash: Some(b256!(
                "9c6edc17c0f3a16df0e9384bb84a4f4847d56a32e5ccaaa10a0be93b83a9698e"
            )),
            paris_block_and_final_difficulty: Some((0, U256::from(0))),
            hardforks: OptimismHardfork::opbnb_qa(),
            base_fee_params: BaseFeeParamsKind::Variable(
                vec![(EthereumHardfork::London.boxed(), BaseFeeParams::ethereum())].into(),
            ),
            prune_delete_limit: 0,
            ..Default::default()
        },
    }
    .into()
});
