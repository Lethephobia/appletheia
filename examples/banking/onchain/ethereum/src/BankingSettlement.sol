// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {
    AccessControlDefaultAdminRulesUpgradeable
} from "@openzeppelin/contracts-upgradeable/access/extensions/AccessControlDefaultAdminRulesUpgradeable.sol";
import {Initializable} from "@openzeppelin/contracts-upgradeable/proxy/utils/Initializable.sol";
import {UUPSUpgradeable} from "@openzeppelin/contracts-upgradeable/proxy/utils/UUPSUpgradeable.sol";
import {PausableUpgradeable} from "@openzeppelin/contracts-upgradeable/utils/PausableUpgradeable.sol";
import {EIP712Upgradeable} from "@openzeppelin/contracts-upgradeable/utils/cryptography/EIP712Upgradeable.sol";
import {IERC20} from "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import {IERC20Permit} from "@openzeppelin/contracts/token/ERC20/extensions/IERC20Permit.sol";
import {SafeERC20} from "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import {ReentrancyGuardTransient} from "@openzeppelin/contracts/utils/ReentrancyGuardTransient.sol";
import {SignatureChecker} from "@openzeppelin/contracts/utils/cryptography/SignatureChecker.sol";
import {IERC3009} from "@openzeppelin/contracts/interfaces/draft-IERC3009.sol";

