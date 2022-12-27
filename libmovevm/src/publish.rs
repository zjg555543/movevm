#[no_mangle]
pub extern "C" fn publish(gas_limit: u64) {
    println!("hello, {}", gas_limit);
}