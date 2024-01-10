// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {Load} from "../src/Load.sol";

contract LoadTest is Test {
    Load public load;

    function setUp() public {
        load = new Load();
    }

    function test_Factorial() public {
        uint256 fact = load.factorial(10);
        assertEq(fact, 3628800);
    }

    /// fuzz test for factorial
    function testFuzz_Factorial(uint256 num) public view {
        vm.assume(num > 0 && num < 58);
        uint256 fact = load.factorial(num);
    }

    // test for setArray
    function test_SetArray_Max() public {
        // NOTE: This is the best count value which consumes `59978072`
        // gas which is less than the block limit. So, we can say that
        // this function can be executed in a single block with this
        // count value.
        load.setArray(2650);
        assertEq(load.arr1(0), 0);
        assertEq(load.arr1(1), 1);
        assertEq(load.arr1(2), 16);
    }

    // test for setArray with n count
    function test_SetArray_n() public {
        load.setArray(2200);
        // assertEq(load.arr1(0), 0);
        // assertEq(load.arr1(1), 1);
        // assertEq(load.arr1(2), 16);
    }
    // test for setArray with n count after storage initialization.

    function test_SetArray_n_after_storage_init() public {
        load.setArray(2200);
        load.setArray(2200);
        load.setArray(2200);
        // assertEq(load.arr1(0), 0);
        // assertEq(load.arr1(1), 1);
        // assertEq(load.arr1(2), 16);
    }
}
