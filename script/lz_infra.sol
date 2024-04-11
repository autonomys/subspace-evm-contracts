// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {EndpointV2} from "@layerzerolabs/protocol/contracts/EndpointV2.sol";
import {SendUln302} from "@layerzerolabs/messagelib/contracts/uln/uln302/SendUln302.sol";
import {ReceiveUln302} from "@layerzerolabs/messagelib/contracts/uln/uln302/ReceiveUln302.sol";
import {PriceFeed} from "@layerzerolabs/messagelib/contracts/PriceFeed.sol";
import {IDVN} from "@layerzerolabs/messagelib/contracts/uln/interfaces/IDVN.sol";
import {DVNFeeLib} from "@layerzerolabs/messagelib/contracts/uln/dvn/DVNFeeLib.sol";
import {DVN} from "@layerzerolabs/messagelib/contracts/uln/dvn/DVN.sol";
import {Executor} from "@layerzerolabs/messagelib/contracts/Executor.sol";
import {ExecutorFeeLib} from "@layerzerolabs/messagelib/contracts/ExecutorFeeLib.sol";
import {SendUln301} from "@layerzerolabs/messagelib/contracts/uln/uln301/SendUln301.sol";
import {ReceiveUln301} from "@layerzerolabs/messagelib/contracts/uln/uln301/ReceiveUln301.sol";
import {Treasury} from "@layerzerolabs/messagelib/contracts/Treasury.sol";
import {TreasuryFeeHandler} from "@layerzerolabs/messagelib/contracts/uln/uln301/TreasuryFeeHandler.sol";
import {NonceContractMock as NonceContract} from
    "@layerzerolabs/messagelib/contracts/uln/uln301/mocks/NonceContractMock.sol";
import {EndpointV1} from "@layerzerolabs/messagelib/test/mocks/EndpointV1.sol";
import {SimpleMessageLib} from "@layerzerolabs/protocol/contracts/messagelib/SimpleMessageLib.sol";

/* 

For Anvil: `$ forge script ./script/lz_infra.sol:LzInfraScript --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://127.0.0.1:8545 --broadcast`


*/

contract LzInfraScript is Script {
    // Provide Endpoint networks where this contract is to be deployed
    // address epContract = vm.envAddress("NOVA_ENDPOINT_V2");
    // address epContract = vm.envAddress("SEPOLIA_ENDPOINT_V2");

    // Endpoint
    uint32 private constant EID = 490_000; // for Nova
    uint32 private constant REMOTE_EID = 40161; // for Sepolia

    // SendUln302
    uint256 private constant TREASURY_GAS_CAP = 100_000;
    uint256 private constant TREASURY_GAS_FOR_FEE_CAP = 100_000;

    // DVN
    uint256 private constant NATIVE_DECIMAL_RATE = 1e18;

    address delegate;

    SimpleMessageLib simpleMessageLib;
    EndpointV2 endpointV2;
    SendUln302 sendUln302;
    ReceiveUln302 receiveUln302;
    PriceFeed priceFeed;
    Treasury treasury;
    TreasuryFeeHandler feeHandler;

    ReceiveUln301 receiveUln301;
    SendUln301 sendUln301;
    DVN dvn;
    DVNFeeLib feeLib;
    Executor executor;
    ExecutorFeeLib executorFeeLib;

    function setUp() public {
        // uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        // delegate = vm.addr(privateKey);

        // For Anvil: `$ forge script ./script/lz_infra.sol:LzInfraScript --private-key 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80 --rpc-url http://127.0.0.1:8545 --broadcast`
        delegate = address(0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266); // testing for Anvil
    }

    function run() public {
        vm.startBroadcast();

        // Endpoint V2 for Nova
        endpointV2 = new EndpointV2(EID, delegate);

        // Message Libs (Simple, SendUln, ReceiveUln) for Nova
        treasury = new Treasury();
        simpleMessageLib = new SimpleMessageLib(address(endpointV2), address(treasury));
        // TODO: verify treasuryGasCap, treasuryGasForFeeCap for Nova ??
        sendUln302 = new SendUln302(address(endpointV2), TREASURY_GAS_CAP, TREASURY_GAS_FOR_FEE_CAP);
        receiveUln302 = new ReceiveUln302(address(endpointV2));
        // TODO: Add `receiveUln302::setDefaultUlnConfigs`

        // register all 3 message libs. BlockedMessageLib is already registered during EP deployment.
        endpointV2.registerLibrary(address(simpleMessageLib));
        endpointV2.registerLibrary(address(sendUln302));
        endpointV2.registerLibrary(address(receiveUln302));

        // Deploy for Executor
        EndpointV1 endpointV1 = new EndpointV1(uint16(EID));
        feeHandler = new TreasuryFeeHandler(address(endpointV1));
        sendUln301 = new SendUln301(
            address(endpointV1),
            TREASURY_GAS_CAP,
            TREASURY_GAS_FOR_FEE_CAP,
            address(new NonceContract(address(endpointV1))),
            EID,
            address(feeHandler)
        );
        receiveUln301 = new ReceiveUln301(address(endpointV1), EID);

        // PriceFeed
        priceFeed = new PriceFeed();
        priceFeed.initialize(delegate);
        // TODO: More?

        // DVN
        address[] memory libs = new address[](2);
        libs[0] = address(sendUln302);
        libs[1] = address(receiveUln302);
        address[] memory signers = new address[](1);
        signers[0] = delegate;
        address[] memory admins = new address[](1);
        admins[0] = delegate;
        dvn = new DVN(EID, libs, address(priceFeed), signers, 1, admins);
        IDVN.DstConfigParam[] memory dstConfigParams = new IDVN.DstConfigParam[](1);
        dstConfigParams[0] = IDVN.DstConfigParam({dstEid: REMOTE_EID, gas: 5000, multiplierBps: 0, floorMarginUSD: 0}); // TODO: confirm gas... ??
        dvn.setDstConfig(dstConfigParams);
        DVNFeeLib dvnFeeLib = new DVNFeeLib(NATIVE_DECIMAL_RATE);
        dvn.setWorkerFeeLib(address(dvnFeeLib));

        // Executor
        executor = new Executor();
        executorFeeLib = new ExecutorFeeLib(NATIVE_DECIMAL_RATE);
        {
            address[] memory libs2 = new address[](3);
            // FIXME: Issue raised: https://github.com/LayerZero-Labs/LayerZero-v2/issues/58
            libs2[0] = address(sendUln301); // TODO: can be removed?
            // libs[1] = address(receiveUln301);
            libs2[1] = address(receiveUln302); // TODO: verify
            libs2[2] = address(sendUln302);
            executor.initialize(address(endpointV2), address(receiveUln301), libs, address(priceFeed), delegate, admins);
            executor.setWorkerFeeLib(address(executorFeeLib));
        }

        // TODO: Wire remote

        vm.stopBroadcast();
    }
}
