pub mod account;
pub mod helpers;

use move_core_types::{account_address::AccountAddress, identifier::Identifier};
use move_vm_runtime::native_functions::{make_table_from_iter, NativeFunctionTable};
use move_vm_types::values::Value;
use crate::gas::gas::AbstractValueSize;
use crate::adapt::vm::types::{GasInfo, Storage, Querier};


use std::sync::Arc;
use std::sync::Mutex;

pub mod status {
    // Failure in parsing a struct type tag
    pub const NFE_EXPECTED_STRUCT_TYPE_TAG: u64 = 0x1;
    // Failure in address parsing (likely no correct length)
    pub const NFE_UNABLE_TO_PARSE_ADDRESS: u64 = 0x2;
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub account: account::GasParameters,
}

impl GasParameters {
    pub fn zeros() -> Self {
        Self {
            account: account::GasParameters {
                create_signer: account::CreateSignerGasParameters { base: 0.into() },
                get_amount: account::GetAmountGasParameters { base: 0.into()},
                transfer_amount: account::TransferAmountGasParameters { base: 0.into() },
            },
        }
    }
}
pub fn all_natives(
    querier: Arc<Mutex<Box<dyn Querier + Send +'static>>>,
    framework_addr: AccountAddress,
    gas_params: GasParameters,
) -> NativeFunctionTable {
    let mut natives = vec![];

    macro_rules! add_natives_from_module {
        ($module_name: expr, $natives: expr) => {
            natives.extend(
                $natives.map(|(func_name, func)| ($module_name.to_string(), func_name, func)),
            );
        };
    }

    add_natives_from_module!("Account", account::make_all(querier, gas_params.account.clone()));

    make_table_from_iter(framework_addr, natives)
}

/// A temporary hack to patch Table -> table module name as long as it is not upgraded
/// in the Move repo.
pub fn patch_table_module(table: NativeFunctionTable) -> NativeFunctionTable {
    table
        .into_iter()
        .map(|(m, _, f, i)| (m, Identifier::new("table").unwrap(), f, i))
        .collect()
}
