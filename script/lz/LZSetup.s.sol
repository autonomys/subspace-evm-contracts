// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Script, console2} from "forge-std/Script.sol";
import {EndpointV2} from "@layerzerolabs/protocol/contracts/EndpointV2.sol";
import {EndpointV2View} from "@layerzerolabs/protocol/contracts/EndpointV2View.sol";
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
import {Strings} from "@openzeppelin/contracts/utils/Strings.sol";
import {IExecutor} from "@layerzerolabs/messagelib/contracts/interfaces/IExecutor.sol";
import {UlnConfig, SetDefaultUlnConfigParam} from "@layerzerolabs/messagelib/contracts/uln/UlnBase.sol";
import {SetDefaultExecutorConfigParam, ExecutorConfig} from "@layerzerolabs/messagelib/contracts/SendLibBase.sol";
import {ILayerZeroEndpointV2} from "@layerzerolabs/protocol/contracts/interfaces/ILayerZeroEndpointV2.sol";

/* 
NOTE: For Nova & Sepolia to support cross-chain messages, you have to deploy the whole script flow on each chain, wiring up for each other.

## Using Anvil
### For Nova
1. `$ anvil --fork-url $NOVA_RPC_URL --port 8545`
2. `$ forge script ./script/lz/LZSetup.s.sol:LZSetupScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8545 --broadcast`

### For Sepolia
1. `$ anvil --fork-url $SEPOLIA_RPC_URL --port 8546`
2. `$ forge script ./script/lz/LZSetup.s.sol:LZSetupScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8546 --broadcast`


## For Nova
`$ forge script ./script/lz/LZSetup.s.sol:LZSetupScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${NOVA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $NOVA_VERIFIER_URL`

## For Sepolia
`$ forge script ./script/lz/LZSetup.s.sol:LZSetupScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url ${SEPOLIA_RPC_URL} --broadcast  --verify --verifier blockscout --verifier-url $ETHSEPOLIA_VERIFIER_URL`

*/
/// @title LZ Setup Script
/// @author abhi3700
/// @notice LZ Setup Script
/// @dev Setup LZ for Nova & Sepolia in 2 runs respectively
contract LZSetupScript is Script {
    // Endpoint address, ids,
    // NOTE: Disable comment when deploying on Nova
    address endpointV2Address = vm.envAddress("NOVA_ENDPOINT_V2");
    uint32 private constant LOCAL_EID = 490_000; // for Nova
    uint32 private constant REMOTE_EID = 40161; // for Sepolia
    string constant FILE_NAME = "./lzsetup_addresses_nova.txt";

    // Endpoint address, ids,
    // NOTE: Disable comment when deploying on Sepolia
    // address endpointV2Address = vm.envAddress("SEPOLIA_ENDPOINT_V2");
    // uint32 private constant LOCAL_EID = 40161; // for Sepolia
    // uint32 private constant REMOTE_EID = 490_000; // for Nova
    // string constant FILE_NAME = "./lzsetup_addresses_sepolia.txt";

    // SendUln302
    uint256 private constant TREASURY_GAS_CAP = 100_000;
    uint256 private constant TREASURY_GAS_FOR_FEE_CAP = 100_000;

    // DVN
    uint256 private constant NATIVE_DECIMAL_RATE = 1e18;

    address delegate;

    SimpleMessageLib simpleMessageLib;
    // ILayerZeroEndpointV2 endpointV2;
    EndpointV2 endpointV2;
    EndpointV2View endpointV2View;
    SendUln302 sendUln302;
    ReceiveUln302 receiveUln302;
    PriceFeed priceFeed;
    Treasury treasury;
    TreasuryFeeHandler feeHandler;

    EndpointV1 endpointV1;
    ReceiveUln301 receiveUln301;
    SendUln301 sendUln301;
    DVN dvn;
    DVNFeeLib dvnFeeLib;
    Executor executor;
    ExecutorFeeLib executorFeeLib;

    function setUp() public {
        uint256 privateKey = vm.envUint("DEPLOYER_PRIVATE_KEY");
        delegate = vm.addr(privateKey);
    }

    function run() public {
        vm.startBroadcast(delegate);

        // Endpoint V2
        // endpointV2 = ILayerZeroEndpointV2(endpointV2Address);
        endpointV2 = new EndpointV2(LOCAL_EID, delegate);

        // for viewing if a message is at what state - verifiable/executable/....
        endpointV2View = new EndpointV2View();
        endpointV2View.initialize(address(endpointV2));

        treasury = new Treasury();

        // Message Libs (Simple, SendUln, ReceiveUln) for Nova
        simpleMessageLib = new SimpleMessageLib(address(endpointV2), address(treasury));
        // TODO: verify treasuryGasCap, treasuryGasForFeeCap for Nova ??
        sendUln302 = new SendUln302(address(endpointV2), TREASURY_GAS_CAP, TREASURY_GAS_FOR_FEE_CAP);
        receiveUln302 = new ReceiveUln302(address(endpointV2));
        // TODO: Add `receiveUln302::setDefaultUlnConfigs`

        // register all 3 message libs. BlockedMessageLib is already registered during EP deployment.
        // endpointV2.
        endpointV2.registerLibrary(address(simpleMessageLib));
        endpointV2.registerLibrary(address(sendUln302));
        endpointV2.registerLibrary(address(receiveUln302));

        // Deploy for Executor
        endpointV1 = new EndpointV1(uint16(LOCAL_EID));
        feeHandler = new TreasuryFeeHandler(address(endpointV1));
        sendUln301 = new SendUln301(
            address(endpointV1),
            TREASURY_GAS_CAP,
            TREASURY_GAS_FOR_FEE_CAP,
            address(new NonceContract(address(endpointV1))),
            LOCAL_EID,
            address(feeHandler)
        );
        receiveUln301 = new ReceiveUln301(address(endpointV1), LOCAL_EID);

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
        dvn = new DVN(LOCAL_EID, libs, address(priceFeed), signers, 1, admins);
        IDVN.DstConfigParam[] memory dvnDstConfigParams = new IDVN.DstConfigParam[](1);
        dvnDstConfigParams[0] =
            IDVN.DstConfigParam({dstEid: REMOTE_EID, gas: 5000, multiplierBps: 0, floorMarginUSD: 0}); // TODO: confirm gas... ??
        dvn.setDstConfig(dvnDstConfigParams);
        dvnFeeLib = new DVNFeeLib(NATIVE_DECIMAL_RATE);
        dvn.setWorkerFeeLib(address(dvnFeeLib));
        dvn.setDefaultMultiplierBps(100); // TODO: verify 100 ?

        // Executor
        executor = new Executor();
        executorFeeLib = new ExecutorFeeLib(NATIVE_DECIMAL_RATE);
        {
            address[] memory libs2 = new address[](3);
            // FIXME: Issue raised: https://github.com/LayerZero-Labs/LayerZero-v2/issues/58
            // Temporarily, have to support both the v1 for Executor. LzExecutor is for LZ Scan explorer.
            libs2[0] = address(sendUln301); // TODO: can be removed?
            // libs[1] = address(receiveUln301);
            libs2[1] = address(receiveUln302); // TODO: verify
            libs2[2] = address(sendUln302);
            executor.initialize(
                address(endpointV2), address(receiveUln301), libs2, address(priceFeed), delegate, admins
            );
            executor.setWorkerFeeLib(address(executorFeeLib));
        }

        // Wire remote
        IExecutor.DstConfigParam[] memory executorDstConfigParams = new IExecutor.DstConfigParam[](1);
        executorDstConfigParams[0] = IExecutor.DstConfigParam({
            dstEid: REMOTE_EID,
            baseGas: 5000,
            multiplierBps: 10000,
            floorMarginUSD: 1e10,
            nativeCap: 1 gwei
        });
        executor.setDstConfig(executorDstConfigParams);

        address[] memory dvns = new address[](1);
        dvns[0] = address(dvn);
        UlnConfig memory ulnConfig = UlnConfig(1, uint8(dvns.length), 0, 0, dvns, new address[](0));
        SetDefaultUlnConfigParam[] memory ulnConfigParams = new SetDefaultUlnConfigParam[](1);
        ulnConfigParams[0] = SetDefaultUlnConfigParam(REMOTE_EID, ulnConfig);

        // set send uln config
        SetDefaultExecutorConfigParam[] memory executorConfigParams = new SetDefaultExecutorConfigParam[](1);
        executorConfigParams[0] = SetDefaultExecutorConfigParam(REMOTE_EID, ExecutorConfig(1000, address(executor)));
        sendUln302.setDefaultExecutorConfigs(executorConfigParams);
        sendUln302.setDefaultUlnConfigs(ulnConfigParams);

        // set receive uln config
        receiveUln302.setDefaultUlnConfigs(ulnConfigParams);

        endpointV2.setDefaultSendLibrary(REMOTE_EID, address(sendUln302));
        endpointV2.setDefaultReceiveLibrary(REMOTE_EID, address(receiveUln302), 0);
        // NOTE: OPTIONAL: set receive library is to overwrite the previously set lib with a timeout (in blocks).
        // endpointV2.setReceiveLibrary(address(wTsscLzRemote), REMOTE_EID, address(receiveUln302), 0);

        // export contract, lib addresses so that it can be used in
        //      another script to get info like quotes; send txs.
        exportAddresses();

        vm.stopBroadcast();
    }

    function exportAddresses() internal {
        // Step 1: Convert addresses to hexadecimal strings
        string memory treasuryHex = Strings.toHexString(uint256(uint160(address(treasury))), 20);
        string memory simpleMessageLibHex = Strings.toHexString(uint256(uint160(address(simpleMessageLib))), 20);
        string memory sendUln301Hex = Strings.toHexString(uint256(uint160(address(sendUln301))), 20);
        string memory sendUln302Hex = Strings.toHexString(uint256(uint160(address(sendUln302))), 20);
        string memory receiveUln301Hex = Strings.toHexString(uint256(uint160(address(receiveUln301))), 20);
        string memory receiveUln302Hex = Strings.toHexString(uint256(uint160(address(receiveUln302))), 20);
        string memory endpointV1Hex = Strings.toHexString(uint256(uint160(address(endpointV1))), 20);
        string memory endpointV2Hex = Strings.toHexString(uint256(uint160(address(endpointV2))), 20);
        string memory endpointV2ViewHex = Strings.toHexString(uint256(uint160(address(endpointV2View))), 20);
        string memory priceFeedHex = Strings.toHexString(uint256(uint160(address(priceFeed))), 20);
        string memory executorHex = Strings.toHexString(uint256(uint160(address(executor))), 20);
        string memory executorFeeLibHex = Strings.toHexString(uint256(uint160(address(executorFeeLib))), 20);
        string memory dvnHex = Strings.toHexString(uint256(uint160(address(dvn))), 20);
        string memory dvnFeeLibHex = Strings.toHexString(uint256(uint160(address(dvnFeeLib))), 20);

        // Step 2: Concatenate strings iteratively or in smaller groups
        string memory content = string(abi.encodePacked("Treasury=", treasuryHex, "\n"));
        content = string(abi.encodePacked(content, "SimpleMessageLib=", simpleMessageLibHex, "\n"));
        content = string(abi.encodePacked(content, "SendUln301=", sendUln301Hex, "\n"));
        content = string(abi.encodePacked(content, "SendUln302=", sendUln302Hex, "\n"));
        content = string(abi.encodePacked(content, "ReceiveUln301=", receiveUln301Hex, "\n"));
        content = string(abi.encodePacked(content, "ReceiveUln302=", receiveUln302Hex, "\n"));
        content = string(abi.encodePacked(content, "EndpointV1=", endpointV1Hex, "\n"));
        content = string(abi.encodePacked(content, "EndpointV2=", endpointV2Hex, "\n"));
        content = string(abi.encodePacked(content, "EndpointV2View=", endpointV2ViewHex, "\n"));
        content = string(abi.encodePacked(content, "PriceFeed=", priceFeedHex, "\n"));
        content = string(abi.encodePacked(content, "Executor=", executorHex, "\n"));
        content = string(abi.encodePacked(content, "ExecutorFeeLib=", executorFeeLibHex, "\n"));
        content = string(abi.encodePacked(content, "DVN=", dvnHex, "\n"));
        content = string(abi.encodePacked(content, "DVNFeeLib=", dvnFeeLibHex));

        vm.writeFile(FILE_NAME, content);
        console2.log("LZ Infra addresses written to ", FILE_NAME);
    }
}
