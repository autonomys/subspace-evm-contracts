// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Fund} from "../src/Fund.sol";

contract FundTest is Test {
    Fund public fund;

    address public constant ALICE = address(0x123);
    address public constant BOB = address(0x456);
    address public constant EVE = address(0x789);
    // uint256 public constant FUND_AMOUNT = 44100000000000000;
    uint256 public constant FUND_AMOUNT = 10000;

    address[] public tos;

    function setUp() public {
        fund = new Fund();
    }

    /// @notice Transfer TSSC to single address
    function test_transfer_to_single() public {
        tos = [ALICE];
        fund.transferTsscToMany{value: FUND_AMOUNT}(tos);
        assertEq(address(ALICE).balance, FUND_AMOUNT);
    }

    /// @notice Transfer TSSC to multiple addresses
    function test_transfer_to_many() public {
        tos = [ALICE, BOB, EVE];

        fund.transferTsscToMany{value: FUND_AMOUNT * tos.length}(tos);

        assertEq(address(ALICE).balance, FUND_AMOUNT);
        assertEq(address(BOB).balance, FUND_AMOUNT);
        assertEq(address(EVE).balance, FUND_AMOUNT);
    }

    /// @notice Transfer TSSC to 8 addresses
    /// NOTE: Above 8 addresses test case is failing with error: "Failed to send Ether"
    function test_transfer_to_8_addrs() public {
        uint256 n = 8;
        for (uint256 i = 1; i <= n; ++i) {
            tos.push(address(uint160(i)));
        }

        uint256 requiredBal = FUND_AMOUNT * tos.length;
        assertGt(address(this).balance, requiredBal, "insufficient balance");

        fund.transferTsscToMany{value: requiredBal}(tos);

        assertEq(address(1).balance, FUND_AMOUNT);
        assertEq(address(2).balance, FUND_AMOUNT);
        assertEq(address(3).balance, FUND_AMOUNT);
    }

    // TODO: Add more tests
}
