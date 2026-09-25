// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {
    AccessControlDefaultAdminRules
} from "@openzeppelin/contracts/access/extensions/AccessControlDefaultAdminRules.sol";
import {ERC7821} from "@openzeppelin/contracts/account/extensions/draft-ERC7821.sol";
import {PaymasterSigner} from "@openzeppelin/contracts/account/paymaster/extensions/PaymasterSigner.sol";
import {EIP7702Utils} from "@openzeppelin/contracts/account/utils/EIP7702Utils.sol";
import {Execution} from "@openzeppelin/contracts/interfaces/draft-IERC7579.sol";
import {PackedUserOperation} from "@openzeppelin/contracts/interfaces/IERC4337.sol";
import {EIP712} from "@openzeppelin/contracts/utils/cryptography/EIP712.sol";
import {SignerECDSA} from "@openzeppelin/contracts/utils/cryptography/signers/SignerECDSA.sol";

import {BankingSettlement} from "./BankingSettlement.sol";

/// Sponsors only signature-authorized ERC-4337 deposits executed by the supported EIP-7702 account.
contract BankingSettlementPaymaster is PaymasterSigner, SignerECDSA, AccessControlDefaultAdminRules {
    uint48 public constant DEFAULT_ADMIN_DELAY = 2 days;
    bytes32 public constant FUND_MANAGER_ROLE = keccak256("FUND_MANAGER_ROLE");
    bytes32 public constant SIGNER_MANAGER_ROLE = keccak256("SIGNER_MANAGER_ROLE");

    bytes32 private constant ERC7821_BATCH_MODE = bytes32(uint256(1) << 248);

    address public immutable accountImplementation;
    address public immutable settlement;

    event SponsorshipSignerChanged(address indexed previousSigner, address indexed newSigner);

    error InvalidAddress();
    error InvalidSponsoredAccount();
    error InvalidSponsoredCall();

    constructor(
        address admin,
        address fundManager,
        address signerManager,
        address sponsorshipSigner,
        address accountImplementationAddress,
        address settlementAddress
    )
        EIP712("BankingSettlementPaymaster", "1")
        SignerECDSA(sponsorshipSigner)
        AccessControlDefaultAdminRules(DEFAULT_ADMIN_DELAY, admin)
    {
        if (
            admin == address(0) || fundManager == address(0) || signerManager == address(0)
                || sponsorshipSigner == address(0) || accountImplementationAddress == address(0)
                || settlementAddress == address(0)
        ) {
            revert InvalidAddress();
        }

        accountImplementation = accountImplementationAddress;
        settlement = settlementAddress;
        _grantRole(FUND_MANAGER_ROLE, fundManager);
        _grantRole(SIGNER_MANAGER_ROLE, signerManager);
    }

    function deposit() external payable {
        _deposit(msg.value);
    }

    function withdraw(address payable recipient, uint256 amount) external onlyRole(FUND_MANAGER_ROLE) {
        if (recipient == address(0)) revert InvalidAddress();
        _withdraw(recipient, amount);
    }

    function addStake(uint32 unstakeDelaySeconds) external payable onlyRole(FUND_MANAGER_ROLE) {
        _addStake(msg.value, unstakeDelaySeconds);
    }

    function unlockStake() external onlyRole(FUND_MANAGER_ROLE) {
        _unlockStake();
    }

    function withdrawStake(address payable recipient) external onlyRole(FUND_MANAGER_ROLE) {
        if (recipient == address(0)) revert InvalidAddress();
        _withdrawStake(recipient);
    }

    function setSponsorshipSigner(address newSigner) external onlyRole(SIGNER_MANAGER_ROLE) {
        if (newSigner == address(0)) revert InvalidAddress();
        address previousSigner = signer();
        _setSigner(newSigner);
        emit SponsorshipSignerChanged(previousSigner, newSigner);
    }

    function sponsorshipDigest(PackedUserOperation calldata userOp, uint48 validAfter, uint48 validUntil)
        external
        view
        returns (bytes32)
    {
        return _signableUserOpHash(userOp, validAfter, validUntil);
    }

    function _validatePaymasterUserOp(PackedUserOperation calldata userOp, bytes32 userOpHash, uint256 maxCost)
        internal
        override
        returns (bytes memory context, uint256 validationData)
    {
        if (EIP7702Utils.fetchDelegate(userOp.sender) != accountImplementation) {
            revert InvalidSponsoredAccount();
        }
        _validateSponsoredCall(userOp.callData);
        return super._validatePaymasterUserOp(userOp, userOpHash, maxCost);
    }

    function _validateSponsoredCall(bytes calldata accountCallData) private view {
        if (accountCallData.length < 4 || bytes4(accountCallData[:4]) != ERC7821.execute.selector) {
            revert InvalidSponsoredCall();
        }

        (bytes32 mode, bytes memory executionData) = abi.decode(accountCallData[4:], (bytes32, bytes));
        if (mode != ERC7821_BATCH_MODE) revert InvalidSponsoredCall();

        Execution[] memory executions = abi.decode(executionData, (Execution[]));
        if (
            executions.length != 1 || executions[0].target != settlement || executions[0].value != 0
                || executions[0].callData.length < 4
        ) {
            revert InvalidSponsoredCall();
        }
        bytes4 selector = bytes4(executions[0].callData);
        if (
            selector != BankingSettlement.settleDeposit.selector
                && selector != BankingSettlement.settleDepositWithPermit.selector
                && selector != BankingSettlement.settleDepositWithAuthorization.selector
        ) revert InvalidSponsoredCall();
    }
}
