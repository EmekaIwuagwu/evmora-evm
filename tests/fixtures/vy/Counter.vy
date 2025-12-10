# @version ^0.3.0

count: public(uint256)

@external
def increment():
    self.count += 1
