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
///
///     Q. Why script in solidity, not in TS or other lang?
///     A. Because i need to see the details using foundry logging, and i can't see the details in TS ethers.
contract AutoBridgeScript is Script, Test {
    using OptionsBuilder for bytes;

    // Endpoint ID
    uint32 private constant LOCAL_EID = 490_000; // for Nova
    uint32 private constant REMOTE_EID = 40161; // for Sepolia

    IWTsscLz public wTsscLzNova;
    IWTsscLz public wTsscLzRemote;

    address payable wTsscLzAddressNova = payable(vm.envAddress("WTSSCLZ_NOVA"));
    address payable wTsscLzAddressRemote = payable(vm.envAddress("WTSSCLZ_SEPOLIA"));

    address private delegate;

    uint256 private constant tokensToSend = 0.01 ether;

    function isContract(address account) public view returns (bool) {
        uint256 size;
        assembly {
            size := extcodesize(account)
        }
        return (size > 0);
    }

    function setUp() public {
        // make sure the contracts in "lz_infra_addresses_nova.txt" are already deployed (on Nova or Anvil local).
        // disable comment when deploying on Nova
        assertEq(isContract(vm.envAddress("Treasury")), true, "Treasury should be deployed");
        assertEq(isContract(vm.envAddress("SimpleMessageLib")), true, "SimpleMessageLib should be deployed");
        assertEq(isContract(vm.envAddress("SendUln301")), true, "SendUln301 should be deployed");
        assertEq(isContract(vm.envAddress("SendUln302")), true, "SendUln302 should be deployed");
        assertEq(isContract(vm.envAddress("ReceiveUln301")), true, "ReceiveUln301 should be deployed");
        assertEq(isContract(vm.envAddress("ReceiveUln302")), true, "ReceiveUln302 should be deployed");
        assertEq(isContract(vm.envAddress("EndpointV1")), true, "EndpointV1 should be deployed");
        assertEq(isContract(vm.envAddress("EndpointV2")), true, "EndpointV2 should be deployed");
        assertEq(isContract(vm.envAddress("PriceFeed")), true, "PriceFeed should be deployed");
        assertEq(isContract(vm.envAddress("Executor")), true, "Executor should be deployed");
        assertEq(isContract(vm.envAddress("ExecutorFeeLib")), true, "ExecutorFeeLib should be deployed");
        assertEq(isContract(vm.envAddress("DVN")), true, "DVN should be deployed");
        assertEq(isContract(vm.envAddress("DVNFeeLib")), true, "DVNFeeLib should be deployed");

        // make sure delegate gets sufficient faucet from default addresses on Anvil.
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);

        wTsscLzNova = IWTsscLz(wTsscLzAddressNova);
        wTsscLzRemote = IWTsscLz(wTsscLzAddressRemote);
    }

    function run() public {
        vm.startBroadcast(delegate);

        sendTokenFromNova();

        // disable comment when deploying on Sepolia
        // NOTE: In order to check if the peer is correctly set on Sepolia, check via TS script from `layerzero-demo` repo.
        // It's not possible to set the peer on Sepolia simultaneously on Sepolia and Nova.
        // if peer is not set correctly on Remote, set it.
        // if (!wTsscLzRemote.isPeer(LOCAL_EID, bytes32(uint256(uint160(address(wTsscLzAddressNova)))))) {
        //     wTsscLzRemote.setPeer(LOCAL_EID, bytes32(uint256(uint160(address(wTsscLzAddressNova)))));
        // }

        vm.stopBroadcast();
    }

    function sendTokenFromNova() public {
        // if peer is not set correctly on Nova, set it.
        if (!wTsscLzNova.isPeer(REMOTE_EID, bytes32(uint256(uint160(address(wTsscLzAddressRemote)))))) {
            // NOTE: Won't execute if any action of the entire transaction fails
            wTsscLzNova.setPeer(REMOTE_EID, bytes32(uint256(uint160(address(wTsscLzAddressRemote)))));
        }

        uint256 preDepositBalance = wTsscLzNova.balanceOf(delegate);
        console2.log("===Before deposit:");
        console2.log("\tWTSSC Balance: ", preDepositBalance);

        // deposit
        if (tokensToSend > preDepositBalance) {
            // NOTE: Won't execute if any action of the entire transaction fails
            wTsscLzNova.deposit{value: tokensToSend - preDepositBalance}();

            console2.log("===After deposit:");
            console2.log("\tWTSSC Balance: ", wTsscLzNova.balanceOf(delegate));
        }

        // TODO: may need to change gasLimit
        bytes memory options = OptionsBuilder.newOptions().addExecutorLzReceiveOption(200000, 0);
        SendParam memory sendParam = SendParam(
            REMOTE_EID, bytes32(uint256(uint160(address(delegate)))), tokensToSend, tokensToSend, options, "", ""
        );

        // get the quote
        MessagingFee memory fee = wTsscLzNova.quoteSend(sendParam, false);

        // // send token to receiver (self)
        // // TODO: may need to add gas limit
        // wTsscLzNova.send{value: fee.nativeFee}(sendParam, fee, delegate);
    }
}
