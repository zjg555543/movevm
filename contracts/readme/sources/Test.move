// sources/Test.move
module 0x2::Test {
    use std::signer;

    struct Resource2 has key { i: u64 }

    public fun publish(account: &signer) {
        move_to(account, Resource2 { i: 10 })
    }

    public fun write(account: &signer, i: u64) acquires Resource2 {
        borrow_global_mut<Resource2>(signer::address_of(account)).i = i;
    }

    public fun unpublish(account: &signer) acquires Resource2 {
        let Resource2 { i: _ } = move_from(signer::address_of(account));
  }
}
