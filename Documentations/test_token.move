module TokenModule {
    struct Balance has key {
        value: u64
    }

    public entry fun initialize(account: &signer) {
        move_to(account, Balance { value: 1000000 });
    }

    public entry fun transfer(from: &signer, to: address, amount: u64) {
        let from_balance = borrow_global_mut<Balance>(signer::address_of(from));
        from_balance.value = from_balance.value - amount;
        
        let to_balance = borrow_global_mut<Balance>(to);
        to_balance.value = to_balance.value + amount;
    }

    public fun get_balance(addr: address): u64 {
        borrow_global<Balance>(addr).value
    }
}
