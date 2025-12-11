# Vyper Token Contract
totalSupply: public(uint256)
balances: HashMap[address, uint256]
owner: public(address)

@constructor
@external
def __init__():
    self.owner = msg.sender
    self.totalSupply = 1000000
    self.balances[msg.sender] = self.totalSupply

@external
def transfer(to: address, amount: uint256):
    assert self.balances[msg.sender] >= amount
    self.balances[msg.sender] = self.balances[msg.sender] - amount
    self.balances[to] = self.balances[to] + amount

@view
@external
def balanceOf(account: address) -> uint256:
    return self.balances[account]
