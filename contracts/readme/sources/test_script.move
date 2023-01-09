// sources/test_script.move
script {
    use 0x3::Caller;
    fun test_script(account: signer) {
        Caller::publish(&account)
    }
}
