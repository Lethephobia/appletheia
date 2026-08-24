// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {BankingSettlement, IERC20} from "../src/BankingSettlement.sol";

contract MockToken is IERC20 {
    mapping(address account => uint256 amount) public balanceOf;
    mapping(address owner => mapping(address spender => uint256 amount)) public allowance;

    function mint(address account, uint256 amount) external {
        balanceOf[account] += amount;
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        return true;
    }

    function transfer(address recipient, uint256 amount) external returns (bool) {
        _transfer(msg.sender, recipient, amount);
        return true;
    }

    function transferFrom(address sender, address recipient, uint256 amount) external returns (bool) {
        uint256 approved = allowance[sender][msg.sender];
        require(approved >= amount, "allowance");
        allowance[sender][msg.sender] = approved - amount;
        _transfer(sender, recipient, amount);
        return true;
    }

    function _transfer(address sender, address recipient, uint256 amount) private {
        require(balanceOf[sender] >= amount, "balance");
        balanceOf[sender] -= amount;
        balanceOf[recipient] += amount;
    }
}

contract BankingSettlementTest {
    BankingSettlement private settlement;
    MockToken private token;

    function setUp() public {
        settlement = new BankingSettlement(address(this));
        token = new MockToken();
        token.mint(address(this), 1_000);
        token.approve(address(settlement), 1_000);
    }

    function testDepositAndWithdrawalAreIdempotent() public {
        bytes16 depositId = bytes16(uint128(1));
        bytes16 withdrawalId = bytes16(uint128(2));

        settlement.settleDeposit(depositId, address(token), 500);
        require(settlement.processedDeposits(depositId), "deposit marker");
        require(token.balanceOf(address(settlement)) == 500, "deposit balance");

        (bool replayedDeposit,) = address(settlement).call(
            abi.encodeCall(settlement.settleDeposit, (depositId, address(token), 500))
        );
        require(!replayedDeposit, "deposit replay accepted");

        settlement.settleWithdrawal(withdrawalId, address(token), address(0xBEEF), 200);
        require(settlement.processedWithdrawals(withdrawalId), "withdrawal marker");
        require(token.balanceOf(address(0xBEEF)) == 200, "withdrawal balance");

        (bool replayedWithdrawal,) = address(settlement).call(
            abi.encodeCall(
                settlement.settleWithdrawal,
                (withdrawalId, address(token), address(0xBEEF), 200)
            )
        );
        require(!replayedWithdrawal, "withdrawal replay accepted");
    }
}
