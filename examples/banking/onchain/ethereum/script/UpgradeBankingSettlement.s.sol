// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";
import {Upgrades} from "openzeppelin-foundry-upgrades/Upgrades.sol";
import {BankingSettlement} from "../src/BankingSettlement.sol";

contract UpgradeBankingSettlement is Script {
    function run() external returns (BankingSettlement settlement) {
        address proxy = vm.envAddress("BANKING_SETTLEMENT_PROXY");
        string memory implementationContract = vm.envString("BANKING_SETTLEMENT_IMPLEMENTATION_CONTRACT");

        vm.startBroadcast();
        Upgrades.upgradeProxy(proxy, implementationContract, bytes(""));
        vm.stopBroadcast();

        settlement = BankingSettlement(proxy);
    }
}
