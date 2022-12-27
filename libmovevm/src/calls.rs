

/// Execute func
///
/// params is gas u64
#[no_mangle]
pub extern "C" fn execute(gas_limit: u64) {
    println!("hello, {}", gas_limit);
}
