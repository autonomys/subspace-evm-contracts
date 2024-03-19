// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {WETH9} from "../src/WETH9.sol";

contract WETH9Test is Test {
    WETH9 public weth9;

    address public constant ALICE = address(0x123);
    address public constant BOB = address(0x456);
    address public constant EVE = address(0x789);
    // uint256 public constant FUND_AMOUNT = 44100000000000000;
    uint256 public constant FUND_AMOUNT = 10000;

    address[] public tos;

    function setUp() public {
        weth9 = new WETH9();
    }

    /// @notice Deposit TSSC in WETH9 contract
    function test_deposit() public {
        vm.deal(ALICE, FUND_AMOUNT);
        vm.prank(ALICE);
        weth9.deposit{value: FUND_AMOUNT}();
        assertEq(weth9.balanceOf(address(ALICE)), FUND_AMOUNT);
    }

    /// @notice Deposit TSSC in WETH9 contract and withdraw
    function test_deposit_and_withdraw() public {
        vm.deal(ALICE, FUND_AMOUNT);
        vm.prank(ALICE);
        weth9.deposit{value: FUND_AMOUNT}();
        assertEq(weth9.balanceOf(address(ALICE)), FUND_AMOUNT);

        vm.prank(ALICE);
        weth9.withdraw(FUND_AMOUNT);
        assertEq(weth9.balanceOf(address(ALICE)), 0);
        assertEq(address(ALICE).balance, FUND_AMOUNT);
    }

    /// @notice Deposit TSSC in WETH9 contract, transfer and withdraw
    function test_deposit_transfer_and_withdraw() public {
        vm.deal(ALICE, FUND_AMOUNT);
        vm.prank(ALICE);
        weth9.deposit{value: FUND_AMOUNT}();
        assertEq(weth9.balanceOf(address(ALICE)), FUND_AMOUNT);
        assertEq(weth9.balanceOf(address(ALICE)), FUND_AMOUNT);

        vm.prank(ALICE);
        weth9.transfer(BOB, FUND_AMOUNT);
        assertEq(weth9.balanceOf(address(ALICE)), 0);
        assertEq(weth9.balanceOf(address(BOB)), FUND_AMOUNT);

        vm.prank(BOB);
        weth9.withdraw(FUND_AMOUNT);
        assertEq(weth9.balanceOf(address(BOB)), 0);
        assertEq(address(BOB).balance, FUND_AMOUNT);
    }

    // TODO: Add more tests
}
