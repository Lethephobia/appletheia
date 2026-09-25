// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {IAccessControl} from "@openzeppelin/contracts/access/IAccessControl.sol";
import {Initializable} from "@openzeppelin/contracts/proxy/utils/Initializable.sol";
import {ERC20} from "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import {ERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/ERC20Permit.sol";
import {ECDSA} from "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";
import {Pausable} from "@openzeppelin/contracts/utils/Pausable.sol";
import {Test} from "forge-std/Test.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {BankingSettlement} from "../src/BankingSettlement.sol";
import {BankingSettlementV2} from "./mocks/BankingSettlementV2.sol";

contract MockToken is ERC20, ERC20Permit {
    bytes32 private constant RECEIVE_WITH_AUTHORIZATION_TYPEHASH = keccak256(
        "ReceiveWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)"
    );

    uint256 public transferFee;
    mapping(address authorizer => mapping(bytes32 nonce => bool used)) public authorizationState;

    constructor() ERC20("Mock Token", "MOCK") ERC20Permit("Mock Token") {}

    function mint(address account, uint256 amount) external {
        _mint(account, amount);
    }

    function setTransferFee(uint256 fee) external {
        transferFee = fee;
    }

    function receiveWithAuthorization(
        address from,
        address to,
        uint256 value,
        uint256 validAfter,
        uint256 validBefore,
        bytes32 nonce,
        uint8 v,
        bytes32 r,
        bytes32 s
    ) external {
        require(msg.sender == to, "caller is not payee");
        // forge-lint: disable-next-line(block-timestamp)
        require(block.timestamp > validAfter, "authorization is not yet valid");
        // forge-lint: disable-next-line(block-timestamp)
        require(block.timestamp < validBefore, "authorization is expired");
        require(!authorizationState[from][nonce], "authorization is used");
        bytes32 structHash =
            keccak256(abi.encode(RECEIVE_WITH_AUTHORIZATION_TYPEHASH, from, to, value, validAfter, validBefore, nonce));
        require(ECDSA.recover(_hashTypedDataV4(structHash), v, r, s) == from, "invalid authorization");
        authorizationState[from][nonce] = true;
        _transfer(from, to, value);
    }

    function _update(address sender, address recipient, uint256 amount) internal override {
        uint256 fee = transferFee;
        if (sender == address(0) || recipient == address(0) || fee == 0) {
            super._update(sender, recipient, amount);
            return;
        }

        require(amount >= fee, "fee");
        super._update(sender, recipient, amount - fee);
        super._update(sender, address(0), fee);
    }
}

