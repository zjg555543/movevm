module std::Account {
    native fun create_signer(addr: address): signer;

    native fun get_amount(addr: address): u8;

    native fun transfer_amount(from: address, to: address, amount: u128): bool;

    public fun create() {
        get_amount(@0x1);
        transfer_amount(@0x1, @0x2, 1000000);
        create_signer(@0x3);
    }
}