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
    address public constant charlie = address(0x789);
    address public constant dan = address(0x789);
    address public constant eve = address(0x789);
    address public constant fredie = address(0x789);

    address public constant ZERO_ADDRESS = address(0);

    /// ===== Events =====
    event Deposit(address indexed sender, uint256 amount);
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

    function test_RequestPayFailsFromZeroAddress() public {
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
        uint256[] memory aliceIds = sendersTreasury.getSenderPaymentIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = keccak256(
            abi.encodePacked(
                "\x19Auto Request Payments:\n32",
                keccak256(abi.encodePacked(alice, charlie, uint256(10), aliceIds[0], address(sendersTreasury)))
            )
        );
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectEmit(true, true, false, true, address(sendersTreasury));
        emit PayRequestSigned(alice, id);
        sendersTreasury.signPayReq{value: 10}(id, abi.encodePacked(v, r, s));
    }

    function test_SenderSignPayRequestFailWZeroReqId() public {
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        // get the list of request ids
        uint256[] memory aliceIds = sendersTreasury.getSenderPaymentIdsOf(alice);
        assertEq(aliceIds.length, 1, "alice must have got 1 pay request by now");

        // construct message
        bytes32 message = keccak256(
            abi.encodePacked(
                "\x19Auto Request Payments:\n32",
                keccak256(abi.encodePacked(alice, charlie, uint256(10), aliceIds[0], address(sendersTreasury)))
            )
        );
        // construct the signature by alice
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(aliceSKey, message);
        vm.prank(alice);
        vm.expectRevert(SendersTreasury.ZeroRequestId.selector);
        sendersTreasury.signPayReq{value: 10}(0, abi.encodePacked(v, r, s));
    }

    function test_SenderSignPayRequestFailWZeroSig() public {
        uint256 id = sendersTreasury.requestPayId();
        vm.prank(charlie);
        sendersTreasury.requestPayment(alice, 10);

        vm.prank(alice);
        vm.expectRevert(SendersTreasury.ZeroSignature.selector);
        // parse a zero bytes signature
        sendersTreasury.signPayReq{value: 10}(id, "");
    }

    function test_SenderSignPayRequestFailWWrongReqId() public {}
    function test_SenderSignPayRequestFailWhenWrongSender() public {}
    function test_SenderSignPayRequestFailWhenInsufficientBal() public {}

    // ===== claimPayment =====

    // TODO: Add more tests
}
