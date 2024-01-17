// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import {Test, console2} from "forge-std/Test.sol";
import {SendersTreasury} from "../src/SendersTreasury.sol";

contract SendersTreasuryTest is Test {
    SendersTreasury public sendersTreasury;

    // Friends
    uint256 private aliceSKey = 123;
    address public alice = vm.addr(aliceSKey);
    uint256 private bobSKey = 456;
    address public bob = vm.addr(bobSKey);
    address public constant charlie = address(0x3);
    address public constant dan = address(0x4);
    address public constant eve = address(0x5);
    address public constant fredie = address(0x6);

    address public constant ZERO_ADDRESS = address(0);

    /// ===== Events =====
    // event Deposit(address indexed sender, uint256 amount);
    event PaymentRequested(
        address indexed receiver, uint256 indexed requestPayId, address indexed sender, uint256 amount
    );
    event PayRequestSigned(address indexed sender, uint256 indexed requestPayId);
    event PaymentDone(uint256 indexed requestPayId, address indexed sender, address indexed receiver, uint256 amount);

    function setUp() public {
        sendersTreasury = new SendersTreasury();

        // init balances
        deal(alice, 100);
        deal(bob, 100);
    }

    // ===== requestPayment =====

    function test_RequestPay() public {
        uint256 preBalCharlie = charlie.balance;

        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(sendersTreasury));
        emit PaymentRequested(charlie, 1, alice, 10);
        sendersTreasury.requestPayment(alice, 10);
        uint256 postBalCharlie = charlie.balance;

        assertEq(preBalCharlie, postBalCharlie, "pre & post balance of charlie must be same");
    }

    function test_RequestPayFailsFromZeroSender() public {
        vm.prank(charlie);
        vm.expectRevert(SendersTreasury.ZeroAddress.selector);
        sendersTreasury.requestPayment(ZERO_ADDRESS, 10);
    }

    function test_RequestPayFailsWhenZeroAmount() public {
        vm.prank(charlie);
        vm.expectRevert(SendersTreasury.ZeroAmount.selector);
        sendersTreasury.requestPayment(alice, 0);
    }

    // ===== signPayReq =====

    function test_SenderSignPayRequest() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectEmit(true, true, false, true, address(sendersTreasury));
        emit PayRequestSigned(alice, id);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
    }

    function test_SenderSignPayRequestFailWZeroReqId() public {
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(1);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", 0));
        sendersTreasury.signPayReq{value: 10}(0, abi.encodePacked(r, s, v));
    }

    function test_SenderSignPayRequestFailWInvalidReqId() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", id + 1));
        sendersTreasury.signPayReq{value: 10}(id + 1, abi.encodePacked(r, s, v));
    }

    function test_SenderSignPayRequestFailWZeroSig() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        vm.prank(alice);
        vm.expectRevert(SendersTreasury.InvalidSignature.selector);
        // parse a zero bytes signature
        sendersTreasury.signPayReq{value: 10}(id, "");
    }

    function test_SenderSignPayRequestFailWInvalidSig() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by bob
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(bobSKey, message);
        vm.prank(alice);
        vm.expectRevert(SendersTreasury.InvalidSignature.selector);
        // parse a zero bytes signature
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
    }

    function test_SenderSignPayRequestFailWhenWrongSender() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(bob);
        vm.expectRevert(SendersTreasury.CallerIsNotSender.selector);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
    }

    function test_SenderSignPayRequestFailWhenInsufficientBal() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectRevert(abi.encodeWithSignature("InsufficientBalanceOf(address)", alice));
        sendersTreasury.signPayReq{value: 2}(id, abi.encodePacked(r, s, v));
    }

    /// test sender fails to re-sign requested payment with transferring token
    function test_SenderSignPayRequestFailWhenReSigningWToken() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.startPrank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", id));
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
        vm.stopPrank();
    }

    /// test sender fails to re-sign requested payment without transferring token
    function test_SenderSignPayRequestFailWhenReSigningWoToken() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getRequestedPayIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.startPrank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", id));
        sendersTreasury.signPayReq(id, abi.encodePacked(r, s, v));
        vm.stopPrank();
    }

    // ===== claimPayment =====

    function test_ClaimPay() public {
        // pre balance of charlie
        uint256 charliePreBal = charlie.balance;
        // pre balance of alice
        uint256 alicePreBal = alice.balance;

        /* 1. request payment */
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        /* 2. sender sign requested payment.
            The 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `REQUESTED`.
        */
        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));

        /* 3. claim payment request 
            claims the 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `SIGNED`.
        */
        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(sendersTreasury));
        emit PaymentDone(id, alice, charlie, 10);
        sendersTreasury.claimPayment(id);

        // post balance of charlie
        uint256 charliePostBal = charlie.balance;
        assertEq(charliePostBal - charliePreBal, 10);

        // post balance of alice
        uint256 alicePostBal = alice.balance;
        assertEq(alicePreBal - alicePostBal, 10);
    }

    // test receiver fails to claim pay id when already claimed
    function test_ClaimPayIdFailsWhenAlreadyClaimed() public {
        /* 1. request payment */
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        /* 2. sender sign requested payment.
            The 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `REQUESTED`.
        */
        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));

        /* 3. claim payment request 
            claims the 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `SIGNED`.
        */
        vm.prank(charlie);
        vm.expectEmit(true, true, true, true, address(sendersTreasury));
        emit PaymentDone(id, alice, charlie, 10);
        sendersTreasury.claimPayment(id);

        /* 4. claim payment request again */
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", id));
        sendersTreasury.claimPayment(id);
    }

    function test_ClaimPayFailWZeroReqId() public {
        vm.prank(charlie);
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", 0));
        sendersTreasury.claimPayment(0);
    }

    function test_ClaimPayFailWInvalidReqId() public {
        /* 1. request payment */
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        /* 2. claim payment */
        vm.prank(charlie);
        vm.expectRevert(abi.encodeWithSignature("InvalidRequestId(uint256)", id));
        sendersTreasury.claimPayment(id);
    }

    function test_ClaimPayFailWhenInsufficientBal() public {
        /* 1. request payment */
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        /* 2. sender sign requested payment.
            The 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `REQUESTED`. */
        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.startPrank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));

        /* 3. sender withdraw some amount */
        sendersTreasury.withdraw(1);
        vm.stopPrank();

        /* 4. claim payment request 
            claims the 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `SIGNED`. */
        vm.prank(charlie);
        vm.expectRevert(abi.encodeWithSignature("InsufficientBalanceOf(address)", alice));
        sendersTreasury.claimPayment(id);
    }

    function test_ClaimPayFailWhenIncorrectReceiver() public {
        /* 1. request payment */
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        /* 2. sender sign requested payment.
            The 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `REQUESTED`. */
        // construct message
        bytes32 message = sendersTreasury.constructMessageOf(id);
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(r, s, v));

        /* 3. claim payment request 
            claims the 1st found requested payment from the list of ids to caller.
            Filter for id with status code as `SIGNED`. */
        vm.prank(dan);
        vm.expectRevert(SendersTreasury.CallerIsNotReceiver.selector);
        sendersTreasury.claimPayment(id);
    }
    // TODO: Add more tests
}
