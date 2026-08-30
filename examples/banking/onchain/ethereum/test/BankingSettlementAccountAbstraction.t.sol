// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {ERC7821} from "@openzeppelin/contracts/account/extensions/draft-ERC7821.sol";
import {Execution} from "@openzeppelin/contracts/interfaces/draft-IERC7579.sol";
import {PackedUserOperation} from "@openzeppelin/contracts/interfaces/IERC4337.sol";
import {Test} from "forge-std/Test.sol";

import {BankingSettlement} from "../src/BankingSettlement.sol";
import {BankingSettlementAccount} from "../src/BankingSettlementAccount.sol";
import {BankingSettlementPaymaster} from "../src/BankingSettlementPaymaster.sol";

contract BankingSettlementAccountAbstractionTest is Test {
    bytes32 private constant ERC7821_BATCH_MODE = bytes32(uint256(1) << 248);

    BankingSettlementAccount private accountImplementation;
    BankingSettlementPaymaster private paymaster;
    address private admin;
    address private fundManager;
    address private settlement;
    address private signerManager;
    address private sponsorshipSigner;
    uint256 private sponsorshipSignerPrivateKey;

    function setUp() public {
        admin = makeAddr("admin");
        fundManager = makeAddr("fundManager");
        settlement = makeAddr("settlement");
        signerManager = makeAddr("signerManager");
        (sponsorshipSigner, sponsorshipSignerPrivateKey) = makeAddrAndKey("sponsorshipSigner");
        accountImplementation = new BankingSettlementAccount();
        paymaster = new BankingSettlementPaymaster(
            admin, fundManager, signerManager, sponsorshipSigner, address(accountImplementation), settlement
        );
    }

    function test_AccountAllowsOnlyEntryPointOrSelfToExecute() public {
        Execution[] memory executions = new Execution[](0);
        bytes memory executionData = abi.encode(executions);

        vm.expectRevert();
        accountImplementation.execute(ERC7821_BATCH_MODE, executionData);
    }

    function test_DelegatedAccountValidatesTheTokenOwnerSignature() public {
        (address sender, uint256 senderPrivateKey) = makeAddrAndKey("sender");
        vm.etch(sender, abi.encodePacked(hex"ef0100", address(accountImplementation)));
        PackedUserOperation memory userOp = _depositUserOperation(sender, settlement);
        bytes32 userOpHash = keccak256("user-operation");
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(senderPrivateKey, userOpHash);
        userOp.signature = abi.encodePacked(r, s, v);

        vm.prank(address(accountImplementation.entryPoint()));
        uint256 validationData = BankingSettlementAccount(payable(sender)).validateUserOp(userOp, userOpHash, 0);

        assertEq(validationData, 0);
    }

    function test_SponsorshipSignerCanBeRotatedOnlyBySignerManager() public {
        address newSigner = makeAddr("newSigner");

        vm.expectRevert();
        paymaster.setSponsorshipSigner(newSigner);

        vm.prank(signerManager);
        paymaster.setSponsorshipSigner(newSigner);
        assertEq(paymaster.signer(), newSigner);
    }

    function test_ValidatesSignedDepositSponsorship() public {
        address sender = makeAddr("sender");
        vm.etch(sender, abi.encodePacked(hex"ef0100", address(accountImplementation)));
        PackedUserOperation memory userOp = _depositUserOperation(sender, settlement);
        bytes32 digest = paymaster.sponsorshipDigest(userOp, 0, 0);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(sponsorshipSignerPrivateKey, digest);
        userOp.paymasterAndData =
            abi.encodePacked(address(paymaster), uint128(100_000), uint128(50_000), bytes6(0), bytes6(0), r, s, v);
        assertEq(paymaster.sponsorshipDigest(userOp, 0, 0), digest);

        vm.prank(address(paymaster.entryPoint()));
        (, uint256 validationData) = paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        assertEq(validationData, 0);
    }

    function test_SponsorsEveryDepositSettlementMethod() public {
        _assertSponsoredDepositSelector(BankingSettlement.settleDeposit.selector);
        _assertSponsoredDepositSelector(BankingSettlement.settleDepositWithPermit.selector);
        _assertSponsoredDepositSelector(BankingSettlement.settleDepositWithAuthorization.selector);
    }

    function test_RejectsSponsorshipForAnotherTarget() public {
        address sender = makeAddr("sender");
        vm.etch(sender, abi.encodePacked(hex"ef0100", address(accountImplementation)));
        PackedUserOperation memory userOp = _depositUserOperation(sender, makeAddr("otherTarget"));

        vm.prank(address(paymaster.entryPoint()));
        vm.expectRevert(BankingSettlementPaymaster.InvalidSponsoredCall.selector);
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);
    }

    function test_RejectsSponsorshipForAnotherAccountImplementation() public {
        address sender = makeAddr("sender");
        vm.etch(sender, abi.encodePacked(hex"ef0100", makeAddr("otherImplementation")));
        PackedUserOperation memory userOp = _depositUserOperation(sender, settlement);

        vm.prank(address(paymaster.entryPoint()));
        vm.expectRevert(BankingSettlementPaymaster.InvalidSponsoredAccount.selector);
        paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);
    }

    function _depositUserOperation(address sender, address target) private returns (PackedUserOperation memory) {
        return _depositUserOperation(sender, target, BankingSettlement.settleDepositWithPermit.selector);
    }

    function _depositUserOperation(address sender, address target, bytes4 selector)
        private
        returns (PackedUserOperation memory)
    {
        Execution[] memory executions = new Execution[](1);
        executions[0] = Execution({target: target, value: 0, callData: _depositCallData(selector)});

        return PackedUserOperation({
            sender: sender,
            nonce: 0,
            initCode: bytes(""),
            callData: abi.encodeCall(ERC7821.execute, (ERC7821_BATCH_MODE, abi.encode(executions))),
            accountGasLimits: bytes32(0),
            preVerificationGas: 0,
            gasFees: bytes32(0),
            paymasterAndData: abi.encodePacked(address(paymaster), uint128(100_000), uint128(50_000)),
            signature: bytes("")
        });
    }

    function _depositCallData(bytes4 selector) private returns (bytes memory) {
        BankingSettlement.DepositSettlement memory depositSettlement =
            BankingSettlement.DepositSettlement({depositId: bytes16(uint128(1)), token: makeAddr("token"), amount: 1});
        BankingSettlement.OperatorSignature memory operatorSignature =
            BankingSettlement.OperatorSignature({deadline: 1, operator: makeAddr("operator"), signature: bytes("")});
        if (selector == BankingSettlement.settleDeposit.selector) {
            return abi.encodeCall(BankingSettlement.settleDeposit, (depositSettlement, operatorSignature));
        }
        if (selector == BankingSettlement.settleDepositWithAuthorization.selector) {
            return abi.encodeCall(
                BankingSettlement.settleDepositWithAuthorization,
                (
                    depositSettlement,
                    operatorSignature,
                    BankingSettlement.ERC3009ReceiveAuthorization({
                        validAfter: 0, validBefore: 1, nonce: bytes32(0), v: 27, r: bytes32(0), s: bytes32(0)
                    })
                )
            );
        }
        return abi.encodeCall(
            BankingSettlement.settleDepositWithPermit,
            (
                depositSettlement,
                operatorSignature,
                BankingSettlement.ERC2612Permit({deadline: 1, v: 27, r: bytes32(0), s: bytes32(0)})
            )
        );
    }

    function _assertSponsoredDepositSelector(bytes4 selector) private {
        address sender = makeAddr("selectorSender");
        vm.etch(sender, abi.encodePacked(hex"ef0100", address(accountImplementation)));
        PackedUserOperation memory userOp = _depositUserOperation(sender, settlement, selector);
        bytes32 digest = paymaster.sponsorshipDigest(userOp, 0, 0);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(sponsorshipSignerPrivateKey, digest);
        userOp.paymasterAndData =
            abi.encodePacked(address(paymaster), uint128(100_000), uint128(50_000), bytes6(0), bytes6(0), r, s, v);

        vm.prank(address(paymaster.entryPoint()));
        (, uint256 validationData) = paymaster.validatePaymasterUserOp(userOp, bytes32(0), 0);

        assertEq(validationData, 0);
    }
}
