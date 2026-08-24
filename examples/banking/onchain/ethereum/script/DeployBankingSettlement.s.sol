// SPDX-License-Identifier: MIT
pragma solidity ^0.8.30;

import {BankingSettlement} from "../src/BankingSettlement.sol";

interface Vm {
    function envAddress(string calldata name) external returns (address value);
    function startBroadcast() external;
    function stopBroadcast() external;
}

contract DeployBankingSettlement {
    Vm private constant VM = Vm(address(uint160(uint256(keccak256("hevm cheat code")))));

    function run() external returns (BankingSettlement settlement) {
        address operator = VM.envAddress("BANKING_SETTLEMENT_OPERATOR");
        VM.startBroadcast();
        settlement = new BankingSettlement(operator);
        VM.stopBroadcast();
    }
}
