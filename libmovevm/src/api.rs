use std::path::PathBuf;
use move_package::BuildConfig;
use std::{fs, path::Path};
use crate::adapt::memory::{U8SliceView, ByteSliceView, UnmanagedVector};
use crate::adapt::db::Db;
use crate::adapt::goapi::GoApi;
use crate::adapt::error::{handle_c_error_binary, handle_c_error_default, handle_c_error_ptr, Error};
use crate::adapt::args::{AVAILABLE_CAPABILITIES_ARG, CACHE_ARG, CHECKSUM_ARG, DATA_DIR_ARG, WASM_ARG};
use crate::adapt::querier::GoQuerier;
use crate::adapt::error::GoError;
use crate::adapt::vm::types::{GasInfo, Storage};
use crate::adapt::storage::GoStorage;

use cosmwasm_std::{
    coins, from_binary, to_vec, AllBalanceResponse, BankQuery, Empty, QueryRequest,Binary,
    ContractResult, SystemError, SystemResult,coin, BalanceResponse,
};

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
pub extern "C" fn say_build(db: Db) {
    println!("--------------say build start-------------- ");
    test_build(db);
    println!("--------------say build end-------------- ");
}

pub fn test_build(db: Db)-> Result<()> {
    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme/storage/");
    let build_config = BuildConfig::default();
    let context = PackageContext::new(&path, &build_config)?;
    let mut state = context.prepare_state(&storage_dir, &storage_dir, db)?;

    Ok(())
}

#[no_mangle]
pub extern "C" fn say_publish(module_code: ByteSliceView, sender: ByteSliceView, db: Db) {
    println!("--------------say publish start-------------- ");

    test_publish(module_code, sender, db);
    println!("--------------say publish end-------------- ");
}

pub fn test_publish(module_code: ByteSliceView, sender: ByteSliceView, db: Db)-> Result<()> {
    let module_bytes_params = module_code.to_owned().unwrap();
    println!("module_bytes_params len --- 1:{:?}", module_bytes_params.len());
    let temp_sender = sender.to_owned().unwrap();
    let sender_params = std::str::from_utf8(&temp_sender).unwrap();

    println!("sender_params len --- 1:{:?}", sender_params);

    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme/storage/");
    let build_config = BuildConfig::default();

    println!("--------------test_publish-------------- 0 ");

    let mut state= OnDiskStateView::create(build_config.install_dir
                                               .as_ref()
                                               .unwrap_or(&PathBuf::from(DEFAULT_BUILD_DIR))
                                               .clone(), storage_dir, Box::new(GoStorage::new(db)))?;

    println!("--------------test_publish-------------- 1 ");

    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();

    println!("--------------test_publish-------------- 2 ");
    let natives : Vec<NativeFunctionRecord> = all_natives(addr, GasParameters::zeros())
        .into_iter()
        .chain(nursery_natives(addr, NurseryGasParameters::zeros()))
        .collect();

    println!("--------------test_publish-------------- 3 ");

    publish(
        module_bytes_params,
        sender_params,
        natives,
        cost_table,
        &mut state,
        true,
    );

    return Ok(());
}


#[no_mangle]
pub extern "C" fn say_run(scrpit_code: ByteSliceView, sender: ByteSliceView,  db: Db) {
    println!("--------------say run start-------------- ");
    test_run(scrpit_code,sender, db);
    println!("--------------say run end-------------- ");
}

