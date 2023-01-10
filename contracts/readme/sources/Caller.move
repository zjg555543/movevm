// sources/Caller.move
module 0x3::Caller {
    // use std::signer;
    use 0x2::Test;
    use std::Account;

    public fun publish(account: &signer) {
        Account::create();
        Test::publish(account)
    }
}
