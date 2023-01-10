use crate::{
    api::utils::{
        explain_publish_changeset, get_gas_status, module,
        on_disk_state_view::OnDiskStateView,
    },
    NativeFunctionRecord,
};
use anyhow::{bail, Result};
use move_binary_format::errors::Location;
use move_command_line_common::env::get_bytecode_version_from_env;
use move_package::compilation::compiled_package::CompiledPackage;
use move_vm_runtime::move_vm::MoveVM;
use move_vm_test_utils::gas_schedule::CostTable;
use std::collections::BTreeMap;

use move_core_types::{
    account_address::AccountAddress,
};

pub fn publish(
    module_bytes_params: Vec<u8>,
    sender_param: &str,
    natives: impl IntoIterator<Item = NativeFunctionRecord>,
    cost_table: &CostTable,
    state: &mut OnDiskStateView,
    verbose: bool,
) -> Result<()> {
    println!("publish---debug------1");
    let bytecode_version = get_bytecode_version_from_env();
    let vm = MoveVM::new(natives).unwrap();
    let mut gas_status = get_gas_status(cost_table, None)?;
    let mut session = vm.new_session(state);
    let mut has_error = false;
    println!("publish---debug------2--gas_status:{:?}", gas_status.remaining_gas());
    let sender = AccountAddress::from_hex_literal(sender_param).unwrap();

    let res = session.publish_module(module_bytes_params.clone(), sender, &mut gas_status);
    if let Err(err) = res {
        //explain_publish_error(err, state, unit)?;
        has_error = true;
    }

    if !has_error {
        println!("publish---debug------3--gas_status:{:?}", gas_status.remaining_gas());
        let (changeset, events) = session.finish().map_err(|e| e.into_vm_status())?;
        assert!(events.is_empty());
        if verbose {
            //explain_publish_changeset(&changeset);
        }
        let modules: Vec<_> = changeset
            .into_modules()
            .map(|(module_id, blob_opt)| {
                (module_id, blob_opt.ok().expect("must be non-deletion"))
            })
            .collect();
        state.save_modules(&modules)?;
    }
    Ok(())
}

