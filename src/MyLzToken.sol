// SPDX-License-Identifier: MIT
pragma solidity ^0.8.22;

import "LayerZero-v2/oapp/contracts/oft/OFT.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// TODO: Replace OZ's Ownable with that of Solmate.
// TODO: Replace OZ's ERC20 with that of Solmate.
contract MyToken is OFT {
    constructor(
        string memory _name, // token name
        string memory _symbol, // token symbol
        address _layerZeroEndpoint, // local endpoint address
        address _owner // token owner used as a delegate in LayerZero Endpoint
    ) OFT(_name, _symbol, _layerZeroEndpoint, _owner) Ownable(_owner) {
        // your contract logic here
        _mint(msg.sender, 1_000_000 ether); // mints 1M tokens to the deployer
    }
}
