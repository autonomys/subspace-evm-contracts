pragma solidity ^0.8.20;

import {Script, console2} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {BytesLib} from "solidity-bytes-utils/contracts/BytesLib.sol";
import {Packet} from "@layerzerolabs/protocol/contracts/interfaces/ISendLib.sol";
import {ILayerZeroEndpointV2} from "@layerzerolabs/protocol/contracts/interfaces/ILayerZeroEndpointV2.sol";
import {IWTsscLz} from "../src/interfaces/IWTsscLz.sol";
import {UlnConfig} from "@layerzerolabs/messagelib/contracts/uln/UlnBase.sol";
import {IReceiveLib} from "../src/interfaces/IReceiveLib.sol";

/* 
    Test on Destination chain (Sepolia here) for DVN, Executor roles.
*/
contract AutoBridgeDVNScript is Script, Test {
    ILayerZeroEndpointV2 endpointV2Dst;
    IReceiveLib receiveUlnE2Dst;
    IWTsscLz wTsscLzDst;
    address delegate;
    uint32 srcEid; // of other chain
    uint32 dstEid; // of this chain

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);

        // dst contracts
        endpointV2Dst = ILayerZeroEndpointV2(vm.envAddress("SEPOLIA_ENDPOINT_V2"));
        wTsscLzDst = IWTsscLz(payable(vm.envAddress("WTSSCLZ_SEPOLIA")));

        // endpoint IDs
        dstEid = endpointV2Dst.eid();
        srcEid = uint32(vm.envUint("NOVA_ENDPOINT_V2_ID"));
    }

    function run() public {
        vm.startBroadcast(delegate);

        // This encodedPacket is received from src chain (Nova here).
        // As this is a foundry script and can't be hosted to listen to emitted events, so hardcoded.
        bytes memory encodedPacket =
            hex"01000000000000000300077a10000000000000000000000000378e37eb673fb0604aaad644c8813e084c38ab4100009ce100000000000000000000000087aca95fb76d1617fcb068c4154594ec6149b0ffd1a1330cee6cf6dd16fd47ceefe8e7b1857e999ce4f6a3af26a69694419a8dd4000000000000000000000000b751710af8ce68677ab960adb103060f38d097140000000000002710";

        // 3. After receiving the fee, your DVN should query the address of the MessageLib on the destination chain
        address receiveUlnE2DstAddress = endpointV2Dst.defaultReceiveLibrary(srcEid);
        receiveUlnE2Dst = IReceiveLib(receiveUlnE2DstAddress);

        // 4. After your DVN has retrieved the receive MessageLib, you should read the MessageLib configuration from it.
        // In the configuration is the required block confirmations to wait before calling verify on the destination chain.
        // NOTE: the 1st arg could be wTsscLzSrc as well i.e. both the OFTs (on either chains) are allowed.
        UlnConfig memory ulnConfig = receiveUlnE2Dst.getUlnConfig(address(wTsscLzDst), srcEid);
        console2.log("Confirmations: ", ulnConfig.confirmations);

        // // Packet memory packet = deserializePacket(encodedPacket);
        // (Packet memory packet) = decode(encodedPacket);

        // // (nonce, srcEid, sender, dstEid, receiver, guid, mesaage) =
        // //     abi.decode(data, (uint64, uint32, address, uint32, bytes32, bytes32, bytes, uint256[], MyStruct));
        // console2.log(packet.nonce);
        // console2.log(packet.srcEid);
        // console2.log(packet.sender);
        // console2.log(packet.dstEid);
        // console2.logBytes32(packet.receiver);
        // console2.logBytes32(packet.guid);
        // console2.logBytes(packet.message);

        bytes memory packetHeader = BytesLib.slice(encodedPacket, 0, 81);
        console2.log("Packet Header:");
        console2.logBytes(packetHeader);
        bytes32 payloadHash = keccak256(BytesLib.slice(encodedPacket, 81, encodedPacket.length - 81));
        console2.log("Payload hash:");
        console2.logBytes32(payloadHash);

        // verify [OPTIONAL]. It's alredy included in `commitVerification` step.
        // receiveUlnE2Dst.verify(packetHeader, payloadHash, ulnConfig.confirmations);

        // check if verifiable before committing to save gas (with failure)
        console2.log("Packet header hash");
        console2.logBytes32(keccak256(packetHeader));
        vm.stopBroadcast();
        receiveUlnE2Dst.verifiable(ulnConfig, keccak256(packetHeader), payloadHash);

        vm.broadcast(ulnConfig.requiredDVNs[0]);
        // verify and commit verification
        receiveUlnE2Dst.commitVerification(packetHeader, payloadHash);

        // verified
        // assertEq(uint256(receiveUln302View.verifiable(packetHeader, payloadHash)), uint256(VerificationState.Verified));
        // executable
        // assertEq(uint256(lzExecutor.executable(origin, receiver)), uint256(ExecutionState.Executable));

        // commit and execute
        // NativeDropParam[] memory nativeDrop = new NativeDropParam[](0);
        // lzExecutor.commitAndExecute(
        //     address(receiveUln302),
        //     LzReceiveParam(origin, receiver, packet.guid, packet.message, "", 100000, 0),
        //     nativeDrop
        // );

        // vm.stopBroadcast();
    }

    // function decode(bytes memory data) internal pure returns (Packet memory packet) {
    //     // (uint x, address addr, uint[] memory arr, MyStruct myStruct) = ...
    //     (packet) = abi.decode(data, (Packet));
    // }

    // function deserializePacket(bytes memory data) public pure returns (Packet memory) {
    //     Packet memory packet;
    //     uint256 offset = 33; // Start reading after the length prefix of dynamic bytes array

    //     uint64 nonce;
    //     uint32 srcEid;
    //     address sender;
    //     uint32 dstEid;
    //     bytes32 receiver;
    //     bytes32 guid;
    //     bytes memory message;

    //     assembly {
    //         // Read uint64 nonce
    //         nonce := mload(add(data, offset))

    //         // Read uint32 srcEid, positioned right after nonce
    //         srcEid := mload(add(data, add(offset, 8)))

    //         // Read address sender, positioned right after srcEid
    //         sender := mload(add(data, add(offset, 12)))

    //         // Read uint32 dstEid, positioned after sender
    //         dstEid := mload(add(data, add(offset, 32)))

    //         // Read bytes32 receiver, positioned right after dstEid
    //         receiver := mload(add(data, add(offset, 36)))

    //         // Read bytes32 guid, positioned right after receiver
    //         guid := mload(add(data, add(offset, 68)))

    //         // Handle the message bytes
    //         let messageOffset := add(data, add(offset, 100))
    //         let messageLength := mload(messageOffset)
    //         message := mload(add(messageOffset, 32))
    //     }

    //     // Assign values from temporary variables
    //     packet.nonce = nonce;
    //     packet.srcEid = srcEid;
    //     packet.sender = sender;
    //     packet.dstEid = dstEid;
    //     packet.receiver = receiver;
    //     packet.guid = guid;
    //     packet.message = message;

    //     return packet;
    // }
}
