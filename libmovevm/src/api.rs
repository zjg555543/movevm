use std::path::PathBuf;
use move_package::BuildConfig;
use std::{fs, path::Path};
use crate::adapt::memory::{ByteSliceView, UnmanagedVector};

use anyhow::Result;
use move_core_types::{
    account_address::AccountAddress,
    errmap::ErrorMapping,
    language_storage::TypeTag,
    transaction_argument::{convert_txn_args, TransactionArgument},
};
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
    println!("--------------say publish-------------- ");
    test_publish();
}

pub fn test_publish()-> Result<()> {
    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme/storage/");
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
    println!("--------------say run-------------- ");
    test_run();
}

pub fn test_run()-> Result<()> {
    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme/storage/");
    let build_config = BuildConfig::default();
    // let script_name Option<String> = None;
    let script_file = Path::new("/Users/oker/workspace/move/movevm/contracts/readme/sources/test_script.move");
    let context = PackageContext::new(&path, &build_config)?;
    let state = context.prepare_state(&storage_dir)?;
    // let error_descriptions: ErrorMapping = bcs::from_bytes(move_stdlib::error_descriptions())?;
    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();
    let natives : Vec<NativeFunctionRecord> = all_natives(addr, GasParameters::zeros())
        .into_iter()
        .chain(nursery_natives(addr, NurseryGasParameters::zeros()))
        .collect();
    let mut my_vec = Vec::new();
    let name1 = String::from("0xf");
    my_vec.push(name1);

    let args :Vec<TransactionArgument> = Vec::new();
    let vm_type_args : Vec<TypeTag>  = Vec::new();
    run(
        natives,
        cost_table,
        &ErrorMapping::default(),
        &state,
        context.package(),
        &script_file,
        &None,
        &my_vec,
        &args,
        vm_type_args,
        None,
        false,
        true,
    );
    return Ok(());
}


#[no_mangle]
pub extern "C" fn say_input_output(code: ByteSliceView) -> UnmanagedVector{
    println!("--------------say input output-------------- ");

    test_input_output(code);

    // let r = match to_cache(cache) {
    //     Some(c) => catch_unwind(AssertUnwindSafe(move || do_save_wasm(c, wasm)))
    //         .unwrap_or_else(|_| Err(Error::panic())),
    //     None => Err(Error::unset_arg(CACHE_ARG)),
    // };
    // let checksum = handle_c_error_binary(r, error_msg);

    // let mut result Vec<u8>;
    // result.push(0);
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    UnmanagedVector::new(Some(vec))
}

pub fn test_input_output(code: ByteSliceView)-> Result<()> {
    let arg1 = code.read();
    println!("code:{:?}", arg1);

    return Ok(());
}

// pub extern "C" fn save_wasm(
//     cache: *mut cache_t,
//     wasm: ByteSliceView,
//     error_msg: Option<&mut UnmanagedVector>,
// ) -> UnmanagedVector {
//     let r = match to_cache(cache) {
//         Some(c) => catch_unwind(AssertUnwindSafe(move || do_save_wasm(c, wasm)))
//             .unwrap_or_else(|_| Err(Error::panic())),
//         None => Err(Error::unset_arg(CACHE_ARG)),
//     };
//     let checksum = handle_c_error_binary(r, error_msg);
//     UnmanagedVector::new(Some(checksum))
// }
