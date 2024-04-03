// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import {Script, console2} from "forge-std/Script.sol";
import {EndpointV2} from "../src/EndpointV2.sol";

/* 
    $ source .env
    $ forge script script/EndpointV2.s.sol:EndpointV2Script --r $NOVA_RPC_URL --private-key $DEPLOYER_PRIVATE_KEY --broadcast --verify --verifier blockscout --verifier-url $VERIFIER_URL

    // set peers for Sepolia contract
    $ cast send $MY_LZ_TOKEN_SEPOLIA "setPeer(uint32,bytes32)" $MUMBAI_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_MUMBAI) --private-key $DEPLOYER_PRIVATE_KEY -r $SEPOLIA_RPC_URL
    // set peers for Mumbai contract
    $ cast send $MY_LZ_TOKEN_MUMBAI "setPeer(uint32,bytes32)" $SEPOLIA_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_SEPOLIA) --private-key $DEPLOYER_PRIVATE_KEY -r $MUMBAI_RPC_URL

    // check if peer correctly set for Sepolia contract
    $ cast call $MY_LZ_TOKEN_SEPOLIA "isPeer(uint32,bytes32)" $MUMBAI_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_MUMBAI) -r $SEPOLIA_RPC_URL
    // check if peer correctly set for Mumbai contract
    $ cast call $MY_LZ_TOKEN_MUMBAI "isPeer(uint32,bytes32)" $SEPOLIA_ENDPOINT_V2_ID $(cast --to-bytes32 $MY_LZ_TOKEN_SEPOLIA) -r $MUMBAI_RPC_URL

    // get quote of send tokens for Sepolia contract
*/
contract EndpointV2Script is Script {
    address private owner;
    uint32 private epId;

    function setUp() public {
        epId = uint32(vm.envUint("NOVA_ENDPOINT_V2_ID"));
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        owner = vm.addr(privateKey);
    }

    function run() public {
        vm.startBroadcast();

        EndpointV2 endpointV2 = new EndpointV2(epId, owner);
        // Verify the contract was deployed successfully
        require(address(endpointV2) != address(0), "Deployment failed");
        console2.log("EndpointV2 contract deployed at ", address(endpointV2));

        vm.stopBroadcast();
    }
}
