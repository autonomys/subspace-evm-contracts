// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

/// @title Interface for WETH contract
interface IWETH {
    /// @notice Event emitted when Ether is deposited and WETH tokens are minted
    /// @param from The address of the depositor
    /// @param amount The amount of Ether deposited
    event Deposit(address indexed from, uint256 amount);

    /// @notice Event emitted when WETH tokens are burned and Ether is withdrawn
    /// @param to The address of the withdrawer
    /// @param amount The amount of Ether withdrawn
    event Withdrawal(address indexed to, uint256 amount);

    /// @notice Deposit Ether and mint corresponding WETH tokens to the sender's address
    function deposit() external payable;

    /// @notice Withdraw Ether by burning WETH tokens from the sender's address
    /// @param amount The amount of WETH tokens to burn
    function withdraw(uint256 amount) external;

    /// @notice Allow contract to receive Ether directly
    receive() external payable;
}
