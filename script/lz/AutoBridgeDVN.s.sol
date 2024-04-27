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
    Run on Destination chain (Sepolia here) for DVN, Executor roles.

    Sepolia: forge script script/lz/AutoBridgeDVN.sol:AutoBridgeDVNScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL --broadcast -vvvv
*/

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

        // dst contracts
        endpointV2Dst = ILayerZeroEndpointV2(vm.envAddress("SEPOLIA_ENDPOINT_V2"));
        wTsscLzDst = IWTsscLz(payable(vm.envAddress("WTSSCLZ_SEPOLIA")));

        // endpoint IDs
        // dstEid = endpointV2Dst.eid();
        srcEid = uint32(vm.envUint("NOVA_ENDPOINT_V2_ID"));
    }

    function run() public {
        vm.startBroadcast(delegate);

        // This encodedPacket is received from src chain (Nova here).
        // As this is a foundry script and can't be hosted to listen to emitted events, so hardcoded.
        bytes memory encodedPacket =
            hex"01000000000000002f00077a10000000000000000000000000fe4430822f79b1fb24f67593df68947de6a96b2800009ce1000000000000000000000000cfe0a0163b7f5bcea14e7beb4da1fe1a2136f5ff58bda940f1c9956f302cacd61974c7fc1b2a9746fd6aab410ed1d1012a7825ac000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710";
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

        // CLEANUP: remove later
        // verified
        // assertEq(uint256(receiveUln302View.verifiable(packetHeader,_payloadHash)), uint256(VerificationState.Verified));
        // executable
        // assertEq(uint256(lzExecutor.executable(origin, receiver)), uint256(ExecutionState.Executable));

        /* Executor's job */

        // commit and execute
        // NativeDropParam[] memory nativeDrop = new NativeDropParam[](0);
        // lzExecutor.commitAndExecute(
        //     address(receiveUln302),
        //     LzReceiveParam(origin, receiver, packet.guid, packet.message, "", 100000, 0),
        //     nativeDrop
        // );

        Origin memory origin = Origin(
            PacketV1CodecWrapper.srcEid(encodedPacket),
            PacketV1CodecWrapper.sender(encodedPacket),
            PacketV1CodecWrapper.nonce(encodedPacket)
        );

        address _receiver = PacketV1CodecWrapper.receiverB20(encodedPacket);

        // TODO: call lzReceive function
        endpointV2Dst.lzReceive(
            origin, _receiver, PacketV1CodecWrapper.guid(encodedPacket), PacketV1CodecWrapper.message(encodedPacket), ""
        );

        vm.stopBroadcast();
    }
}
