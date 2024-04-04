// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {WTsscLz} from "../src/WTsscLz.sol";

/* 
    $ source .env
    Nova: $ forge script script/WTsscLz.s.sol:WTsscLzScript --rpc-url $NOVA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL
    Sepolia: $ forge script script/WTsscLz.s.sol:WTsscLzScript --rpc-url $SEPOLIA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL
    // set peers for Sepolia contract
    $ cast send $MY_LZ_TOKEN_SEPOLIA "setPeer(uint32,bytes32)" $MUMBAI_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_MUMBAI) --private-key $DEPLOYER_PRIVATE_KEY -r $SEPOLIA_RPC_URL
    // set peers for Mumbai contract
    $ cast send $MY_LZ_TOKEN_MUMBAI "setPeer(uint32,bytes32)" $SEPOLIA_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_SEPOLIA) --private-key $DEPLOYER_PRIVATE_KEY -r $MUMBAI_RPC_URL

    // check if peer correctly set for Sepolia contract
    $ cast call $MY_LZ_TOKEN_SEPOLIA "isPeer(uint32,bytes32)" $MUMBAI_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_MUMBAI) -r $SEPOLIA_RPC_URL
    // check if peer correctly set for Mumbai contract
    $ cast call $MY_LZ_TOKEN_MUMBAI "isPeer(uint32,bytes32)" $SEPOLIA_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_SEPOLIA) -r $MUMBAI_RPC_URL

    // get quote of send tokens for Sepolia contract
    TODO: Add command
*/
contract WTsscLzScript is Script {
    // Provide Endpoint networks where this contract is to be deployed
    // address epContract = vm.envAddress("NOVA_ENDPOINT_V2");
    address private epContract = vm.envAddress("SEPOLIA_ENDPOINT_V2");
    // address private epContract = vm.envAddress("MUMBAI_ENDPOINT_V2");

    address delegate;

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);
    }

    function run() public {
        vm.startBroadcast();

        WTsscLz wTsscLz = new WTsscLz("Wrapped Subspace Token", "WTSSC", epContract, delegate);
        // Verify the contract was deployed successfully
        require(address(wTsscLz) != address(0), "Deployment failed");
        console2.log("WTsscLz contract deployed at ", address(wTsscLz));

        vm.stopBroadcast();
    }
}
