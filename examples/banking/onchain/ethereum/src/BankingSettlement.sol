// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

interface IERC20 {
    function transfer(address recipient, uint256 amount) external returns (bool);

    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool);
}

/// Coordinates replay-safe deposits and withdrawals of existing ERC-20 tokens.
contract BankingSettlement {
    error InvalidAddress();
    error InvalidAmount();
    error NotOperator();
    error SettlementAlreadyProcessed();
    error TokenTransferFailed();

    address public immutable operator;
    mapping(bytes16 depositId => bool processed) public processedDeposits;
    mapping(bytes16 withdrawalId => bool processed) public processedWithdrawals;

    event DepositSettled(
        bytes16 indexed depositId,
        address indexed token,
        address indexed tokenOwner,
        uint256 amount
    );
    event WithdrawalSettled(
        bytes16 indexed withdrawalId,
        address indexed token,
        address indexed tokenOwner,
        uint256 amount
    );

    constructor(address operator_) {
        if (operator_ == address(0)) revert InvalidAddress();
        operator = operator_;
    }

    /// Pulls an approved external token amount into the settlement pool.
    function settleDeposit(bytes16 depositId, address token, uint256 amount) external {
        if (token == address(0)) revert InvalidAddress();
        if (amount == 0) revert InvalidAmount();
        if (processedDeposits[depositId]) revert SettlementAlreadyProcessed();

        processedDeposits[depositId] = true;
        _safeTransferFrom(token, msg.sender, address(this), amount);
        emit DepositSettled(depositId, token, msg.sender, amount);
    }

    /// Pays an external token amount from the pool to the requested owner.
    function settleWithdrawal(
        bytes16 withdrawalId,
        address token,
        address tokenOwner,
        uint256 amount
    ) external {
        if (msg.sender != operator) revert NotOperator();
        if (token == address(0) || tokenOwner == address(0)) revert InvalidAddress();
        if (amount == 0) revert InvalidAmount();
        if (processedWithdrawals[withdrawalId]) revert SettlementAlreadyProcessed();

        processedWithdrawals[withdrawalId] = true;
        _safeTransfer(token, tokenOwner, amount);
        emit WithdrawalSettled(withdrawalId, token, tokenOwner, amount);
    }

    function _safeTransfer(address token, address recipient, uint256 amount) private {
        (bool success, bytes memory result) =
            token.call(abi.encodeCall(IERC20.transfer, (recipient, amount)));
        if (!success || (result.length != 0 && !abi.decode(result, (bool)))) {
            revert TokenTransferFailed();
        }
    }

    function _safeTransferFrom(address token, address sender, address recipient, uint256 amount)
        private
    {
        (bool success, bytes memory result) =
            token.call(abi.encodeCall(IERC20.transferFrom, (sender, recipient, amount)));
        if (!success || (result.length != 0 && !abi.decode(result, (bool)))) {
            revert TokenTransferFailed();
        }
    }
}
