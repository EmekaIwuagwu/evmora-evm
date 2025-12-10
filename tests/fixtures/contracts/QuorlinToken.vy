# @version ^0.3.7

name: public(String[32])
symbol: public(String[8])
decimals: public(uint256)
total_supply: public(uint256)

balances: HashMap[address, uint256]
allowances: HashMap[address, HashMap[address, uint256]]

event Transfer:
    sender: indexed(address)
    receiver: indexed(address)
    value: uint256

event Approval:
    owner: indexed(address)
    spender: indexed(address)
    value: uint256

@external
def __init__():
    self.name = "Quorlin Token"
    self.symbol = "QRLN"
    self.decimals = 18
    self.total_supply = 0

@external
def mint(to: address, amount: uint256):
    self.total_supply += amount
    self.balances[to] += amount
    log Transfer(ZERO_ADDRESS, to, amount)

@external
def transfer(to: address, amount: uint256) -> bool:
    assert self.balances[msg.sender] >= amount
    self.balances[msg.sender] -= amount
    self.balances[to] += amount
    log Transfer(msg.sender, to, amount)
    return True

@external
def approve(spender: address, amount: uint256) -> bool:
    self.allowances[msg.sender][spender] = amount
    log Approval(msg.sender, spender, amount)
    return True

@external
def transferFrom(sender: address, recipient: address, amount: uint256) -> bool:
    assert self.allowances[sender][msg.sender] >= amount
    assert self.balances[sender] >= amount

    self.allowances[sender][msg.sender] -= amount
    self.balances[sender] -= amount
    self.balances[recipient] += amount

    log Transfer(sender, recipient, amount)
    return True
