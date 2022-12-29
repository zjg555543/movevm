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

