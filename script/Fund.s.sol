// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Fund} from "../src/Fund.sol";

/* 
    $ source .env
    $ forge script script/Fund.s.sol:FundScript --rpc-url $SUBSPACE_EVM_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $VERIFIER_URL
*/
contract FundScript is Script {
    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        Fund fund = new Fund();
        console2.log("Fund SC deployed at", address(fund));

        vm.stopBroadcast();
    }
}
