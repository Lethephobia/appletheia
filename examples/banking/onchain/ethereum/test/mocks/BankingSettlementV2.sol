// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {BankingSettlement} from "../../src/BankingSettlement.sol";

/// @custom:oz-upgrades-from src/BankingSettlement.sol:BankingSettlement
/// @custom:oz-upgrades-unsafe-allow missing-initializer
contract BankingSettlementV2 is BankingSettlement {
    function version() external pure returns (uint256) {
        return 2;
    }
}
