// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title Counter
 * @dev A simple counter contract to demonstrate state changes on EVMora
 */
contract Counter {
    uint256 private count;

    event CountChanged(uint256 newCount);

    /**
     * @dev Constructor initializes count to 0
     */
    constructor() {
        count = 0;
    }

    /**
     * @dev Increments the count by 1
     * Gas optimization: unchecked for overflow is not needed for uint256 in realistic lifetimes, 
     * but default solidity 0.8+ checks.
     */
    function increment() public {
        count += 1;
        emit CountChanged(count);
    }

    /**
     * @dev Decrements the count by 1
     * Reverts on underflow automatically in Solidity 0.8+
     */
    function decrement() public {
        require(count > 0, "Counter: count is zero");
        count -= 1;
        emit CountChanged(count);
    }

    /**
     * @dev Resets the count to 0
     */
    function reset() public {
        count = 0;
        emit CountChanged(count);
    }

    /**
     * @dev Returns the current count
     */
    function getCount() public view returns (uint256) {
        return count;
    }
}
