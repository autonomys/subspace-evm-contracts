pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {BytesLib} from "solidity-bytes-utils/contracts/BytesLib.sol";
import {Packet} from "@layerzerolabs/protocol/contracts/interfaces/ISendLib.sol";
import {ILayerZeroEndpointV2, Origin} from "@layerzerolabs/protocol/contracts/interfaces/ILayerZeroEndpointV2.sol";
import {IWTsscLz} from "../../src/interfaces/IWTsscLz.sol";
import {UlnConfig} from "@layerzerolabs/messagelib/contracts/uln/UlnBase.sol";
import {IReceiveLib} from "../../src/interfaces/IReceiveLib.sol";
import {IDVN} from "@layerzerolabs/messagelib/contracts/uln/interfaces/IDVN.sol";
import {PacketV1Codec} from "@layerzerolabs/protocol/contracts/messagelib/libs/PacketV1Codec.sol";
import {PacketV1CodecWrapper} from "@layerzerolabs/protocol/test/libtests/PacketV1Codec.t.sol";
/* 
    Run on Destination chain for DVN, Executor roles.

    Nova Anvil: forge script script/lz/AutoBridgeDVN.s.sol:AutoBridgeDVNScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url http://127.0.0.1:8545 --broadcast --legacy -vvvv
    Sepolia: forge script script/lz/AutoBridgeDVN.s.sol:AutoBridgeDVNScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL --broadcast -vvvv
*/

/// @dev This script is for manually executing the missed messages. Suppose, msg with nonce 1 is missed and msg with nonce 2 fails to execute via Bridge TS script.
///         In this case, we can manually execute the missed message (w nonce 1) via this script.
contract AutoBridgeDVNScript is Script, Test {
    ILayerZeroEndpointV2 endpointV2Dst;
    IReceiveLib receiveUlnE2Dst;
    IDVN dvnSrc;
    IWTsscLz wTsscLzDst;
    address delegate;
    uint32 srcEid;
    // uint32 dstEid;

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);

        // =================================================================================
        /* Disable comment for Nova to Sepolia */
        // dst contracts, EID
        endpointV2Dst = ILayerZeroEndpointV2(vm.envAddress("SEPOLIA_ENDPOINT_V2"));
        wTsscLzDst = IWTsscLz(payable(vm.envAddress("WTSSCLZ_SEPOLIA")));
        srcEid = uint32(vm.envUint("NOVA_ENDPOINT_V2_ID"));
        // =================================================================================

        // =================================================================================
        /* Disable comment for Sepolia to Nova */
        // dst contracts, EID
        // endpointV2Dst = ILayerZeroEndpointV2(vm.envAddress("NOVA_ENDPOINT_V2"));
        // wTsscLzDst = IWTsscLz(payable(vm.envAddress("WTSSCLZ_NOVA")));
        // srcEid = uint32(vm.envUint("SEPOLIA_ENDPOINT_V2_ID"));
        // =================================================================================
    }

    function run() public {
        vm.startBroadcast(delegate);

        // This encodedPacket is received from src chain (Nova here).
        // As this is a foundry script and can't be hosted to listen to emitted events, so hardcoded.
        bytes memory encodedPacket =
            hex"01000000000000000d00077a10000000000000000000000000a66782c958e08275566463cb76a7892e72f2edb100009ce10000000000000000000000008ecc60d2a42747742b9fc67fb25de774677e260e4305366c89dfe7e1d1efd0517970e906076681de77f1381ff4cad53de1441db9000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710";
        // 3. After receiving the fee, your DVN should query the address of the MessageLib on the destination chain
        address receiveUln302DstAddress = endpointV2Dst.defaultReceiveLibrary(srcEid);
        receiveUlnE2Dst = IReceiveLib(receiveUln302DstAddress);

        // 4. After your DVN has retrieved the receive MessageLib, you should read the MessageLib configuration from it.
        // In the configuration is the required block confirmations to wait before calling verify on the destination chain.
        // NOTE: the 1st arg could be wTsscLzSrc as well i.e. both the OFTs (on either chains) are allowed.
        UlnConfig memory ulnConfig = receiveUlnE2Dst.getUlnConfig(address(wTsscLzDst), srcEid);
        console2.log("Confirmations: ", ulnConfig.confirmations);

        bytes memory packetHeader = BytesLib.slice(encodedPacket, 0, 81);
        console2.log("Packet Header:");
        console2.logBytes(packetHeader);
        bytes32 _payloadHash = keccak256(BytesLib.slice(encodedPacket, 81, encodedPacket.length - 81));
        console2.log("Payload hash:");
        console2.logBytes32(_payloadHash);

        // done on src chain
        dvnSrc = IDVN(ulnConfig.requiredDVNs[0]);
        dvnSrc.verify(receiveUln302DstAddress, packetHeader, _payloadHash, ulnConfig.confirmations);

        // check if verifiable before committing to save gas (with failure)
        console2.log("Packet header hash");
        console2.logBytes32(keccak256(packetHeader));
        receiveUlnE2Dst.verifiable(ulnConfig, keccak256(packetHeader), _payloadHash);

        // verify and commit verification
        receiveUlnE2Dst.commitVerification(packetHeader, _payloadHash);

        Origin memory origin = Origin(
            PacketV1CodecWrapper.srcEid(encodedPacket),
            PacketV1CodecWrapper.sender(encodedPacket),
            PacketV1CodecWrapper.nonce(encodedPacket)
        );

        address _receiver = PacketV1CodecWrapper.receiverB20(encodedPacket);

        // call lzReceive function
        endpointV2Dst.lzReceive(
            origin, _receiver, PacketV1CodecWrapper.guid(encodedPacket), PacketV1CodecWrapper.message(encodedPacket), ""
        );

        vm.stopBroadcast();
    }
}