pub fn test_run(scrpit_code: ByteSliceView, sender: ByteSliceView, db: Db)-> Result<()> {
    let scrpit_bytes_params = scrpit_code.to_owned().unwrap();
    println!("scrpit_bytes_params len --- 1:{:?}", scrpit_bytes_params.len());
    let temp_sender = sender.to_owned().unwrap();
    let sender_params = std::str::from_utf8(&temp_sender).unwrap();


    let path = Some(PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme"));
    let storage_dir = PathBuf::from(r"/Users/oker/workspace/move/movevm/contracts/readme/storage/");
    let build_config = BuildConfig::default();
    // let script_file = Path::new("/Users/oker/workspace/move/movevm/contracts/readme/sources/test_script.move");
    // let context = PackageContext::new(&path, &build_config)?;
    // let mut state = context.prepare_state(&storage_dir, &storage_dir, db)?;
    let mut state= OnDiskStateView::create(build_config.install_dir
                                               .as_ref()
                                               .unwrap_or(&PathBuf::from(DEFAULT_BUILD_DIR))
                                               .clone(), storage_dir, Box::new(GoStorage::new(db)))?;
    let cost_table = &move_vm_test_utils::gas_schedule::INITIAL_COST_SCHEDULE;
    let addr = AccountAddress::from_hex_literal("0x1").unwrap();
    let natives : Vec<NativeFunctionRecord> = all_natives(addr, GasParameters::zeros())
        .into_iter()
        .chain(nursery_natives(addr, NurseryGasParameters::zeros()))
        .collect();
    let mut signers = Vec::new();
    let name1 = String::from(sender_params);
    signers.push(name1);

    let args :Vec<TransactionArgument> = Vec::new();
    let vm_type_args : Vec<TypeTag>  = Vec::new();
    run(
        scrpit_bytes_params,
        natives,
        cost_table,
        &ErrorMapping::default(),
        &mut state,
        &signers,
        &args,
        vm_type_args,
        None,
        true,
    );
    return Ok(());
}


#[no_mangle]
pub extern "C" fn say_input_output(code: ByteSliceView,
                                   db: Db,
                                   api: GoApi,
                                   querier: GoQuerier,
                                   error_msg: Option<&mut UnmanagedVector>) -> UnmanagedVector{
    println!("--------------say input output start-------------- ");

    let arg1 = code.read();
    println!("code:{:?}", arg1);

    // for query test
    let mut output = UnmanagedVector::default();
    let mut error_msg = UnmanagedVector::default();
    let mut used_gas = 0_u64;
    const INIT_ADDR: &str = "contract";
    let req: QueryRequest<Empty> = QueryRequest::Bank(BankQuery::AllBalances {
        address: INIT_ADDR.to_string(),
    });

    let request = &to_vec(&req).unwrap();

    const DEFAULT_QUERY_GAS_LIMIT: u64 = 300_000;
    let gas_limit = DEFAULT_QUERY_GAS_LIMIT;

    let go_result: GoError = (querier.vtable.query_external)(
        querier.state,
        gas_limit,
        &mut used_gas as *mut u64,
        U8SliceView::new(Some(request)),
        &mut output as *mut UnmanagedVector,
        &mut error_msg as *mut UnmanagedVector,
    ).into();

    let gas_info = GasInfo::with_externally_used(used_gas);
    println!("gas use{:?}", used_gas);
    let output = output.consume();

    let bin_result: Vec<u8> = output.unwrap_or_default();
    let result :Result<SystemResult<ContractResult<Binary>>> = serde_json::from_slice(&bin_result).or_else(|e| {
        Ok(SystemResult::Err(SystemError::InvalidResponse {
            error: format!("Parsing Go response: {}", e),
            response: bin_result.into(),
        }))
    });
    let bin_data :Binary = result.unwrap().unwrap().unwrap();
    let AllBalanceResponse { amount } = from_binary(&bin_data).unwrap();
    println!("result{:?}", amount);

    test_db(db);

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("--------------say input output end-------------- ");
    UnmanagedVector::new(Some(vec))
}

pub fn test_db(db: Db){
    let mut store = GoStorage::new(db);
    let mut key = Vec::new();
    key.push(1);
    key.push(1);
    key.push(1);
    key.push(1);

    let mut value = Vec::new();
    value.push(2);
    value.push(2);
    value.push(2);
    value.push(2);
    store.set(&key, &value);


    let hello_db = store.get(&key);
    println!("get db hello_db:{:?}", hello_db)
}
