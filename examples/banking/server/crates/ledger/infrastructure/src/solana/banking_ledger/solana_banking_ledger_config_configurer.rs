use anchor_lang::{InstructionData, ToAccountMetas};
use banking_ledger::{
    BankingLedgerConfig, accounts::BankingLedgerConfigConfigureInstructionAccounts,
    instruction::ConfigureBankingLedgerConfig,
};
use banking_ledger_application::{
    BankingLedgerConfigConfigurer, BankingLedgerConfigConfigurerError,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_loader_v3_interface::get_program_data_address;
use solana_sdk::{instruction::Instruction, pubkey::Pubkey, signature::Signer};
use solana_system_interface::program as system_program;
use solana_transaction::Transaction;

use super::{SolanaBankingLedgerConfigConfigurerConfig, SolanaBankingLedgerConfigConfigurerError};

/// Solana implementation of `BankingLedgerConfigConfigurer`.
pub struct SolanaBankingLedgerConfigConfigurer {
    rpc_client: RpcClient,
    config: SolanaBankingLedgerConfigConfigurerConfig,
}

impl SolanaBankingLedgerConfigConfigurer {
    pub fn new(rpc_client: RpcClient, config: SolanaBankingLedgerConfigConfigurerConfig) -> Self {
        Self { rpc_client, config }
    }

    fn banking_ledger_config_address(program_id: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(&[BankingLedgerConfig::SEED], program_id).0
    }

    async fn send_transaction(
        &self,
        instructions: Vec<Instruction>,
        signers: Vec<&dyn Signer>,
    ) -> Result<(), SolanaBankingLedgerConfigConfigurerError> {
        let blockhash = self.rpc_client.get_latest_blockhash().await?;
        let transaction = {
            let payer = self.config.payer().as_ref();
            let mut transaction = Transaction::new_with_payer(&instructions, Some(&payer.pubkey()));
            transaction.try_sign(&signers, blockhash)?;
            transaction
        };

        self.rpc_client
            .send_and_confirm_transaction(&transaction)
            .await?;

        Ok(())
    }
}

impl BankingLedgerConfigConfigurer for SolanaBankingLedgerConfigConfigurer {
    async fn configure(&self) -> Result<(), BankingLedgerConfigConfigurerError> {
        let program_id = *self.config.program_id();
        let banking_ledger_config_address = Self::banking_ledger_config_address(&program_id);
        let payer = self.config.payer().as_ref();
        let operator = *self.config.operator();
        let upgrade_authority = self.config.upgrade_authority().as_ref();
        let program_data_address = get_program_data_address(&program_id);
        let instruction = Instruction {
            program_id,
            accounts: BankingLedgerConfigConfigureInstructionAccounts {
                payer: payer.pubkey(),
                operator,
                banking_ledger_config: banking_ledger_config_address,
                program: program_id,
                program_data: program_data_address,
                upgrade_authority: upgrade_authority.pubkey(),
                system_program: system_program::id(),
            }
            .to_account_metas(None),
            data: ConfigureBankingLedgerConfig.data(),
        };
        let signers = unique_signers(&[payer, upgrade_authority]);

        self.send_transaction(vec![instruction], signers)
            .await
            .map_err(|error| BankingLedgerConfigConfigurerError::Backend(Box::new(error)))?;

        Ok(())
    }
}

fn unique_signers<'a>(signers: &[&'a dyn Signer]) -> Vec<&'a dyn Signer> {
    let mut unique: Vec<&dyn Signer> = Vec::new();
    for signer in signers {
        if unique
            .iter()
            .all(|existing| existing.pubkey() != signer.pubkey())
        {
            unique.push(*signer);
        }
    }

    unique
}
