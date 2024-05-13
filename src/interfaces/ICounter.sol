// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

interface ICounter {
    // Declares the public variable 'number' for external access
    function number() external view returns (uint256);

    // Event declaration for setting a new number
    event NumberSet(address indexed caller, uint256 newNumber);

    // Event declaration for incrementing the number
    event Incremented(address indexed caller);

    // Function to set a new number
    function setNumber(uint256 newNumber) external;

    // Function to increment the number
    function increment() external;
}
