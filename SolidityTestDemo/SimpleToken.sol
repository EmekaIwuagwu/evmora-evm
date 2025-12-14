// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

/**
 * @title SimpleToken
 * @dev A minimal ERC20-like token for EVMora testing
 */
contract SimpleToken {
    string public name;
    string public symbol;
    uint256 public totalSupply;
    
    mapping(address => uint256) private balances;

    event Transfer(address indexed from, address indexed to, uint256 value);
    event Mint(address indexed to, uint256 value);

    address public owner;

    modifier onlyOwner() {
        require(msg.sender == owner, "SimpleToken: only owner");
        _;
    }

    /**
     * @dev Constructor sets the token metadata and owner
     */
    constructor(string memory _name, string memory _symbol) {
        name = _name;
        symbol = _symbol;
        owner = msg.sender;
    }

    /**
     * @dev Mints new tokens to a specified address (Owner only)
     * @param to The address to receive the tokens
     * @param amount The amount of tokens to mint
     */
    function mint(address to, uint256 amount) public onlyOwner {
        totalSupply += amount;
        balances[to] += amount;
        emit Mint(to, amount);
        emit Transfer(address(0), to, amount);
    }

    /**
     * @dev Transfers tokens from caller to another address
     * @param to The recipient address
     * @param amount The amount to transfer
     */
    function transfer(address to, uint256 amount) public returns (bool) {
        require(balances[msg.sender] >= amount, "SimpleToken: insufficient balance");
        require(to != address(0), "SimpleToken: transfer to zero address");

        balances[msg.sender] -= amount;
        balances[to] += amount;
        emit Transfer(msg.sender, to, amount);
        return true;
    }

    /**
     * @dev Returns the balance of a specific address
     * @param account The address to check
     */
    function balanceOf(address account) public view returns (uint256) {
        return balances[account];
    }
}
