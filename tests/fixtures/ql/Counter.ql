contract Counter:
    count: uint256

    @external
    fn increment():
        self.count += 1
