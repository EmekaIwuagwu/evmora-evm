contract Counter {
    uint256 count;

    fn increment() {
        self.count += 1;
    }

    fn decrement() {
        if self.count > 0 {
            self.count -= 1;
        }
    }

    fn getCount() {
        return self.count;
    }

    fn reset() {
        self.count = 0;
    }
}
