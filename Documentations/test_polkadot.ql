contract PolkadotToken {
    self.balance: uint256;  // Should fail - ink! doesn't support uint256
    
    @constructor
    fn __init__() {
        self.balance = 0;
    }
    
    @external
    fn get_balance() -> uint256 {
        return self.balance;
    }
}
