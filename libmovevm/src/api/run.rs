use crate::{
    api::utils::{
        contains_module, explain_execution_effects, explain_execution_error, get_gas_status,
        is_bytecode_file, maybe_commit_effects, on_disk_state_view::OnDiskStateView,
    },
    NativeFunctionRecord,
};
use anyhow::{anyhow, bail, Result};
use move_binary_format::file_format::CompiledModule;
use move_command_line_common::env::get_bytecode_version_from_env;
use move_core_types::{
    account_address::AccountAddress,
    errmap::ErrorMapping,
    identifier::IdentStr,
    language_storage::TypeTag,
    transaction_argument::{convert_txn_args, TransactionArgument},
    value::MoveValue,
};
use move_package::compilation::compiled_package::CompiledPackage;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_test_utils::gas_schedule::CostTable;
use std::{fs, path::Path};

pub fn run(
    script_bytes_params: Vec<u8>,
    natives: impl IntoIterator<Item = NativeFunctionRecord>,
    cost_table: &CostTable,
    error_descriptions: &ErrorMapping,
    state: &mut OnDiskStateView,
    signers: &[String],
    txn_args: &[TransactionArgument],
    vm_type_args: Vec<TypeTag>,
    gas_budget: Option<u64>,
    verbose: bool,
) -> Result<()> {
    println!("run---debug------1");
    let signer_addresses = signers
        .iter()
        .map(|s| AccountAddress::from_hex_literal(s))
        .collect::<Result<Vec<AccountAddress>, _>>()?;
    // TODO: parse Value's directly instead of going through the indirection of TransactionArgument?
    let vm_args: Vec<Vec<u8>> = convert_txn_args(txn_args);
    println!("run---debug------5, {:?},{:?}{:?}", signer_addresses, signers, gas_budget);
    let vm = MoveVM::new(natives).unwrap();
    let mut gas_status = get_gas_status(cost_table, gas_budget)?;
    let mut session = vm.new_session(state);

    let script_type_parameters = vec![];
    let script_parameters = vec![];
    let vm_args = signer_addresses
        .iter()
        .map(|a| {
            MoveValue::Signer(*a)
                .simple_serialize()
                .expect("transaction arguments must serialize")
        })
        .chain(vm_args)
        .collect();
    println!("run---debug------6");

    let res = session.execute_script(
        script_bytes_params.clone(),
        vm_type_args.clone(),
        vm_args,
        &mut gas_status,
    );

    if let Err(err) = res {
        println!("run---debug------9");
        explain_execution_error(
            error_descriptions,
            err,
            state,
            &script_type_parameters,
            &script_parameters,
            &vm_type_args,
            &signer_addresses,
            txn_args,
        )
    } else {
        println!("run---debug------10");
        let (changeset, events) = session.finish().map_err(|e| e.into_vm_status())?;
        if verbose {
            explain_execution_effects(&changeset, &events, state)?
        }
        maybe_commit_effects(true, changeset, events, state)
    }
}

