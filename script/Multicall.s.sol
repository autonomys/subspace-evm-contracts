// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Multicall3} from "../src/Multicall.sol";

/* 
    $ source .env
    $ forge script script/Multicall.s.sol:MulticallContractScript --rpc-url $SUBSPACE_EVM_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $VERIFIER_URL
*/

contract MulticallScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        Multicall3 multicall = new Multicall3();
        console2.log("Multicall SC deployed at", address(multicall));

        vm.stopBroadcast();
    }
}
