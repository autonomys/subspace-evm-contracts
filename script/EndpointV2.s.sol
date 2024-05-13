// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {EndpointV2} from "@layerzerolabs/protocol/contracts/EndpointV2.sol";

/* 
## For Nova
`$ forge script ./script/EndpointV2.s.sol:EndpointV2Script --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${NOVA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL`

## For Sepolia
`$ forge script ./script/EndpointV2.s.sol:EndpointV2Script --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL`

*/

/// @notice Deploy EndpointV2 contract for native LZ Infra.
/// @dev This contract is deployed separately in order to make my account as owner
///     in order to register message libraries (onlyOwner access) for native LZ Infra.
contract EndpointV2Script is Script {
    // uint32 private constant LOCAL_EID = 490000; // for Nova
    uint32 private constant LOCAL_EID = 40161; // for Sepolia

    address delegate;

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);
    }

    function run() public {
        vm.startBroadcast(delegate);

        EndpointV2 endpointV2 = new EndpointV2(LOCAL_EID, delegate);

        vm.stopBroadcast();
    }
}
