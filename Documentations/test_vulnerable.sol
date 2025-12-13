// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

contract VulnerableBank {
    mapping(address => uint256) public balances;

    function deposit() public payable {
        balances[msg.sender] = balances[msg.sender] + msg.value;
    }

    function withdraw(uint256 amount) public {
        require(balances[msg.sender] >= amount);
        
        // DANGEROUS: External call before state update
        msg.sender.call{value: amount}("");
        
        // State update after external call - REENTRANCY!
        balances[msg.sender] = balances[msg.sender] - amount;
    }
}
