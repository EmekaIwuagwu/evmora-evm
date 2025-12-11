contract SimpleToken {
    self.total_supply: uint256 = 1000000;
    self.balances: mapping[address => uint256];
    self.owner: address;

    @constructor
    fn __init__() {
        self.owner = msg.sender;
        self.balances[msg.sender] = self.total_supply;
    }

    @external
    fn transfer(to: address, amount: uint256) {
        require(self.balances[msg.sender] >= amount);
        self.balances[msg.sender] = self.balances[msg.sender] - amount;
        self.balances[to] = self.balances[to] + amount;
    }

    @view
    fn balance_of(account: address) -> uint256 {
        return self.balances[account];
    }
}
