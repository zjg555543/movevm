use move_binary_format::errors::PartialVMResult;
use move_core_types::{account_address::AccountAddress, gas_algebra::InternalGas};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::collections::VecDeque;
use std::sync::Arc;
use crate::adapt::vm::types::{GasInfo, Storage, Querier};
use crate::adapt::memory::{U8SliceView, ByteSliceView, UnmanagedVector};
use std::sync::Mutex;

use crate::stdvm::query::{BankQuery, AllBalanceResponse, QueryRequest, BalanceResponse};
use crate::stdvm::{binary::Binary, result::contract_result::ContractResult, errors::system_error::SystemError, result::system_result::SystemResult};
use crate::stdvm::result::empty::Empty;

use crate::stdvm::serde::to_vec;
use crate::stdvm::serde::from_binary;

#[derive(Debug, Clone)]
pub struct CreateSignerGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GetAmountGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct TransferAmountGasParameters {
    pub base: InternalGas,
}

fn native_create_signer(
    gas_params: &CreateSignerGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let address = pop_arg!(arguments, AccountAddress);

    println!("native_create_signer--------------address:{:?}", address);
    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::signer(address)],
    ))
}

fn native_get_amount(
    querier: Arc<Mutex<Box<dyn Querier + Send +'static>>>,
    gas_params: &GetAmountGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let address = pop_arg!(arguments, AccountAddress);


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

    println!("native_get_amount----------start ");
    // let querierInface: Box<dyn Querier> = Box::new(querier);
    let newResult = querier.lock().unwrap().query_raw(request, gas_limit);
    let bin_data :Binary = newResult.0.unwrap().unwrap().unwrap();
    let AllBalanceResponse { amount } = from_binary(&bin_data).unwrap();
    println!("native_get_amount----------amount {:?}", amount);


    println!("native_get_amount--------------address:{:?}----test data", address);
    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::u8(0)],
    ))
}

fn native_transfer_amount(
    gas_params: &TransferAmountGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 2);

    let address1 = pop_arg!(arguments, AccountAddress);
    let address2 = pop_arg!(arguments, AccountAddress);

    println!("native_transfer_amount--------------address1:{:?},address2:{:?}", address1, address2);
    Ok(NativeResult::ok(
        gas_params.base,
        smallvec![Value::bool(true)],
    ))
}

pub fn make_native_create_signer(gas_params: CreateSignerGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_create_signer(&gas_params, context, ty_args, args)
    })
}

pub fn make_native_get_amount(querier: Arc<Mutex<Box<dyn Querier + Send +'static>>>, gas_params: GetAmountGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_get_amount(querier.clone(),&gas_params, context, ty_args, args)
    })
}

pub fn make_native_transfer_amount(gas_params: TransferAmountGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_transfer_amount(&gas_params, context, ty_args, args)
    })
}

#[derive(Debug, Clone)]
pub struct GasParameters {
    pub create_signer: CreateSignerGasParameters,
    pub get_amount: GetAmountGasParameters,
    pub transfer_amount: TransferAmountGasParameters,
}

pub fn make_all(querier: Arc<Mutex<Box<dyn Querier + Send +'static>>>, gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "create_signer",
            make_native_create_signer(gas_params.create_signer),
        ),
        (
            "get_amount",
            make_native_get_amount(querier, gas_params.get_amount),
        ),
        (
            "transfer_amount",
            make_native_transfer_amount(gas_params.transfer_amount),
        ),
    ];

    crate::natives::helpers::make_module_natives(natives)
}
