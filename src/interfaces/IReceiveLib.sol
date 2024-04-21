// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {IReceiveUlnE2} from "@layerzerolabs/messagelib/contracts/uln/interfaces/IReceiveUlnE2.sol";
import {UlnConfig} from "@layerzerolabs/messagelib/contracts/uln/UlnBase.sol";

interface IReceiveLib is IReceiveUlnE2 {
    /// @dev assuming most oapps use default, we get default as memory and custom as storage to save gas
    function getUlnConfig(address _oapp, uint32 _remoteEid) external view returns (UlnConfig memory rtnConfig);

    function verifiable(
        UlnConfig memory _config,
        bytes32 _headerHash,
        bytes32 _payloadHash
    ) external view returns (bool);
}
