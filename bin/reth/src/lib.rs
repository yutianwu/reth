//! Rust Ethereum (reth) binary executable.
//!
//! ## Feature Flags
//!
//! - `jemalloc`: Uses [jemallocator](https://github.com/tikv/jemallocator) as the global allocator.
//!   This is **not recommended on Windows**. See [here](https://rust-lang.github.io/rfcs/1974-global-allocators.html#jemalloc)
//!   for more info.
//! - `jemalloc-prof`: Enables [jemallocator's](https://github.com/tikv/jemallocator) heap profiling
//!   and leak detection functionality. See [jemalloc's opt.prof](https://jemalloc.net/jemalloc.3.html#opt.prof)
//!   documentation for usage details. This is **not recommended on Windows**. See [here](https://rust-lang.github.io/rfcs/1974-global-allocators.html#jemalloc)
//!   for more info.
//! - `min-error-logs`: Disables all logs below `error` level.
//! - `min-warn-logs`: Disables all logs below `warn` level.
//! - `min-info-logs`: Disables all logs below `info` level. This can speed up the node, since fewer
//!   calls to the logging component is made.
//! - `min-debug-logs`: Disables all logs below `debug` level.
//! - `min-trace-logs`: Disables all logs below `trace` level.

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![warn(missing_docs, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

pub mod args;
pub mod chain;
pub mod cli;
pub mod config;
pub mod db;
pub mod debug_cmd;
pub mod dirs;
pub mod init;
pub mod node;
pub mod p2p;
pub mod parallel_execution;
pub mod prometheus_exporter;
pub mod recover;
pub mod runner;
pub mod stage;
pub mod test_vectors;
pub mod utils;
pub mod version;

/// Re-exported from `reth_provider`.
pub mod providers {
    pub use reth_provider::*;
}

/// Re-exported from `reth_primitives`.
pub mod primitives {
    pub use reth_primitives::*;
}

/// Re-exported from `reth_beacon_consensus`.
pub mod beacon_consensus {
    pub use reth_beacon_consensus::*;
}
/// Re-exported from `reth_blockchain_tree`.
pub mod blockchain_tree {
    pub use reth_blockchain_tree::*;
}

/// Re-exported from `reth_consensus_common`.
pub mod consensus_common {
    pub use reth_consensus_common::*;
}

/// Re-exported from `reth_revm`.
pub mod revm {
    pub use reth_revm::*;
}

/// Re-exported from `reth_tasks`.
pub mod tasks {
    pub use reth_tasks::*;
}

/// Re-exported from `reth_network`.
pub mod network {
    pub use reth_network::*;
    pub use reth_network_api::{noop, reputation, NetworkInfo, PeerKind, Peers, PeersInfo};
}

/// Re-exported from `reth_transaction_pool`.
pub mod transaction_pool {
    pub use reth_transaction_pool::*;
}

/// Re-export of `reth_rpc_*` crates.
pub mod rpc {

    /// Re-exported from `reth_rpc_builder`.
    pub mod builder {
        pub use reth_rpc_builder::*;
    }

    /// Re-exported from `reth_rpc_types`.
    pub mod types {
        pub use reth_rpc_types::*;
    }

    /// Re-exported from `reth_rpc_api`.
    pub mod api {
        pub use reth_rpc_api::*;
    }
    /// Re-exported from `reth_rpc::eth`.
    pub mod eth {
        pub use reth_rpc::eth::*;
    }

    /// Re-exported from `reth_rpc::rpc`.
    pub mod result {
        pub use reth_rpc::result::*;
    }

    /// Re-exported from `reth_rpc::eth`.
    pub mod compat {
        pub use reth_rpc_types_compat::*;
    }
}

#[cfg(all(feature = "jemalloc", unix))]
use jemallocator as _;

// for rendering diagrams
use aquamarine as _;
