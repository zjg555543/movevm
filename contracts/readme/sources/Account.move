module std::Account {
    native fun create_signer(addr: address): signer;

    public fun create() {
        let account_address : address = @0x3;
        create_signer(account_address);
    }
}