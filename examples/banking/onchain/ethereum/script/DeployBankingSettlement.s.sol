// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {BankingSettlement} from "../src/BankingSettlement.sol";

contract DeployBankingSettlement is Script {
    function run() external returns (BankingSettlement settlement) {
        address admin = vm.envAddress("BANKING_SETTLEMENT_ADMIN");
        address pauser = vm.envAddress("BANKING_SETTLEMENT_PAUSER");
        address operator = vm.envAddress("BANKING_SETTLEMENT_OPERATOR");
        address upgrader = vm.envAddress("BANKING_SETTLEMENT_UPGRADER");

        vm.startBroadcast();
        address proxy = Upgrades.deployUUPSProxy(
            "BankingSettlement.sol:BankingSettlement",
            abi.encodeCall(BankingSettlement.initialize, (admin, pauser, operator, upgrader))
        );
        vm.stopBroadcast();

        settlement = BankingSettlement(proxy);
    }
}
