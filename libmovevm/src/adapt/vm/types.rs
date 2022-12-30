use std::fmt::Debug;
use std::ops::AddAssign;
use std::string::FromUtf8Error;
use thiserror::Error;
use std::fmt;
use std::ops::Deref;

// use schemars::JsonSchema;
use serde::{de, ser, Deserialize, Deserializer, Serialize};


use cosmwasm_std::{Binary, ContractResult, SystemResult};

pub type Record<V = Vec<u8>> = (Vec<u8>, V);


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct GasInfo {
    /// The gas cost of a computation that was executed already but not yet charged.
    ///
    /// This could be renamed to `internally_used` for consistency because it is used inside
    /// of the `cosmwasm_vm`.
    pub cost: u64,
    /// Gas that was used and charged externally. This is needed to
    /// adjust the VM's gas limit but does not affect the gas usage.
    pub externally_used: u64,
}

impl GasInfo {
    pub fn new(cost: u64, externally_used: u64) -> Self {
        GasInfo {
            cost,
            externally_used,
        }
    }

    pub fn with_cost(amount: u64) -> Self {
        GasInfo {
            cost: amount,
            externally_used: 0,
        }
    }

    pub fn with_externally_used(amount: u64) -> Self {
        GasInfo {
            cost: 0,
            externally_used: amount,
        }
    }

    /// Creates a gas information with no cost for the caller and with zero externally used gas.
    ///
    /// Caution: when using this you need to make sure no gas was metered externally to keep the gas values in sync.
    pub fn free() -> Self {
        GasInfo {
            cost: 0,
            externally_used: 0,
        }
    }
}

impl AddAssign for GasInfo {
    fn add_assign(&mut self, other: Self) {
        *self = GasInfo {
            cost: self.cost + other.cost,
            externally_used: self.externally_used + other.externally_used,
        };
    }
}

pub type BackendResult<T> = (core::result::Result<T, BackendError>, GasInfo);

#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackendError {
    #[error("Panic in FFI call")]
    ForeignPanic {},
    #[error("Bad argument")]
    BadArgument {},
    #[error("VM received invalid UTF-8 data from backend")]
    InvalidUtf8 {},
    #[error("Iterator with ID {id} does not exist")]
    IteratorDoesNotExist { id: u32 },
    #[error("Ran out of gas during call into backend")]
    OutOfGas {},
    #[error("Unknown error during call into backend: {msg}")]
    Unknown { msg: String },
    // This is the only error case of BackendError that is reported back to the contract.
    #[error("User error during call into backend: {msg}")]
    UserErr { msg: String },
}

impl BackendError {
    pub fn foreign_panic() -> Self {
        BackendError::ForeignPanic {}
    }

    pub fn bad_argument() -> Self {
        BackendError::BadArgument {}
    }

    pub fn iterator_does_not_exist(iterator_id: u32) -> Self {
        BackendError::IteratorDoesNotExist { id: iterator_id }
    }

    pub fn out_of_gas() -> Self {
        BackendError::OutOfGas {}
    }

    pub fn unknown(msg: impl Into<String>) -> Self {
        BackendError::Unknown { msg: msg.into() }
    }

    pub fn user_err(msg: impl Into<String>) -> Self {
        BackendError::UserErr { msg: msg.into() }
    }
}

impl From<FromUtf8Error> for BackendError {
    fn from(_original: FromUtf8Error) -> Self {
        BackendError::InvalidUtf8 {}
    }
}

pub trait BackendApi: Copy + Clone + Send {
    fn canonical_address(&self, human: &str) -> BackendResult<Vec<u8>>;
    fn human_address(&self, canonical: &[u8]) -> BackendResult<String>;
}



pub trait Querier {
    /// This is all that must be implemented for the Querier.
    /// This allows us to pass through binary queries from one level to another without
    /// knowing the custom format, or we can decode it, with the knowledge of the allowed
    /// types.
    ///
    /// The gas limit describes how much VM gas this particular query is allowed
    /// to comsume when measured separately from the rest of the contract.
    /// The returned gas info (in BackendResult) can exceed the gas limit in cases
    /// where the query could not be aborted exactly at the limit.
    fn query_raw(
        &self,
        request: &[u8],
        gas_limit: u64,
    ) -> BackendResult<SystemResult<ContractResult<Binary>>>;
}

pub trait Storage {
    /// Returns Err on error.
    /// Returns Ok(None) when key does not exist.
    /// Returns Ok(Some(Vec<u8>)) when key exists.
    ///
    /// Note: Support for differentiating between a non-existent key and a key with empty value
    /// is not great yet and might not be possible in all backends. But we're trying to get there.
    fn get(&self, key: &[u8]) -> BackendResult<Option<Vec<u8>>>;

    /// Allows iteration over a set of key/value pairs, either forwards or backwards.
    /// Returns an interator ID that is unique within the Storage instance.
    ///
    /// The bound `start` is inclusive and `end` is exclusive.
    ///
    /// If `start` is lexicographically greater than or equal to `end`, an empty range is described, mo matter of the order.
    ///
    /// This call must not change data in the storage, but creating and storing a new iterator can be a mutating operation on
    /// the Storage implementation.
    /// The implementation must ensure that iterator IDs are assigned in a deterministic manner as this is
    /// environment data that is injected into the contract.
    #[cfg(feature = "iterator")]

    fn scan(
        &mut self,
        start: Option<&[u8]>,
        end: Option<&[u8]>,
        order: Order,
    ) -> BackendResult<u32>;

    /// Returns the next element of the iterator with the given ID.
    ///
    /// If the ID is not found, a BackendError::IteratorDoesNotExist is returned.
    ///
    /// This call must not change data in the storage, but incrementing an iterator can be a mutating operation on
    /// the Storage implementation.
    #[cfg(feature = "iterator")]
    fn next(&mut self, iterator_id: u32) -> BackendResult<Option<Record>>;

    fn set(&mut self, key: &[u8], value: &[u8]) -> BackendResult<()>;

    /// Removes a database entry at `key`.
    ///
    /// The current interface does not allow to differentiate between a key that existed
    /// before and one that didn't exist. See https://github.com/CosmWasm/cosmwasm/issues/290
    fn remove(&mut self, key: &[u8]) -> BackendResult<()>;
}