/// Coordinates replay-safe deposits and withdrawals of existing ERC-20 tokens.
contract BankingSettlement is
    Initializable,
    AccessControlDefaultAdminRulesUpgradeable,
    EIP712Upgradeable,
    PausableUpgradeable,
    ReentrancyGuardTransient,
    UUPSUpgradeable
{
    using SafeERC20 for IERC20;

    /// @custom:storage-location erc7201:appletheia.storage.BankingSettlement
    struct BankingSettlementStorage {
        mapping(bytes16 depositId => bytes32 settlementHash) depositSettlementHashes;
        mapping(bytes16 withdrawalId => bytes32 settlementHash) withdrawalSettlementHashes;
    }

    struct DepositSettlement {
        bytes16 depositId;
        address token;
        uint256 amount;
    }

    struct OperatorSignature {
        uint256 deadline;
        address operator;
        bytes signature;
    }

    struct ERC2612Permit {
        uint256 deadline;
        uint8 v;
        bytes32 r;
        bytes32 s;
    }

    struct ERC3009ReceiveAuthorization {
        uint256 validAfter;
        uint256 validBefore;
        bytes32 nonce;
        uint8 v;
        bytes32 r;
        bytes32 s;
    }

    struct WithdrawalSettlement {
        bytes16 withdrawalId;
        address token;
        address tokenOwner;
        uint256 amount;
    }

    uint48 public constant DEFAULT_ADMIN_DELAY = 2 days;
    bytes32 public constant OPERATOR_ROLE = keccak256("OPERATOR_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant UPGRADER_ROLE = keccak256("UPGRADER_ROLE");
    bytes32 public constant DEPOSIT_OPERATOR_SIGNATURE_TYPEHASH = keccak256(
        "DepositOperatorSignature(bytes16 depositId,address token,uint256 amount,address tokenOwner,uint256 deadline,address operator)"
    );

    // keccak256(abi.encode(uint256(keccak256("appletheia.storage.BankingSettlement")) - 1)) & ~bytes32(uint256(0xff))
    bytes32 private constant BANKING_SETTLEMENT_STORAGE_LOCATION =
        0xd5257cb5c05ae81fa5e5d7c83619bbf5a06a1cfb1dee7b36b5dc82d99a7c1e00;

    event DepositSettled(bytes16 indexed depositId, address indexed token, address indexed tokenOwner, uint256 amount);
    event WithdrawalSettled(
        bytes16 indexed withdrawalId, address indexed token, address indexed tokenOwner, uint256 amount
    );

    error InvalidAddress();
    error InvalidAmount();
    error UnexpectedPoolBalanceChange(uint256 beforeBalance, uint256 afterBalance, uint256 expectedChange);

    error DepositOperatorSignatureExpired(uint256 deadline);
    error DepositSettlementConflict();
    error InvalidDepositOperatorSignature();

    error UnexpectedRecipientBalanceChange(uint256 beforeBalance, uint256 afterBalance, uint256 expectedChange);
    error WithdrawalSettlementConflict();

    /// @custom:oz-upgrades-unsafe-allow constructor
    constructor() {
        _disableInitializers();
    }

    function initialize(address admin, address pauser, address operator, address upgrader) external initializer {
        if (admin == address(0) || pauser == address(0) || operator == address(0) || upgrader == address(0)) {
            revert InvalidAddress();
        }

        __AccessControlDefaultAdminRules_init(DEFAULT_ADMIN_DELAY, admin);
        __EIP712_init("BankingSettlement", "1");
        __Pausable_init();

        _grantRole(OPERATOR_ROLE, operator);
        _grantRole(PAUSER_ROLE, pauser);
        _grantRole(UPGRADER_ROLE, upgrader);
        _setRoleAdmin(UPGRADER_ROLE, UPGRADER_ROLE);
    }

    function isDepositSettled(bytes16 depositId) external view returns (bool) {
        return _getStorage().depositSettlementHashes[depositId] != bytes32(0);
    }

    function depositSettlementHash(bytes16 depositId) external view returns (bytes32) {
        return _getStorage().depositSettlementHashes[depositId];
    }

    function depositOperatorSignatureDigest(
        DepositSettlement calldata depositSettlement,
        address tokenOwner,
        uint256 deadline,
        address operator
    ) public view returns (bytes32) {
        return _hashTypedDataV4(
            keccak256(
                abi.encode(
                    DEPOSIT_OPERATOR_SIGNATURE_TYPEHASH,
                    depositSettlement.depositId,
                    depositSettlement.token,
                    depositSettlement.amount,
                    tokenOwner,
                    deadline,
                    operator
                )
            )
        );
    }

    function isWithdrawalSettled(bytes16 withdrawalId) external view returns (bool) {
        return _getStorage().withdrawalSettlementHashes[withdrawalId] != bytes32(0);
    }

    function withdrawalSettlementHash(bytes16 withdrawalId) external view returns (bytes32) {
        return _getStorage().withdrawalSettlementHashes[withdrawalId];
    }

    /// Pulls an approved external token amount into the settlement pool.
    function settleDeposit(DepositSettlement calldata depositSettlement, OperatorSignature calldata operatorSignature)
        external
        whenNotPaused
        nonReentrant
    {
        address tokenOwner = msg.sender;
        _validateDepositParameters(depositSettlement.token, tokenOwner, depositSettlement.amount);
        _verifyDepositOperatorSignature(depositSettlement, tokenOwner, operatorSignature);
        bytes32 settlementHash = _settlementHash(depositSettlement.token, tokenOwner, depositSettlement.amount);
        if (_isDepositAlreadySettled(depositSettlement.depositId, settlementHash)) return;
        _recordDepositSettlement(depositSettlement.depositId, settlementHash);
        _transferDeposit(depositSettlement.token, tokenOwner, depositSettlement.amount);
        emit DepositSettled(depositSettlement.depositId, depositSettlement.token, tokenOwner, depositSettlement.amount);
    }

    /// Permits and pulls an external token amount into the settlement pool.
    function settleDepositWithPermit(
        DepositSettlement calldata depositSettlement,
        OperatorSignature calldata operatorSignature,
        ERC2612Permit calldata permit
    ) external whenNotPaused nonReentrant {
        address tokenOwner = msg.sender;
        _validateDepositParameters(depositSettlement.token, tokenOwner, depositSettlement.amount);
        _verifyDepositOperatorSignature(depositSettlement, tokenOwner, operatorSignature);
        bytes32 settlementHash = _settlementHash(depositSettlement.token, tokenOwner, depositSettlement.amount);
        if (_isDepositAlreadySettled(depositSettlement.depositId, settlementHash)) return;
        _recordDepositSettlement(depositSettlement.depositId, settlementHash);
        _transferDepositWithPermit(depositSettlement.token, tokenOwner, depositSettlement.amount, permit);
        emit DepositSettled(depositSettlement.depositId, depositSettlement.token, tokenOwner, depositSettlement.amount);
    }

    /// Receives an ERC-3009-authorized external token amount into the settlement pool.
    function settleDepositWithAuthorization(
        DepositSettlement calldata depositSettlement,
        OperatorSignature calldata operatorSignature,
        ERC3009ReceiveAuthorization calldata authorization
    ) external whenNotPaused nonReentrant {
        address tokenOwner = msg.sender;
        _validateDepositParameters(depositSettlement.token, tokenOwner, depositSettlement.amount);
        _verifyDepositOperatorSignature(depositSettlement, tokenOwner, operatorSignature);
        bytes32 settlementHash = _settlementHash(depositSettlement.token, tokenOwner, depositSettlement.amount);
        if (_isDepositAlreadySettled(depositSettlement.depositId, settlementHash)) return;
        _recordDepositSettlement(depositSettlement.depositId, settlementHash);
        _transferDepositWithAuthorization(depositSettlement.token, tokenOwner, depositSettlement.amount, authorization);
        emit DepositSettled(depositSettlement.depositId, depositSettlement.token, tokenOwner, depositSettlement.amount);
    }

    /// Pays an external token amount from the pool to the requested owner.
    function settleWithdrawal(WithdrawalSettlement calldata withdrawalSettlement)
        external
        onlyRole(OPERATOR_ROLE)
        whenNotPaused
        nonReentrant
    {
        _validateWithdrawalParameters(
            withdrawalSettlement.token, withdrawalSettlement.tokenOwner, withdrawalSettlement.amount
        );
        bytes32 settlementHash =
            _settlementHash(withdrawalSettlement.token, withdrawalSettlement.tokenOwner, withdrawalSettlement.amount);
        if (_isWithdrawalAlreadySettled(withdrawalSettlement.withdrawalId, settlementHash)) return;
        _recordWithdrawalSettlement(withdrawalSettlement.withdrawalId, settlementHash);
        _transferWithdrawal(withdrawalSettlement.token, withdrawalSettlement.tokenOwner, withdrawalSettlement.amount);
        emit WithdrawalSettled(
            withdrawalSettlement.withdrawalId,
            withdrawalSettlement.token,
            withdrawalSettlement.tokenOwner,
            withdrawalSettlement.amount
        );
    }

    function pause() external onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() external onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    function _validateDepositParameters(address token, address tokenOwner, uint256 amount) private view {
        if (token == address(0) || tokenOwner == address(this)) {
            revert InvalidAddress();
        }
        if (amount == 0) revert InvalidAmount();
    }

    function _verifyDepositOperatorSignature(
        DepositSettlement calldata depositSettlement,
        address tokenOwner,
        OperatorSignature calldata operatorSignature
    ) private view {
        // forge-lint: disable-next-line(block-timestamp)
        if (block.timestamp > operatorSignature.deadline) {
            revert DepositOperatorSignatureExpired(operatorSignature.deadline);
        }
        bytes32 operatorSignatureDigest = depositOperatorSignatureDigest(
            depositSettlement, tokenOwner, operatorSignature.deadline, operatorSignature.operator
        );
        if (
            !hasRole(OPERATOR_ROLE, operatorSignature.operator)
                || !SignatureChecker.isValidSignatureNowCalldata(
                    operatorSignature.operator, operatorSignatureDigest, operatorSignature.signature
                )
        ) {
            revert InvalidDepositOperatorSignature();
        }
    }

    function _isDepositAlreadySettled(bytes16 depositId, bytes32 settlementHash)
        private
        view
        returns (bool alreadySettled)
    {
        bytes32 recordedSettlementHash = _getStorage().depositSettlementHashes[depositId];
        if (recordedSettlementHash != bytes32(0)) {
            if (recordedSettlementHash != settlementHash) revert DepositSettlementConflict();
            return true;
        }

        return false;
    }

    function _recordDepositSettlement(bytes16 depositId, bytes32 settlementHash) private {
        _getStorage().depositSettlementHashes[depositId] = settlementHash;
    }

    function _transferDeposit(address token, address tokenOwner, uint256 amount) private {
        IERC20 settlementToken = IERC20(token);
        uint256 poolBalanceBefore = settlementToken.balanceOf(address(this));
        settlementToken.safeTransferFrom(tokenOwner, address(this), amount);
        uint256 poolBalanceAfter = settlementToken.balanceOf(address(this));
        if (poolBalanceAfter < poolBalanceBefore || poolBalanceAfter - poolBalanceBefore != amount) {
            revert UnexpectedPoolBalanceChange(poolBalanceBefore, poolBalanceAfter, amount);
        }
    }

    function _transferDepositWithPermit(
        address token,
        address tokenOwner,
        uint256 amount,
        ERC2612Permit calldata permit
    ) private {
        // A submitted permit can be front-run without changing its allowance target. In that
        // case the nonce has already advanced, so rely on transferFrom to verify the allowance.
        try IERC20Permit(token)
            .permit(tokenOwner, address(this), amount, permit.deadline, permit.v, permit.r, permit.s) {}
            catch {}
        _transferDeposit(token, tokenOwner, amount);
    }

    function _transferDepositWithAuthorization(
        address token,
        address tokenOwner,
        uint256 amount,
        ERC3009ReceiveAuthorization calldata authorization
    ) private {
        IERC20 settlementToken = IERC20(token);
        uint256 poolBalanceBefore = settlementToken.balanceOf(address(this));
        IERC3009(token)
            .receiveWithAuthorization(
                tokenOwner,
                address(this),
                amount,
                authorization.validAfter,
                authorization.validBefore,
                authorization.nonce,
                authorization.v,
                authorization.r,
                authorization.s
            );
        uint256 poolBalanceAfter = settlementToken.balanceOf(address(this));
        if (poolBalanceAfter < poolBalanceBefore || poolBalanceAfter - poolBalanceBefore != amount) {
            revert UnexpectedPoolBalanceChange(poolBalanceBefore, poolBalanceAfter, amount);
        }
    }

    function _validateWithdrawalParameters(address token, address tokenOwner, uint256 amount) private view {
        if (token == address(0) || tokenOwner == address(0) || tokenOwner == address(this)) {
            revert InvalidAddress();
        }
        if (amount == 0) revert InvalidAmount();
    }

    function _isWithdrawalAlreadySettled(bytes16 withdrawalId, bytes32 settlementHash)
        private
        view
        returns (bool alreadySettled)
    {
        bytes32 recordedSettlementHash = _getStorage().withdrawalSettlementHashes[withdrawalId];
        if (recordedSettlementHash != bytes32(0)) {
            if (recordedSettlementHash != settlementHash) revert WithdrawalSettlementConflict();
            return true;
        }

        return false;
    }

    function _recordWithdrawalSettlement(bytes16 withdrawalId, bytes32 settlementHash) private {
        _getStorage().withdrawalSettlementHashes[withdrawalId] = settlementHash;
    }

    function _transferWithdrawal(address token, address tokenOwner, uint256 amount) private {
        IERC20 settlementToken = IERC20(token);
        uint256 poolBalanceBefore = settlementToken.balanceOf(address(this));
        uint256 recipientBalanceBefore = settlementToken.balanceOf(tokenOwner);
        settlementToken.safeTransfer(tokenOwner, amount);
        uint256 poolBalanceAfter = settlementToken.balanceOf(address(this));
        uint256 recipientBalanceAfter = settlementToken.balanceOf(tokenOwner);
        if (poolBalanceAfter > poolBalanceBefore || poolBalanceBefore - poolBalanceAfter != amount) {
            revert UnexpectedPoolBalanceChange(poolBalanceBefore, poolBalanceAfter, amount);
        }
        if (recipientBalanceAfter < recipientBalanceBefore || recipientBalanceAfter - recipientBalanceBefore != amount)
        {
            revert UnexpectedRecipientBalanceChange(recipientBalanceBefore, recipientBalanceAfter, amount);
        }
    }

    function _settlementHash(address token, address tokenOwner, uint256 amount) private pure returns (bytes32) {
        return keccak256(abi.encode(token, tokenOwner, amount));
    }

    function _authorizeUpgrade(address) internal override onlyRole(UPGRADER_ROLE) {}

    function _getStorage() private pure returns (BankingSettlementStorage storage $) {
        assembly {
            $.slot := BANKING_SETTLEMENT_STORAGE_LOCATION
        }
    }
}
