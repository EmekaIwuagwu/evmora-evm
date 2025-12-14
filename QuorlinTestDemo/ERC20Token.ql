from quorlin.std import event, require

event Transfer(from_addr: address, to: address, value: uint256)
event Approval(owner: address, spender: address, value: uint256)

contract ERC20Token:
    name: str
    symbol: str
    decimals: uint8
    total_supply: uint256
    balances: mapping[address, uint256]
    allowances: mapping[address, mapping[address, uint256]]

    fn __init__(self, name: str, symbol: str, initial_supply: uint256):
        self.name = name
        self.symbol = symbol
        self.decimals = 18
        self.total_supply = initial_supply
        self.balances[msg.sender] = initial_supply
        emit Transfer(address(0), msg.sender, initial_supply)

    fn balance_of(self, account: address) -> uint256:
        return self.balances[account]

    fn transfer(self, to: address, amount: uint256) -> bool:
        require(to != address(0), "Transfer to zero address")
        require(self.balances[msg.sender] >= amount, "Insufficient balance")

        self.balances[msg.sender] -= amount
        self.balances[to] += amount
        emit Transfer(msg.sender, to, amount)
        return True

    fn approve(self, spender: address, amount: uint256) -> bool:
        require(spender != address(0), "Approve to zero address")
        self.allowances[msg.sender][spender] = amount
        emit Approval(msg.sender, spender, amount)
        return True

    fn transfer_from(self, from_addr: address, to: address, amount: uint256) -> bool:
        require(to != address(0), "Transfer to zero address")
        require(self.balances[from_addr] >= amount, "Insufficient balance")
        require(self.allowances[from_addr][msg.sender] >= amount, "Insufficient allowance")

        self.balances[from_addr] -= amount
        self.balances[to] += amount
        self.allowances[from_addr][msg.sender] -= amount
        emit Transfer(from_addr, to, amount)
        return True
