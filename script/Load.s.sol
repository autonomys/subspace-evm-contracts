// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Load} from "../src/Load.sol";

/* 
    $ source .env
    $ forge script script/Load.s.sol:LoadContractScript --rpc-url $NOVA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $VERIFIER_URL
*/

contract LoadScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        Load load = new Load();
        console2.log("Load SC deployed at", address(load));

        vm.stopBroadcast();
    }
}
