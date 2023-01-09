// sources/Caller.move
module 0x3::Caller {
    // use std::signer;
    use 0x2::Test;

    public fun publish(account: &signer) {
        Test::publish(account)
    }
}
