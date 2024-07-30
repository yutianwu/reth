//! Commonly used types in reth.
//!
//! This crate contains Ethereum primitive types and helper functions.
//!
//! ## Feature Flags
//!
//! - `alloy-compat`: Adds compatibility conversions for certain alloy types.
//! - `arbitrary`: Adds `proptest` and `arbitrary` support for primitive types.
//! - `test-utils`: Export utilities for testing

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/paradigmxyz/reth/main/assets/reth-docs.png",
    html_favicon_url = "https://avatars0.githubusercontent.com/u/97369466?s=256",
    issue_tracker_base_url = "https://github.com/paradigmxyz/reth/issues/"
)]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

pub use alloy_primitives::{
    self, address, Address, B128, b256,
    B256,
    B512, B64, BlockHash, BlockNumber, bloom,
    Bloom,
    BloomInput, bytes, bytes::{Buf, BufMut, BytesMut}, Bytes, ChainId, eip191_hash_message, hex, hex_literal, keccak256,
    ruint, Selector, StorageKey, StorageValue, TxHash, TxIndex, TxKind, TxNumber, U128, U256, U64, U8, utils::format_ether,
};
#[cfg(any(test, feature = "arbitrary"))]
pub use arbitrary;
#[cfg(feature = "c-kzg")]
pub use c_kzg as kzg;
pub use revm_primitives::{self, JumpTable};

#[cfg(any(test, feature = "arbitrary"))]
pub use block::{generate_valid_header, valid_header_strategy};
pub use block::{
    Block, BlockBody, BlockHashOrNumber, BlockId, BlockNumberOrTag, BlockNumHash, BlockWithSenders,
    ForkBlock, RpcBlockHash, SealedBlock, SealedBlockWithSenders,
};
#[cfg(feature = "bsc")]
pub use bsc::*;
#[cfg(feature = "zstd-codec")]
pub use compression::*;
pub use constants::{
    DEV_GENESIS_HASH, EMPTY_OMMER_ROOT_HASH, HOLESKY_GENESIS_HASH, KECCAK_EMPTY,
    MAINNET_GENESIS_HASH, SEPOLIA_GENESIS_HASH,
};
pub use genesis::{ChainConfig, Genesis, GenesisAccount};
#[cfg(feature = "optimism")]
pub use optimism::*;
pub use receipt::{
    gas_spent_by_transactions, Receipt, Receipts, ReceiptWithBloom, ReceiptWithBloomRef,
};
pub use reth_ethereum_forks::*;
pub use reth_primitives_traits::{
    Account, BlobSidecar, BlobSidecars, Bytecode, GotExpected, GotExpectedBoxed, Header,
    HeaderError, Log, LogData, logs_bloom, Request, Requests, SealedHeader, StorageEntry, Withdrawal,
    Withdrawals,
};
pub use reth_static_file_types as static_file;
pub use static_file::StaticFileSegment;
pub use transaction::{
    BlobTransaction, BlobTransactionSidecar, FromRecoveredPooledTransaction,
    PooledTransactionsElement, PooledTransactionsElementEcRecovered,
};
pub use transaction::{
    AccessList,
    AccessListItem, EIP1559_TX_TYPE_ID, EIP2930_TX_TYPE_ID, EIP4844_TX_TYPE_ID, IntoRecoveredTransaction,
    InvalidTransactionError, LEGACY_TX_TYPE_ID, Signature, Transaction,
    TransactionMeta, TransactionSigned, TransactionSignedEcRecovered, TransactionSignedNoHash, TryFromRecoveredTransaction,
    TxEip1559, TxEip2930, TxEip4844, TxHashOrNumber, TxLegacy, TxType,
    util::secp256k1::{public_key_to_address, recover_signer_unchecked, sign_message},
};
#[cfg(feature = "c-kzg")]
pub use transaction::BlobTransactionValidationError;

// Re-exports
pub use self::ruint::UintTryTo;

#[cfg(feature = "alloy-compat")]
mod alloy_compat;
pub mod basefee;
mod block;
#[cfg(feature = "zstd-codec")]
mod compression;
pub mod constants;
pub mod eip4844;
pub mod genesis;
pub mod proofs;
mod receipt;
pub mod parlia;
pub mod system_contracts;
pub mod transaction;

#[doc(hidden)]
#[deprecated = "use B64 instead"]
pub type H64 = B64;
#[doc(hidden)]
#[deprecated = "use B128 instead"]
pub type H128 = B128;
#[doc(hidden)]
#[deprecated = "use Address instead"]
pub type H160 = Address;
#[doc(hidden)]
#[deprecated = "use B256 instead"]
pub type H256 = B256;
#[doc(hidden)]
#[deprecated = "use B512 instead"]
pub type H512 = B512;

/// Optimism specific re-exports
#[cfg(feature = "optimism")]
mod optimism {
    pub use reth_chainspec::{BASE_MAINNET, BASE_SEPOLIA, OP_MAINNET, OP_SEPOLIA};
    #[cfg(feature = "opbnb")]
    pub use reth_chainspec::{OPBNB_MAINNET, OPBNB_QA, OPBNB_TESTNET};

    pub use crate::transaction::{DEPOSIT_TX_TYPE_ID, TxDeposit};
}

/// Bsc specific re-exports
#[cfg(feature = "bsc")]
mod bsc {
    pub use reth_chainspec::{BSC_MAINNET, BSC_TESTNET};
}

