// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    wrap::utils::{
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

pub fn publish(
    natives: impl IntoIterator<Item = NativeFunctionRecord>,
    cost_table: &CostTable,
    state: &mut OnDiskStateView,
    package: &CompiledPackage,
    verbose: bool,
) -> Result<()> {
    // collect all modules compiled
    let compiled_modules = package.root_modules().collect::<Vec<_>>();
    if verbose {
        println!("Found {} modules", compiled_modules.len());
    }
    println!("publish---debug------1");
    // order the modules for publishing
    let modules_to_publish = compiled_modules;
    let bytecode_version = get_bytecode_version_from_env();

    println!("bytecode_version:{:?}", bytecode_version);

    // use the the publish_module API from the VM if we do not allow breaking changes
    println!("publish---debug------8");
    let vm = MoveVM::new(natives).unwrap();
    let mut gas_status = get_gas_status(cost_table, None)?;
    let mut session = vm.new_session(state);
    let mut has_error = false;
    println!("publish---debug------8--gas_status:{:?}", gas_status.remaining_gas());
    println!("publish---debug------10");
    // publish modules sequentially, one module at a time
    for unit in &modules_to_publish {
        let module_bytes = unit.unit.serialize(bytecode_version);
        let id = module(&unit.unit)?.self_id();
        let sender = *id.address();
        println!("publish---debug------10---0");
        let res = session.publish_module(module_bytes, sender, &mut gas_status);
        println!("publish---debug------10---1");
        if let Err(err) = res {
            //explain_publish_error(err, state, unit)?;
            has_error = true;
            break;
        }
    }

    if !has_error {
        println!("publish---debug------11--gas_status:{:?}", gas_status.remaining_gas());
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

