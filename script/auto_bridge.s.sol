// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {IWTsscLz} from "../src/interfaces/IWTsscLz.sol";
import {SendParam} from "@layerzerolabs/oapp/contracts/oft/interfaces/IOFT.sol";
import {OptionsBuilder} from "@layerzerolabs/oapp/contracts/oapp/libs/OptionsBuilder.sol";
import {MessagingFee} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

/// @notice AutoBridge Script demonstrates sending of TSSC token from Nova to Sepolia
/// @dev No need to deploy any contracts. Just read from a separately generated file
///     from other script. Here, `lz_infra_addresses.txt` is auto-generated from "LzInfra.s.sol" file
contract AutoBridgeScript is Script, Test {
    using OptionsBuilder for bytes;

    // Endpoint ID
    uint32 private constant SRC_EID = 490_000; // for Nova
    uint32 private constant DST_EID = 40161; // for Sepolia

    IWTsscLz public wTsscLz;

    address payable wTsscLzAddress = payable(vm.envAddress("WTSSCLZ_NOVA"));

    address delegate;

    function setUp() public {
        // uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        // delegate = vm.addr(privateKey);

        // For Anvil: `$ forge script ./script/LzInfra.s.sol:LzInfraScript --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://127.0.0.1:8545 --broadcast`
        delegate = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266); // testing for Anvil

        wTsscLz = IWTsscLz(wTsscLzAddress);
    }

    function run() public {
        vm.startBroadcast();

        console2.log("===before deposit: ", delegate.balance);
        console2.log("\tTSSC Balance: ", wTsscLz.balanceOf(delegate));
        console2.log("\tWTSSC Balance: ", wTsscLz.balanceOf(delegate));

        uint256 tokensToSend = 0.01 ether;

        // deposit
        vm.startPrank(delegate);
        wTsscLz.deposit{value: tokensToSend}();
        assertEq(wTsscLz.balanceOf(delegate), tokensToSend, "Minted must be same as Deposited.");

        // TODO: may need to change gasLimit
        bytes memory options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(200000, 0);
        SendParam memory sendParam = SendParam(
            DST_EID, bytes32(uint256(uint160(address(delegate))) << 96), tokensToSend, tokensToSend, options, "", ""
        );

        // get the quote
        MessagingFee memory fee = wTsscLz.quoteSend(sendParam, false);

        // send token to receiver (self)
        // TODO: may need to add gas limit
        wTsscLz.send{value: fee.nativeFee}(sendParam, fee, delegate);

        vm.stopPrank();

        vm.stopBroadcast();
    }
}
