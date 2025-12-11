contract VulnerableBank {
    self.balances: mapping[address => uint256];

    @external
    fn deposit() {
        self.balances[msg.sender] = self.balances[msg.sender] + msg.value;
    }

    @external
    fn withdraw(amount: uint256) {
        require(self.balances[msg.sender] >= amount);
        
        // DANGEROUS: external call before state update
        msg.sender.call();
        
        // State update after external call - REENTRANCY RISK!
        self.balances[msg.sender] = self.balances[msg.sender] - amount;
    }
}
