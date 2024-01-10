// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

/**
 * This contract is about load testing which means we need to see if high
 * computing function execution can be handled by the network or not
 */
contract Load {
    // ===========Storage==========================
    uint256[] public arr1;

    // ===========Functions=========================

    /// @dev Calculate factorial of a number
    function factorial(uint256 num) external pure returns (uint256) {
        uint256 fact = 1;
        for (uint256 i = 1; i <= num; ++i) {
            fact = fact * i;
        }

        return fact;
    }

    /// @dev Set values in an array
    function setArray(uint256 count) external {
        uint256[] memory arr = new uint256[](count);

        for (uint256 i = 0; i < count; ++i) {
            arr[i] = i * i * i * i;
        }

        arr1 = arr;
    }
}
