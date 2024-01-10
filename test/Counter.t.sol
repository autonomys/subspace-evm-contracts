// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Counter} from "../src/Counter.sol";

contract CounterTest is Test {
    Counter public counter;

    function setUp() public {
        counter = new Counter();
        counter.setNumber(0);
    }

    function test_Increment() public {
        // NOTE: Here, the count is same as 27523 always
        counter.increment();
        // assertEq(counter.number(), 1);
    }

    function test_SetNumber() public {
        // NOTE: Here, the count is same as 27488 always
        counter.setNumber(20000000000000000);
        assertEq(counter.number(), 20000000000000000);
    }

    function testFuzz_SetNumber(uint256 x) public {
        counter.setNumber(x);
        assertEq(counter.number(), x);
    }
}
