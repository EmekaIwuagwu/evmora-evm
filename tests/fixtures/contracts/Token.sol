// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/// @title Quorlin Token (ERC-20)
/// @author Solidity Developer (assistant)
/// @notice Simple, audited-style ERC-20 implementation with minting by owner
/// @dev No external dependencies; uses Solidity 0.8.x built-in overflow checks

contract QuorlinToken {
    // Token metadata
    string public name = "Quorlin Token";
    string public symbol = "QRLN";
    uint8 public decimals = 18;

    // Total token supply
    uint256 private _totalSupply;

    // Balances and allowances
    mapping(address => uint256) private _balances;
    mapping(address => mapping(address => uint256)) private _allowances;

    // Owner for privileged actions (mint)
    address public owner;

    // Events
    event Transfer(address indexed from, address indexed to, uint256 value);
    event Approval(address indexed owner, address indexed spender, uint256 value);
    event OwnershipTransferred(address indexed previousOwner, address indexed newOwner);

    /// @notice Sets deployer as owner and optionally mints initial supply to deployer
    /// @param initialSupply Initial token amount, in wei (include decimals). Example: 1e24 for 1,000 QRLN if decimals=18
    constructor(uint256 initialSupply) {
        owner = msg.sender;
        if (initialSupply > 0) {
            _mint(msg.sender, initialSupply);
        }
    }

    // --- Ownership ---
    modifier onlyOwner() {
        require(msg.sender == owner, "QuorlinToken: caller is not the owner");
        _;
    }

    /// @notice Transfer ownership to a new account (`newOwner`). Can only be called by the current owner.
    function transferOwnership(address newOwner) external onlyOwner {
        require(newOwner != address(0), "QuorlinToken: new owner is the zero address");
        emit OwnershipTransferred(owner, newOwner);
        owner = newOwner;
    }

    // --- ERC-20 Interface ---

    /// @notice Returns the total token supply
    function totalSupply() external view returns (uint256) {
        return _totalSupply;
    }

    /// @notice Returns the token balance of `account`
    function balanceOf(address account) external view returns (uint256) {
        return _balances[account];
    }

    /// @notice Transfer `amount` tokens to `recipient`
    function transfer(address recipient, uint256 amount) external returns (bool) {
        _transfer(msg.sender, recipient, amount);
        return true;
    }

    /// @notice Returns current allowance from `owner_` to `spender`
    function allowance(address owner_, address spender) external view returns (uint256) {
        return _allowances[owner_][spender];
    }

    /// @notice Approve `spender` to spend `amount` on behalf of caller
    function approve(address spender, uint256 amount) external returns (bool) {
        _approve(msg.sender, spender, amount);
        return true;
    }

    /// @notice Transfer `amount` tokens from `sender` to `recipient` using allowance mechanism
    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool) {
        uint256 currentAllowance = _allowances[sender][msg.sender];
        require(currentAllowance >= amount, "QuorlinToken: transfer amount exceeds allowance");
        _approve(sender, msg.sender, currentAllowance - amount);
        _transfer(sender, recipient, amount);
        return true;
    }

    /// @notice Increase allowance for `spender` by `addedValue`
    function increaseAllowance(address spender, uint256 addedValue) external returns (bool) {
        _approve(msg.sender, spender, _allowances[msg.sender][spender] + addedValue);
        return true;
    }

    /// @notice Decrease allowance for `spender` by `subtractedValue`
    function decreaseAllowance(address spender, uint256 subtractedValue) external returns (bool) {
        uint256 currentAllowance = _allowances[msg.sender][spender];
        require(currentAllowance >= subtractedValue, "QuorlinToken: decreased allowance below zero");
        _approve(msg.sender, spender, currentAllowance - subtractedValue);
        return true;
    }

    // --- Mint & Burn (owner-only mint) ---

    /// @notice Mint `amount` tokens to `account`. Only owner.
    function mint(address account, uint256 amount) external onlyOwner returns (bool) {
        _mint(account, amount);
        return true;
    }

    /// @notice Burn `amount` of caller's tokens
    function burn(uint256 amount) external returns (bool) {
        _burn(msg.sender, amount);
        return true;
    }

    // --- Internal helpers ---

    function _transfer(address sender, address recipient, uint256 amount) internal {
        require(sender != address(0), "QuorlinToken: transfer from the zero address");
        require(recipient != address(0), "QuorlinToken: transfer to the zero address");
        uint256 senderBalance = _balances[sender];
        require(senderBalance >= amount, "QuorlinToken: transfer amount exceeds balance");
        unchecked {
            _balances[sender] = senderBalance - amount;
        }
        _balances[recipient] += amount;
        emit Transfer(sender, recipient, amount);
    }

    function _mint(address account, uint256 amount) internal {
        require(account != address(0), "QuorlinToken: mint to the zero address");
        _totalSupply += amount;
        _balances[account] += amount;
        emit Transfer(address(0), account, amount);
    }

    function _burn(address account, uint256 amount) internal {
        require(account != address(0), "QuorlinToken: burn from the zero address");
        uint256 accountBalance = _balances[account];
        require(accountBalance >= amount, "QuorlinToken: burn amount exceeds balance");
        unchecked {
            _balances[account] = accountBalance - amount;
        }
        _totalSupply -= amount;
        emit Transfer(account, address(0), amount);
    }

    function _approve(address owner_, address spender, uint256 amount) internal {
        require(owner_ != address(0), "QuorlinToken: approve from the zero address");
        require(spender != address(0), "QuorlinToken: approve to the zero address");
        _allowances[owner_][spender] = amount;
        emit Approval(owner_, spender, amount);
    }
}
