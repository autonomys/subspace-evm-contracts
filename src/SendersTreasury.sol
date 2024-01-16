/**
 * Design:
 * 1. [OPTIONAL] The contract is deployed with some initial funds.
 * 2. This contract is used by the sender to deposit fund.
 * 3. The user requests a payment from the sender.
 * 4. The sender signs the payment (like a cheque) and sends it to the user.
 *    NOTE: This could be done via contract call or off-chain (sending a signed message over chat, email, API, etc.)
 *     Let's assume the sender sends the signed message to the user via a contract call.
 *     TODO: Later, we shift this logic to a more cloud based service i.e. offchain rather than onchain to avoid gas cost.
 * 5. The user claims the payment by submitting the signed payment or signature or request ID to the contract.
 *         TODO: We would have to make an API that fetches all the event logs in order to get the unfinished payment requests history.
 *
 *  Architecture diagram: https://github.com/subspace/subspace-evm-contracts/blob/add-receiver-pay-contract/img/sc-senders-treasury.png
 */
// SPDX-License-Identifier: MIT
pragma solidity 0.8.23;

contract SendersTreasury {
    /// ===== State Variables =====
    mapping(address => uint256) public balances;
    // CLEANUP: I don't think it's required, instead we have requestId in place.
    // mapping(address => uint256) senderNonces;

    // NOTE: It's required as would be enabled when cloud is integrated.
    // mapping(bytes32 => bool) signatures;

    enum PayRequestCode {
        UNSET, // NOT set
        REQUESTED, // requested payment by receiver
        SIGNED, // signed request by sender
        PAID // paid to the receiver

    }

    struct PayRequest {
        PayRequestCode statusCode; // 1: requested payment, 2: signed requested payment, 3: payment done
        address sender;
        address receiver;
        uint256 amount;
        bytes signature;
    }
    // TODO: remove later when "payment requesting/signing" is shifted to offchain

    mapping(uint256 => PayRequest) private payRequests;
    uint256 public requestPayId = 1;

    /* FIXME: See if we could shrink to 1 mapping. Reason: why to store an id into 2 places for sender & receiver.
    Any way, we have both info packed into single struct. */
    // TODO: Remove this later when getting the event logs has no discrepancies. Now, it sometimes fails.
    mapping(address => uint256[]) private receiversRequestIds;
    // TODO: Remove this later when getting the event logs has no discrepancies. Now, it sometimes fails.
    mapping(address => uint256[]) private sendersRequestIds;

    /// ===== Errors =====
    error InsufficientBalanceOf(address);
    error InvalidDepositAmount();
    error InvalidSignature();
    error InvalidRequestId(uint256);
    error CallerIsNotSender();
    error CallerIsNotReceiver();
    error ZeroAddress();
    error ZeroAmount();

    /// ===== Events =====
    // event Deposit(address indexed sender, uint256 amount);
    event PaymentRequested(
        address indexed receiver, uint256 indexed requestPayId, address indexed sender, uint256 amount
    );
    event PayRequestSigned(address indexed sender, uint256 indexed requestPayId);
    event PaymentDone(uint256 indexed requestPayId, address indexed sender, address indexed receiver, uint256 amount);

    /// ===== Setter functions =====
    /// @dev Sender deposit funds into the contract
    // TODO: verify this with `receive()`, `fallback()` in tests.
    // function deposit() external payable {
    //     if (msg.value == 0) {
    //         revert InvalidDepositAmount();
    //     }
    //     balances[msg.sender] += msg.value;
    //     emit Deposit(msg.sender, msg.value);
    // }

    /// @notice request payment from the sender
    /// @dev TODO: Move this to offchain later
    /// @param amount The amount of TSSC to request
    function requestPayment(address sender, uint256 amount) external {
        // TODO: enable this code when `deposit` is enabled i.e. when cloud handling is integrated.
        // check for sufficient balance of sender
        // uint256 senderBalance = getBalanceOf(msg.sender);
        // if (balances[sender] < amount) {
        //     revert InsufficientBalanceOf(sender);
        // }

        if (sender == address(0)) {
            revert ZeroAddress();
        }

        if (amount == 0) {
            revert ZeroAmount();
        }

        uint256 currentRequestPayId = requestPayId;
        payRequests[currentRequestPayId] = PayRequest({
            statusCode: PayRequestCode.REQUESTED,
            sender: sender,
            receiver: msg.sender,
            amount: amount,
            signature: ""
        });
        receiversRequestIds[msg.sender].push(currentRequestPayId);
        sendersRequestIds[sender].push(currentRequestPayId);
        ++requestPayId;

        emit PaymentRequested(msg.sender, currentRequestPayId, sender, amount);
    }

    /// @dev sign the pay request with ID that has been REQUESTED and neither already SIGNED or PAID.
    /// NOTE: Here, the sender also transfers the token corresponding to requested pay.
    /// As of now, in sender's screen, list the requests that are in REQUESTED status and are pending to sign.
    function signPayReq(uint256 requestId, bytes memory signature) external payable {
        // ensure requestId must have the status code as REQUESTED
        PayRequest storage pr = payRequests[requestId];
        if (requestId == 0 || pr.statusCode != PayRequestCode.REQUESTED) {
            revert InvalidRequestId(requestId);
        }

        // ensure that the caller is the sender
        if (pr.sender != msg.sender) {
            revert CallerIsNotSender();
        }

        // ensure the parsed signature belongs to the caller
        if (signature.length == 0 || verifySignature(signature, requestId, msg.sender)) {
            revert InvalidSignature();
        }

        // ensure the TSSC transferred is ≥ amount to requestId
        uint256 senderBalance = getBalanceOf(msg.sender);
        if (senderBalance + msg.value < pr.amount) {
            revert InsufficientBalanceOf(msg.sender);
        }
        balances[msg.sender] += msg.value;

        payRequests[requestId].statusCode = PayRequestCode.SIGNED;
        payRequests[requestId].signature = signature;

        emit PayRequestSigned(msg.sender, requestId);
    }

    /// @dev After having the sender's signature, the user can claim the payment.
    ///     NOTE: Here, signature arg is commented because it is automatically fetched from contract storage.
    /// TODO: Later on, signature would be retrieved from offchain storage. Implement this.
    /// As of now, in receiver's screen, list the requests that are in SIGNED status and are pending for claim.
    // function claimPayment(address sender, uint256 amount /* , bytes memory signature */ ) external {
    function claimPayment(uint256 requestId) external {
        PayRequest storage pr = payRequests[requestId];
        if (requestId == 0 || pr.statusCode != PayRequestCode.SIGNED) {
            revert InvalidRequestId(requestId);
        }

        uint256 senderBalance = getBalanceOf(pr.sender);
        uint256 amount = pr.amount;
        address sender = pr.sender;
        if (senderBalance < amount) {
            revert InsufficientBalanceOf(sender);
        }

        // ensure the caller is receiver
        if (pr.receiver != msg.sender) {
            revert CallerIsNotReceiver();
        }

        // CLEANUP: remove later
        // ensure the signature is valid [REDUNDANT as already done in `signPayReq`]
        if (verifySignature(pr.signature, requestId, sender)) {
            revert InvalidSignature();
        }

        // update the status of pay request id
        pr.statusCode = PayRequestCode.PAID;

        // Update balance and nonce
        balances[sender] = senderBalance - amount;

        (bool sent,) = payable(msg.sender).call{value: amount}("");
        if (!sent) {
            revert("claimPayment: Failed to send TSSC");
        }
        emit PaymentDone(requestId, sender, msg.sender, amount);
    }

    /// @notice sender can withdraw deposited fund at any time.
    function withdraw(uint256 amount) external {
        uint256 senderBalance = balances[msg.sender];

        if (senderBalance < amount) {
            revert InsufficientBalanceOf(msg.sender);
        }

        // update the sender's balance
        balances[msg.sender] = senderBalance - amount;

        (bool sent,) = payable(msg.sender).call{value: amount}("");
        if (!sent) {
            revert("Failed to withdraw TSSC");
        }
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

    function constructMessageOf(uint256 requestId) public view returns (bytes32 message) {
        PayRequest memory pr = payRequests[requestId];

        message = keccak256(
            abi.encodePacked(
                "\x19Auto Request Payments:\n32",
                keccak256(abi.encodePacked(pr.sender, pr.receiver, pr.amount, requestId, address(this)))
            )
        );
    }

    function verifySignature(bytes memory signature, uint256 requestId, address callerSender)
        public
        view
        returns (bool)
    {
        // construct message
        bytes32 message = constructMessageOf(requestId);

        // Recover the signer from the signature
        address recoveredAddress = recoverSigner(message, signature);

        // Verify that the recovered address is the same as the sender
        return recoveredAddress != callerSender;
    }

    /// ========== Getter Functions ==========
    /// @dev Get the current (latest, not the next available) nonce for an address (sender)
    // function getNonce(address addr) public view returns (uint256) {
    //     return nonces[addr];
    // }

    /// @dev Get the details of given payment request ID
    function getPaymentRequest(uint256 requestId)
        public
        view
        returns (PayRequestCode statusCode, address sender, address receiver, uint256 amount)
    {
        PayRequest memory payRequest = payRequests[requestId];
        return (payRequest.statusCode, payRequest.sender, payRequest.receiver, payRequest.amount);
    }

    /// @dev Get the payment request Ids of a sender
    function getSenderPaymentIdsOf(address sender) external view returns (uint256[] memory) {
        return sendersRequestIds[sender];
    }

    /// @dev Get the payment request Ids of a receiver
    function getReceiverPaymentIdsOf(address receiver) external view returns (uint256[] memory) {
        return receiversRequestIds[receiver];
    }

    function getBalanceOf(address sender) public view returns (uint256) {
        return balances[sender];
    }
}
