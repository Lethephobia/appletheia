// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {Script} from "forge-std/Script.sol";

import {BankingSettlementAccount} from "../src/BankingSettlementAccount.sol";
import {BankingSettlementPaymaster} from "../src/BankingSettlementPaymaster.sol";

contract DeployBankingSettlementAccountAbstraction is Script {
    function run()
        external
        returns (BankingSettlementAccount accountImplementation, BankingSettlementPaymaster paymaster)
    {
        address admin = vm.envAddress("BANKING_PAYMASTER_ADMIN");
        address fundManager = vm.envAddress("BANKING_PAYMASTER_FUND_MANAGER");
        address signerManager = vm.envAddress("BANKING_PAYMASTER_SIGNER_MANAGER");
        address sponsorshipSigner = vm.envAddress("BANKING_PAYMASTER_SPONSORSHIP_SIGNER");
        address settlement = vm.envAddress("BANKING_SETTLEMENT_PROXY");

        vm.startBroadcast();
        accountImplementation = new BankingSettlementAccount();
        paymaster = new BankingSettlementPaymaster(
            admin, fundManager, signerManager, sponsorshipSigner, address(accountImplementation), settlement
        );
        vm.stopBroadcast();
    }
}
