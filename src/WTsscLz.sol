// SPDX-License-Identifier: MIT
pragma solidity 0.8.24;

import {OFT} from "@layerzerolabs/oapp/contracts/oft/OFT.sol";
import {SafeTransferLib} from "solmate/utils/SafeTransferLib.sol";
import {Ownable} from "@openzeppelin/contracts/access/Ownable.sol";

// NOTE: Replace OZ's ERC20, Ownable with that of Solmate. Both are already used in parent contract.
//      Lot of work required to refactor parent abstracts. Will do later, not the priority now to
//      reduce contract size.
contract WTsscLz is OFT {
    using SafeTransferLib for address;

    event Deposit(address indexed from, uint256 amount);
    event Withdrawal(address indexed to, uint256 amount);

    constructor(
        string memory _name, // token name
        string memory _symbol, // token symbol
        address _layerZeroEndpoint, // local endpoint address
        address _owner // token owner used as a delegate in LayerZero Endpoint
    ) OFT(_name, _symbol, _layerZeroEndpoint, _owner) Ownable() {}

    function deposit() public payable virtual {
        _mint(msg.sender, msg.value);

        emit Deposit(msg.sender, msg.value);
    }

    // TODO: Make deposit & send together so that tx sent via single signature

    function withdraw(uint256 amount) public virtual {
        _burn(msg.sender, amount);

        emit Withdrawal(msg.sender, amount);

        msg.sender.safeTransferETH(amount);
    }

    receive() external payable virtual {
        deposit();
    }
}