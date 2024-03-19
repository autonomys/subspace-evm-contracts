// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {WETH9} from "../src/WETH9.sol";

contract WETH9Script is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        WETH9 weth9 = new WETH9();
        console2.log("WETH9 SC deployed at", address(weth9));

        vm.stopBroadcast();
    }
}
