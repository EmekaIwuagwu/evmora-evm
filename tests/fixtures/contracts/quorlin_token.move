module Quorlin::QuorlinToken {

    use std::signer;
    use std::vector;

    struct Token has key {
        supply: u64,
        decimals: u8,
        name: vector<u8>,
        symbol: vector<u8>,
    }

    struct Balance has store {
        amount: u64,
    }

    public fun initialize(admin: &signer, decimals: u8, name: vector<u8>, symbol: vector<u8>) {
        move_to(admin, Token {
            supply: 0,
            decimals,
            name,
            symbol
        });
    }

    public fun mint(admin: &signer, recipient: address, amount: u64) {
        let token = borrow_global_mut<Token>(signer::address_of(admin));
        token.supply = token.supply + amount;

        if (!exists<Balance>(recipient)) {
            move_to(&signer::new(recipient), Balance { amount });
        } else {
            let bal = borrow_global_mut<Balance>(recipient);
            bal.amount = bal.amount + amount;
        }
    }

    public fun transfer(sender: &signer, recipient: address, amount: u64) {
        let sender_bal = borrow_global_mut<Balance>(signer::address_of(sender));
        assert!(sender_bal.amount >= amount, 1);

        sender_bal.amount = sender_bal.amount - amount;

        if (!exists<Balance>(recipient)) {
            move_to(&signer::new(recipient), Balance { amount });
        } else {
            let rb = borrow_global_mut<Balance>(recipient);
            rb.amount = rb.amount + amount;
        }
    }
}
