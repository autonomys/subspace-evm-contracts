// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {Test} from "forge-std/Test.sol";
import {IWTsscLz} from "../src/interfaces/IWTsscLz.sol";
import {SendParam} from "@layerzerolabs/oapp/contracts/oft/interfaces/IOFT.sol";
import {OptionsBuilder} from "@layerzerolabs/oapp/contracts/oapp/libs/OptionsBuilder.sol";
import {MessagingFee} from "@layerzerolabs/lz-evm-protocol-v2/contracts/interfaces/ILayerZeroEndpointV2.sol";

/* 

## For Nova Anvil
$ forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8545 --broadcast --legacy -vvvv

## For Nova
$ forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${NOVA_RPC_URL} --broadcast --legacy -vvvv

## For Sepolia
$ forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast -vvvv

*/
/// @notice AutoBridge Script demonstrates sending of TSSC token from Nova to Sepolia
/// @dev No need to deploy any contracts. Just use from a separately generated file
///     from other script. Here, `lz_infra_addresses.txt` is auto-generated from "LzInfra.s.sol" file
///
///     Q. Why script in solidity, not in TS or other lang?
///     A. For debug purpose to view intermediate logs that can't be seen in TS.
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

    function setUp() public {
        // NOTE: make sure the contracts in "lz_infra_addresses_nova.txt" are already deployed (on Nova or Anvil local).
        // NOTE: Also, if required, the endpointV2 address needs to be added to WTsscLz contract or else, need to redeploy it.
        //       And if WTsscLz contract address changes, then need to `setPeer` on WTsscLz contract on Sepolia.

        // make sure delegate gets sufficient faucet from default addresses on Anvil.
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);

        wTsscLzNova = IWTsscLz(wTsscLzAddressNova);
        wTsscLzRemote = IWTsscLz(wTsscLzAddressRemote);
    }

    function run() public {
        vm.startBroadcast(delegate);

        // disable comment when deploying on Nova
        sendTokenFromNova();

        // disable comment when deploying on Sepolia
        // checkAndSetPeer(wTsscLzRemote, LOCAL_EID, wTsscLzAddressNova);

        vm.stopBroadcast();
    }

    // NOTE: It's not possible to set the peer on Sepolia simultaneously on Sepolia and Nova unlike TS Script.
    //          Won't execute if any action of the entire transaction fails
    // if peer is incorrectly/not set on Remote, set it.
    function checkAndSetPeer(IWTsscLz oftA, uint32 localEid, address oftBAddres) private {
        if (!oftA.isPeer(localEid, bytes32(uint256(uint160(oftBAddres))))) {
            oftA.setPeer(localEid, bytes32(uint256(uint160(oftBAddres))));
        }
    }

    function sendTokenFromNova() private {
        checkAndSetPeer(wTsscLzNova, REMOTE_EID, wTsscLzAddressRemote);

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
        console2.log("===Quote fee: ", fee.nativeFee);

        // send token to receiver (self)
        // TODO: may need to add gas limit
        wTsscLzNova.send{value: fee.nativeFee}(sendParam, fee, delegate);
    }
}