contract BankingSettlementTest is Test {
    uint256 private constant DEPOSIT_OPERATOR_SIGNATURE_DEADLINE = type(uint256).max;
    bytes32 private constant DEPOSIT_OPERATOR_SIGNATURE_TYPEHASH = keccak256(
        "DepositOperatorSignature(bytes16 depositId,address token,uint256 amount,address tokenOwner,uint256 deadline,address operator)"
    );
    bytes32 private constant EIP712_DOMAIN_TYPEHASH =
        keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)");
    bytes32 private constant PERMIT_TYPEHASH =
        keccak256("Permit(address owner,address spender,uint256 value,uint256 nonce,uint256 deadline)");
    bytes32 private constant RECEIVE_WITH_AUTHORIZATION_TYPEHASH = keccak256(
        "ReceiveWithAuthorization(address from,address to,uint256 value,uint256 validAfter,uint256 validBefore,bytes32 nonce)"
    );

    event DepositSettled(bytes16 indexed depositId, address indexed token, address indexed tokenOwner, uint256 amount);
    event WithdrawalSettled(
        bytes16 indexed withdrawalId, address indexed token, address indexed tokenOwner, uint256 amount
    );

    BankingSettlement private settlement;
    MockToken private token;
    address private admin;
    address private depositor;
    uint256 private depositorPrivateKey;
    address private operator;
    uint256 private operatorPrivateKey;
    address private pauser;
    address private tokenOwner;
    address private upgrader;

    function setUp() public {
        admin = makeAddr("admin");
        (depositor, depositorPrivateKey) = makeAddrAndKey("depositor");
        (operator, operatorPrivateKey) = makeAddrAndKey("operator");
        pauser = makeAddr("pauser");
        tokenOwner = makeAddr("tokenOwner");
        upgrader = makeAddr("upgrader");
        address proxy = Upgrades.deployUUPSProxy(
            "BankingSettlement.sol:BankingSettlement",
            abi.encodeCall(BankingSettlement.initialize, (admin, pauser, operator, upgrader))
        );
        settlement = BankingSettlement(proxy);
        token = new MockToken();
        token.mint(depositor, 1_000);
    }

    function depositOperatorSignature(
        bytes16 depositId,
        address tokenOwnerAddress,
        uint256 amount,
        uint256 deadline,
        address signingOperator,
        uint256 signingOperatorPrivateKey
    ) private view returns (bytes memory) {
        bytes32 digest = settlement.depositOperatorSignatureDigest(
            depositSettlement(depositId, amount), tokenOwnerAddress, deadline, signingOperator
        );
        bytes32 domainSeparator = keccak256(
            abi.encode(
                EIP712_DOMAIN_TYPEHASH,
                keccak256("BankingSettlement"),
                keccak256("1"),
                block.chainid,
                address(settlement)
            )
        );
        bytes32 expectedStructHash = keccak256(
            abi.encode(
                DEPOSIT_OPERATOR_SIGNATURE_TYPEHASH,
                depositId,
                address(token),
                amount,
                tokenOwnerAddress,
                deadline,
                signingOperator
            )
        );
        assertEq(digest, keccak256(abi.encodePacked("\x19\x01", domainSeparator, expectedStructHash)));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(signingOperatorPrivateKey, digest);
        return abi.encodePacked(r, s, v);
    }

    function depositSettlement(bytes16 depositId, uint256 amount)
        private
        view
        returns (BankingSettlement.DepositSettlement memory)
    {
        return BankingSettlement.DepositSettlement({depositId: depositId, token: address(token), amount: amount});
    }

    function operatorSignatureData(uint256 deadline, address signingOperator, bytes memory signature)
        private
        pure
        returns (BankingSettlement.OperatorSignature memory)
    {
        return
            BankingSettlement.OperatorSignature({deadline: deadline, operator: signingOperator, signature: signature});
    }

    function withdrawalSettlement(bytes16 withdrawalId, address tokenOwnerAddress, uint256 amount)
        private
        view
        returns (BankingSettlement.WithdrawalSettlement memory)
    {
        return BankingSettlement.WithdrawalSettlement({
            withdrawalId: withdrawalId, token: address(token), tokenOwner: tokenOwnerAddress, amount: amount
        });
    }

    function settleDeposit(bytes16 depositId, address tokenOwnerAddress, uint256 amount) private {
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, tokenOwnerAddress, amount, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        BankingSettlement.ERC2612Permit memory permit =
            depositPermit(tokenOwnerAddress, depositorPrivateKey, amount, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE);
        vm.prank(tokenOwnerAddress);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, amount),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            permit
        );
    }

    function settleDepositSignedBy(
        bytes16 depositId,
        address signingOperator,
        uint256 signingOperatorKey,
        bool expectInvalidSignature
    ) private {
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, signingOperator, signingOperatorKey
        );
        BankingSettlement.ERC2612Permit memory permit =
            depositPermit(depositor, depositorPrivateKey, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE);
        if (expectInvalidSignature) {
            vm.expectRevert(BankingSettlement.InvalidDepositOperatorSignature.selector);
        }
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, signingOperator, operatorSignature),
            permit
        );
    }

    function depositPermit(address owner, uint256 ownerPrivateKey, uint256 amount, uint256 deadline)
        private
        view
        returns (BankingSettlement.ERC2612Permit memory)
    {
        bytes32 structHash = keccak256(
            abi.encode(PERMIT_TYPEHASH, owner, address(settlement), amount, token.nonces(owner), deadline)
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPrivateKey, digest);
        return BankingSettlement.ERC2612Permit({deadline: deadline, v: v, r: r, s: s});
    }

    function invalidPermit() private pure returns (BankingSettlement.ERC2612Permit memory) {
        return BankingSettlement.ERC2612Permit({deadline: type(uint256).max, v: 0, r: bytes32(0), s: bytes32(0)});
    }

    function receiveAuthorization(address owner, uint256 ownerPrivateKey, uint256 amount, bytes32 nonce)
        private
        view
        returns (BankingSettlement.ERC3009ReceiveAuthorization memory)
    {
        uint256 validAfter = 0;
        uint256 validBefore = type(uint256).max;
        bytes32 structHash = keccak256(
            abi.encode(
                RECEIVE_WITH_AUTHORIZATION_TYPEHASH, owner, address(settlement), amount, validAfter, validBefore, nonce
            )
        );
        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", token.DOMAIN_SEPARATOR(), structHash));
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(ownerPrivateKey, digest);
        return BankingSettlement.ERC3009ReceiveAuthorization({
            validAfter: validAfter, validBefore: validBefore, nonce: nonce, v: v, r: r, s: s
        });
    }

    function test_ImplementationCannotBeInitialized() public {
        address implementation = Upgrades.getImplementationAddress(address(settlement));

        vm.expectRevert(Initializable.InvalidInitialization.selector);
        BankingSettlement(implementation).initialize(admin, pauser, operator, upgrader);
    }

    function test_ProxyCannotBeInitializedTwice() public {
        vm.expectRevert(Initializable.InvalidInitialization.selector);
        settlement.initialize(admin, pauser, operator, upgrader);
    }

    function test_OnlyUpgraderCanUpgrade() public {
        address unauthorizedUpgrader = makeAddr("unauthorizedUpgrader");
        BankingSettlementV2 newImplementation = new BankingSettlementV2();

        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector,
                unauthorizedUpgrader,
                settlement.UPGRADER_ROLE()
            )
        );
        vm.prank(unauthorizedUpgrader);
        settlement.upgradeToAndCall(address(newImplementation), bytes(""));
    }

    function test_DefaultAdminCannotGrantUpgradeAuthority() public {
        address newUpgrader = makeAddr("newUpgrader");
        bytes32 upgraderRole = settlement.UPGRADER_ROLE();

        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, admin, upgraderRole)
        );
        vm.prank(admin);
        settlement.grantRole(upgraderRole, newUpgrader);
    }

    function test_UpgradePreservesSettlementStateAndBalance() public {
        bytes16 depositId = bytes16(uint128(1));
        settleDeposit(depositId, depositor, 500);
        address previousImplementation = Upgrades.getImplementationAddress(address(settlement));

        Upgrades.upgradeProxy(
            address(settlement), "test/mocks/BankingSettlementV2.sol:BankingSettlementV2", bytes(""), upgrader
        );

        BankingSettlementV2 upgradedSettlement = BankingSettlementV2(address(settlement));
        assertNotEq(Upgrades.getImplementationAddress(address(settlement)), previousImplementation);
        assertEq(upgradedSettlement.version(), 2);
        assertTrue(upgradedSettlement.isDepositSettled(depositId));
        assertEq(token.balanceOf(address(upgradedSettlement)), 500);
    }

    function test_DepositAndWithdrawalAreIdempotent() public {
        bytes16 depositId = bytes16(uint128(1));
        bytes16 withdrawalId = bytes16(uint128(2));

        vm.expectEmit(true, true, true, true, address(settlement));
        emit DepositSettled(depositId, address(token), depositor, 500);
        settleDeposit(depositId, depositor, 500);
        assertTrue(settlement.isDepositSettled(depositId));
        assertEq(settlement.depositSettlementHash(depositId), keccak256(abi.encode(address(token), depositor, 500)));
        assertEq(token.balanceOf(address(settlement)), 500);

        settleDeposit(depositId, depositor, 500);
        assertEq(token.balanceOf(address(settlement)), 500);

        bytes memory conflictingOperatorSignature = depositOperatorSignature(
            depositId, depositor, 499, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        vm.expectRevert(BankingSettlement.DepositSettlementConflict.selector);
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 499),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, conflictingOperatorSignature),
            invalidPermit()
        );

        vm.expectEmit(true, true, true, true, address(settlement));
        emit WithdrawalSettled(withdrawalId, address(token), tokenOwner, 200);
        vm.prank(operator);
        settlement.settleWithdrawal(withdrawalSettlement(withdrawalId, tokenOwner, 200));
        assertTrue(settlement.isWithdrawalSettled(withdrawalId));
        assertEq(
            settlement.withdrawalSettlementHash(withdrawalId), keccak256(abi.encode(address(token), tokenOwner, 200))
        );
        assertEq(token.balanceOf(tokenOwner), 200);

        vm.prank(operator);
        settlement.settleWithdrawal(withdrawalSettlement(withdrawalId, tokenOwner, 200));
        assertEq(token.balanceOf(tokenOwner), 200);

        vm.expectRevert(BankingSettlement.WithdrawalSettlementConflict.selector);
        vm.prank(operator);
        settlement.settleWithdrawal(withdrawalSettlement(withdrawalId, tokenOwner, 199));
    }

    function test_OnlyOperatorCanSignDepositsAndSettleWithdrawals() public {
        bytes32 operatorRole = settlement.OPERATOR_ROLE();
        (address newOperator, uint256 newOperatorPrivateKey) = makeAddrAndKey("newOperator");
        bytes16 depositId = bytes16(uint128(1));

        settleDepositSignedBy(depositId, newOperator, newOperatorPrivateKey, true);

        vm.expectRevert(
            abi.encodeWithSelector(IAccessControl.AccessControlUnauthorizedAccount.selector, newOperator, operatorRole)
        );
        vm.prank(newOperator);
        settlement.settleWithdrawal(withdrawalSettlement(bytes16(uint128(2)), tokenOwner, 200));

        vm.prank(admin);
        settlement.grantRole(operatorRole, newOperator);
        settleDepositSignedBy(depositId, newOperator, newOperatorPrivateKey, false);
        vm.prank(newOperator);
        settlement.settleWithdrawal(withdrawalSettlement(bytes16(uint128(2)), tokenOwner, 200));
        assertEq(token.balanceOf(tokenOwner), 200);
    }

    function test_DepositRejectsAnExpiredOperatorSignature() public {
        bytes16 depositId = bytes16(uint128(1));
        uint256 deadline = block.timestamp - 1;
        bytes memory operatorSignature =
            depositOperatorSignature(depositId, depositor, 500, deadline, operator, operatorPrivateKey);

        vm.expectRevert(abi.encodeWithSelector(BankingSettlement.DepositOperatorSignatureExpired.selector, deadline));
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(deadline, operator, operatorSignature),
            invalidPermit()
        );
    }

    function test_DepositRejectsAnInvalidOperatorSignature() public {
        bytes16 depositId = bytes16(uint128(1));
        (, uint256 unauthorizedSignerPrivateKey) = makeAddrAndKey("unauthorizedSigner");
        bytes memory invalidOperatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, unauthorizedSignerPrivateKey
        );

        vm.expectRevert(BankingSettlement.InvalidDepositOperatorSignature.selector);
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, invalidOperatorSignature),
            invalidPermit()
        );
    }

    function test_DepositRequiresAnErc2612Permit() public {
        bytes16 depositId = bytes16(uint128(1));
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );

        vm.expectRevert();
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            invalidPermit()
        );
    }

    function test_DepositContinuesWhenThePermitWasSubmittedFirst() public {
        bytes16 depositId = bytes16(uint128(1));
        BankingSettlement.ERC2612Permit memory permit =
            depositPermit(depositor, depositorPrivateKey, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE);
        token.permit(depositor, address(settlement), 500, permit.deadline, permit.v, permit.r, permit.s);
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );

        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            permit
        );

        assertEq(token.balanceOf(address(settlement)), 500);
    }

    function test_DepositUsesAnExistingAllowance() public {
        bytes16 depositId = bytes16(uint128(1));
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        vm.prank(depositor);
        token.approve(address(settlement), 500);

        vm.prank(depositor);
        settlement.settleDeposit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature)
        );

        assertEq(token.balanceOf(address(settlement)), 500);
        assertEq(token.allowance(depositor, address(settlement)), 0);
    }

    function test_DepositUsesAnErc3009ReceiveAuthorization() public {
        bytes16 depositId = bytes16(uint128(1));
        bytes32 nonce = keccak256("deposit-authorization");
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        BankingSettlement.ERC3009ReceiveAuthorization memory authorization =
            receiveAuthorization(depositor, depositorPrivateKey, 500, nonce);

        vm.prank(depositor);
        settlement.settleDepositWithAuthorization(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            authorization
        );

        assertTrue(token.authorizationState(depositor, nonce));
        assertEq(token.balanceOf(address(settlement)), 500);
        assertEq(token.allowance(depositor, address(settlement)), 0);

        vm.prank(depositor);
        settlement.settleDepositWithAuthorization(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            authorization
        );
        assertEq(token.balanceOf(address(settlement)), 500);
    }

    function test_PauserCanSuspendAndResumeSettlement() public {
        vm.expectRevert(
            abi.encodeWithSelector(
                IAccessControl.AccessControlUnauthorizedAccount.selector, admin, settlement.PAUSER_ROLE()
            )
        );
        vm.prank(admin);
        settlement.pause();

        vm.prank(pauser);
        settlement.pause();
        bytes16 depositId = bytes16(uint128(1));
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        vm.expectRevert(Pausable.EnforcedPause.selector);
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            invalidPermit()
        );

        vm.prank(pauser);
        settlement.unpause();
        settleDeposit(depositId, depositor, 500);
        assertEq(token.balanceOf(address(settlement)), 500);
    }

    function test_DepositRejectsAnUnexpectedReceivedAmount() public {
        bytes16 depositId = bytes16(uint128(1));
        token.setTransferFee(1);
        bytes memory operatorSignature = depositOperatorSignature(
            depositId, depositor, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorPrivateKey
        );
        BankingSettlement.ERC2612Permit memory permit =
            depositPermit(depositor, depositorPrivateKey, 500, DEPOSIT_OPERATOR_SIGNATURE_DEADLINE);

        vm.expectRevert(abi.encodeWithSelector(BankingSettlement.UnexpectedPoolBalanceChange.selector, 0, 499, 500));
        vm.prank(depositor);
        settlement.settleDepositWithPermit(
            depositSettlement(depositId, 500),
            operatorSignatureData(DEPOSIT_OPERATOR_SIGNATURE_DEADLINE, operator, operatorSignature),
            permit
        );

        assertFalse(settlement.isDepositSettled(depositId));
        assertEq(token.balanceOf(address(settlement)), 0);
    }

    function test_WithdrawalRejectsAnUnexpectedReceivedAmount() public {
        bytes16 withdrawalId = bytes16(uint128(2));
        settleDeposit(bytes16(uint128(1)), depositor, 500);
        token.setTransferFee(1);

        vm.expectRevert(
            abi.encodeWithSelector(BankingSettlement.UnexpectedRecipientBalanceChange.selector, 0, 199, 200)
        );
        vm.prank(operator);
        settlement.settleWithdrawal(withdrawalSettlement(withdrawalId, tokenOwner, 200));

        assertFalse(settlement.isWithdrawalSettled(withdrawalId));
        assertEq(token.balanceOf(address(settlement)), 500);
        assertEq(token.balanceOf(tokenOwner), 0);
    }

    function testFuzz_DepositAndWithdrawalPreserveExactBalances(uint128 rawAmount, uint128 rawWithdrawalAmount) public {
        uint256 amount = bound(uint256(rawAmount), 1, 1_000);
        uint256 withdrawalAmount = bound(uint256(rawWithdrawalAmount), 1, amount);
        settleDeposit(bytes16(uint128(1)), depositor, amount);
        vm.prank(operator);
        settlement.settleWithdrawal(withdrawalSettlement(bytes16(uint128(2)), tokenOwner, withdrawalAmount));

        assertEq(token.balanceOf(address(settlement)), amount - withdrawalAmount);
        assertEq(token.balanceOf(tokenOwner), withdrawalAmount);
    }
}
