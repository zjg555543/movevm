use std::path::PathBuf;
use move_package::BuildConfig;

use anyhow::Result;
use move_core_types::{account_address::AccountAddress, errmap::ErrorMapping};
use move_stdlib::natives::{all_natives, nursery_natives, GasParameters, NurseryGasParameters};

use crate::{
    wrap::{
        commands::publish,
        commands::run,
        utils::{on_disk_state_view::OnDiskStateView, PackageContext},
    },
    Move, NativeFunctionRecord, DEFAULT_BUILD_DIR,
};



#[no_mangle]
pub extern "C" fn say_publish(gas_limit: u64) {
    println!("hello, {}", gas_limit);
    test_publish();
}

pub fn test_publish()-> Result<()> {
    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/move-demo/readme/"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/move-demo/readme/storage/");
    let build_config = BuildConfig::default();

    // let context :PackageContext;
    let context = PackageContext::new(&path, &build_config)?;
    // let f = match result_context {
    //     Ok(result) => {
    //         context = result
    //     },
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    let state = context.prepare_state(&storage_dir)?;


    // let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions())?;
    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();

    let natives : Vec<NativeFunctionRecord> = all_natives(addr, GasParameters::zeros())
        .into_iter()
        .chain(nursery_natives(addr, NurseryGasParameters::zeros()))
        .collect();

    let opt_test = None;

    let no_republish = false;
    let ignore_breaking_changes = false;
    let with_deps = false;
    let bundle = false;
    publish(
        natives,
        cost_table,
        &state,
        context.package(),
        no_republish,
        ignore_breaking_changes,
        with_deps,
        bundle,
        opt_test,
        true,
    );

    return Ok(());
}


#[no_mangle]
pub extern "C" fn say_run(gas_limit: u64) {
    println!("run, {}", gas_limit);
    //test_publish();
}

pub fn test_run()-> Result<()> {
    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/move-demo/readme/"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/move-demo/readme/storage/");
    let build_config = BuildConfig::default();

    // let context :PackageContext;
    let context = PackageContext::new(&path, &build_config)?;
    // let f = match result_context {
    //     Ok(result) => {
    //         context = result
    //     },
    //     Err(error) => {
    //         panic!("Problem opening the file: {:?}", error)
    //     },
    // };

    let state = context.prepare_state(&storage_dir)?;


    // let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions())?;
    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();

    let natives : Vec<NativeFunctionRecord> = all_natives(addr, GasParameters::zeros())
        .into_iter()
        .chain(nursery_natives(addr, NurseryGasParameters::zeros()))
        .collect();

    let opt_test = None;

    let no_republish = false;
    let ignore_breaking_changes = false;
    let with_deps = false;
    let bundle = false;
    run(
        natives,
        cost_table,
        &state,
        context.package(),
        no_republish,
        ignore_breaking_changes,
        with_deps,
        bundle,
        opt_test,
        true,
    );

    return Ok(());
}