// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {SendersTreasury} from "../src/SendersTreasury.sol";

/* 
    $ source .env
    $ forge script script/SendersTreasury.s.sol:SendersTreasuryScript --rpc-url $NOVA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL
*/
contract SendersTreasuryScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        SendersTreasury sendersTreasury = new SendersTreasury();
        console2.log("SendersTreasury SC deployed at", address(sendersTreasury));

        vm.stopBroadcast();
    }
}
