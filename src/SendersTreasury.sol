/**
 * Design:
 * 1. [OPTIONAL] The contract is deployed with some initial funds.
 * 2. This contract is used by the sender to deposit fund.
 * 3. The user requests a payment from the sender.
 * 4. The sender signs the payment (like a cheque) and sends it to the user.
 *    NOTE: This could be done via contract call or off-chain (sending a signed message over chat, email, API, etc.)
 *     Let's assume the sender sends the signed message to the user via a contract call.
 *     TODO: Later, we shift this logic to a more cloud based service i.e. offchain rather than onchain to avoid gas cost.
 * 5. The user claims the payment by submitting the signed payment or signature to the contract.
 *
 *  Architecture diagram: https://github.com/subspace/subspace-evm-contracts/blob/add-receiver-pay-contract/img/sc-senders-treasury.png
 */
// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

contract SendersTreasury {
    /// ===== State Variables =====
    mapping(address => uint256) public balances;
    mapping(address => uint256) nonces;

    /// ===== Errors =====
    error InsufficientBalance();
    error InvalidDepositAmount();
    error InvalidSignature();

    /// ===== Events =====
    event Deposit(address indexed sender, uint256 amount);
    event Payment(address indexed sender, address indexed receiver, uint256 amount);

    /// ===== Setter functions =====
    /// @dev Sender deposit funds into the contract
    // TODO: verify this with `receive()`, `fallback()` in tests.
    function deposit() external payable {
        if (msg.value == 0) {
            revert InvalidDepositAmount();
        }
        balances[msg.sender] += msg.value;
        emit Deposit(msg.sender, msg.value);
    }

    /// @dev After having the sender's signature, the user can claim the payment.
    function claimPayment(address sender, uint256 amount, bytes memory signature) external {
        uint256 senderBalance = balances[sender];
        if (senderBalance < amount) {
            revert InsufficientBalance();
        }

        // Construct the message
        bytes32 message = prefixed(keccak256(abi.encodePacked(sender, msg.sender, amount, nonces[sender], this)));

        // Recover the signer from the signature
        address recoveredAddress = recoverSigner(message, signature);

        // Verify that the recovered address is the same as the sender
        if (recoveredAddress != sender) {
            revert InvalidSignature();
        }

        // Update balance and nonce
        balances[sender] = senderBalance - amount;
        ++nonces[sender];

        (bool sent, bytes memory data) = payable(msg.sender).call{value: amount}("");
        if (!sent) {
            revert("claimPayment: Failed to send TSSC");
        }
        emit Payment(sender, msg.sender, amount);
    }

    // ========== Signature Methods ==========
    function splitSignature(bytes memory sig) private pure returns (uint8 v, bytes32 r, bytes32 s) {
        require(sig.length == 65);

        assembly {
            // solhint-disable-line no-inline-assembly
            // first 32 bytes, after the length prefix.
            r := mload(add(sig, 32))
            // second 32 bytes.
            s := mload(add(sig, 64))
            // final byte (first byte of the next 32 bytes).
            v := byte(0, mload(add(sig, 96)))
        }

        return (v, r, s);
    }

    function recoverSigner(bytes32 message, bytes memory sig) private pure returns (address) {
        (uint8 v, bytes32 r, bytes32 s) = splitSignature(sig);

        return ecrecover(message, v, r, s);
    }

    // builds a prefixed hash to mimic the behavior of eth_sign.
    function prefixed(bytes32 hash) private pure returns (bytes32) {
        return keccak256(abi.encodePacked("\x19Auto Request Payments:\n32", hash));
    }

    /// ========== Getter Functions ==========
    /// @dev Get the current (latest, not the next available) nonce for an address (sender)
    function getNonce(address addr) public view returns (uint256) {
        return nonces[addr];
    }
}
