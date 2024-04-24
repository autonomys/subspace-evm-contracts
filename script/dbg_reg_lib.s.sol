// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {EndpointV2} from "@layerzerolabs/protocol/contracts/EndpointV2.sol";
import {ReceiveUln302} from "@layerzerolabs/messagelib/contracts/uln/uln302/ReceiveUln302.sol";
import {ILayerZeroEndpointV2} from "@layerzerolabs/protocol/contracts/interfaces/ILayerZeroEndpointV2.sol";

/* 
## For Sepolia
$ forge script ./script/dbg_reg_lib.s.sol:DebugRegLibScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL

*/

/// @dev This script is used only as part of debugging as per trial-1 due to `LZ_ULN_Verifying()` error.
///     Actually, This script could be run multiple times with updated `console2.log` in `ReceiveLibBase.sol`.
///     NOTE: Run only on Dst chain (eg. Sepolia) as the msg verification is done with ReceiveUln302 on dst chain.
contract DebugRegLibScript is Script, Test {
    address endpointV2Address = vm.envAddress("SEPOLIA_ENDPOINT_V2");

    ILayerZeroEndpointV2 endpointV2;
    ReceiveUln302 receiveUln302;

    uint32 srcEid = uint32(vm.envUint("NOVA_ENDPOINT_V2_ID"));
    uint32 dstEid = uint32(vm.envUint("SEPOLIA_ENDPOINT_V2_ID"));

    address delegate;

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);
    }

    function run() public {
        vm.startBroadcast(delegate);

        endpointV2 = ILayerZeroEndpointV2(endpointV2Address);

        receiveUln302 = new ReceiveUln302(address(endpointV2));
        endpointV2.registerLibrary(address(receiveUln302));

        address expectedDefaultSendMsgLib = endpointV2.defaultSendLibrary(srcEid);
        // endpointV2.setDefaultReceiveLibrary(srcEid, address(receiveUln302), 0);

        address expectedDefaultReceiveMsgLib = endpointV2.defaultReceiveLibrary(srcEid);

        assertEq(address(receiveUln302), expectedDefaultReceiveMsgLib);

        vm.stopBroadcast();
    }
}
