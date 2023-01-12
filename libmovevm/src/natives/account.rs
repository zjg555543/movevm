use move_binary_format::errors::PartialVMResult;
use move_core_types::{account_address::AccountAddress, gas_algebra::InternalGas};
use move_vm_runtime::native_functions::{NativeContext, NativeFunction};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, pop_arg, values::Value,
};
use smallvec::smallvec;
use std::collections::VecDeque;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateSignerGasParameters {
    pub base: InternalGas,
}

#[derive(Debug, Clone)]
pub struct GetAmountGasParameters {
    pub base: InternalGas,
    pub api: u64,
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
    gas_params: &GetAmountGasParameters,
    _context: &mut NativeContext,
    ty_args: Vec<Type>,
    mut arguments: VecDeque<Value>,
) -> PartialVMResult<NativeResult> {
    debug_assert!(ty_args.is_empty());
    debug_assert!(arguments.len() == 1);

    let address = pop_arg!(arguments, AccountAddress);

    println!("native_get_amount--------------address:{:?}", address);
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

pub fn make_native_get_amount(gas_params: GetAmountGasParameters) -> NativeFunction {
    Arc::new(move |context, ty_args, args| {
        native_get_amount(&gas_params, context, ty_args, args)
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

pub fn make_all(gas_params: GasParameters) -> impl Iterator<Item = (String, NativeFunction)> {
    let natives = [
        (
            "create_signer",
            make_native_create_signer(gas_params.create_signer),
        ),
        (
            "get_amount",
            make_native_get_amount(gas_params.get_amount),
        ),
        (
            "transfer_amount",
            make_native_transfer_amount(gas_params.transfer_amount),
        ),
    ];

    crate::natives::helpers::make_module_natives(natives)
}
