#!/usr/bin/env bash
set -euo pipefail

script_directory="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
project_directory="$(dirname "$script_directory")"
abi_directory="$project_directory/abi"

mkdir -p "$abi_directory"

generate_abi() {
  local contract_identifier="$1"
  local abi_name="$2"
  local abi_file="$abi_directory/$abi_name.json"
  local temporary_abi_file
  temporary_abi_file="$(mktemp "$abi_directory/.$abi_name.json.XXXXXX")"

  if ! forge inspect \
      --root "$project_directory" \
      --json \
      "$contract_identifier" \
      abi > "$temporary_abi_file"; then
    rm -f "$temporary_abi_file"
    return 1
  fi

  chmod 0644 "$temporary_abi_file"
  mv "$temporary_abi_file" "$abi_file"
}

generate_abi "src/BankingSettlement.sol:BankingSettlement" "BankingSettlement"
generate_abi "src/BankingSettlementAccount.sol:BankingSettlementAccount" "BankingSettlementAccount"
generate_abi "src/BankingSettlementPaymaster.sol:BankingSettlementPaymaster" "BankingSettlementPaymaster"
generate_abi "IEntryPoint" "IEntryPoint"
generate_abi "IERC20Metadata" "IERC20Metadata"
