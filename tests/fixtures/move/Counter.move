module Counter {
    struct CounterResource has key {
        value: u64,
    }

    public fun increment(account: &signer) {
        // Increment counter value + 1
        let count = 1;
    }
}
