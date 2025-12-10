# @version ^0.3.0

# Vyper Counter Contract
count: public(uint256)

@external
def __init__():
    self.count = 0

@external
def increment():
    self.count += 1
