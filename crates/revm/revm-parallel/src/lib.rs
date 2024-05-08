//! Implementation of parallel execution based on revm.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
// TODO:
// #![warn(missing_debug_implementations, missing_docs, unreachable_pub, rustdoc::all)]
// #![deny(unused_must_use, rust_2018_idioms, unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod executor;
pub mod factory;
pub mod queue;
pub mod read_inspector;
pub mod rw_set;
pub mod shared;

mod utils;
pub use utils::resolve_block_dependencies;
