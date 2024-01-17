// SPDX-License-Identifier: MIT
pragma solidity 0.8.19;

/// @title Fund contract
/// @notice This contract is used to transfer TSSC to single/multiple addresses
contract Fund {
    // Errors
    error InvalidReceiverAddress();
    error ZeroTSSC();
    error InsufficientFundsInContract();

    // Events
    event Transfer(address indexed from, address indexed to, uint256 amount);

    /// @notice Transfer TSSC to a single address
    function _transferTsscToSingle(address payable _to, uint256 _amount) private {
        // check for valid receiver i.e. non-zero receiver address
        // and also ensure sender & receiver are different
        if (msg.sender == _to || _to == address(0)) {
            revert InvalidReceiverAddress();
        }

        // check for non-zero msg.value
        if (_amount == 0) {
            revert ZeroTSSC();
        }

        // transfer amount
        (bool sent,) = _to.call{value: _amount}("");
        require(sent, "Failed to send Ether");

        emit Transfer(msg.sender, _to, _amount);
    }

    /// @notice Transfer TSSC to multiple addresses
    function transferTsscToMany(address[] memory _tos) external payable {
        uint256 amountPerAddress = msg.value / _tos.length;
        if (amountPerAddress == 0) {
            revert InsufficientFundsInContract();
        }

        // TODO: May be later on we can add a limit on number of addresses
        // to restrict the gas usage. For now, we are keeping it simple.

        for (uint256 i = 0; i < _tos.length; ++i) {
            _transferTsscToSingle(payable(_tos[i]), amountPerAddress);
        }
    }

    receive() external payable {}
}
