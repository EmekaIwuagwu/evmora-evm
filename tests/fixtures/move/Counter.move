module 0x1::Counter {
    struct Counter has key {
        value: u64,
    }

    public fun increment(account: &signer) {
        // Logic placeholder as evmora-compiler move frontend mock likely handles this
    }
}
