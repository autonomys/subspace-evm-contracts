// SPDX-License-Identifier: UNLICENSED
pragma solidity 0.8.24;

contract Counter {
    uint256 public number;

    event NumberSet(address indexed caller, uint256 newNumber);
    event Incremented(address indexed caller);

    function setNumber(uint256 newNumber) public {
        number = newNumber;
        emit NumberSet(msg.sender, newNumber);
    }

    function increment() public {
        number++;
        emit Incremented(msg.sender);
    }
}
