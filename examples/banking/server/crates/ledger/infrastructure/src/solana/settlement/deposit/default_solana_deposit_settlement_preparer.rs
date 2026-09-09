use anchor_lang::{InstructionData, ToAccountMetas};
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    DepositSettlementPreparerError, SolanaDepositSettlementPreparation,
    SolanaDepositSettlementPrepareRequest, SolanaDepositSettlementPreparer,
    SolanaPreparedDepositTransaction,
};
use banking_ledger_domain::core::TokenDecimals;
use banking_settlement::{
    BankingSettlementConfig, DepositSettlementReceipt, PoolAuthority, accounts, instruction,
};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    hash::Hash,
    instruction::Instruction,
    message::{
        VersionedMessage,
        v1::{Message, TransactionConfig},
    },
    pubkey::Pubkey,
    signature::{NullSigner, Signer},
};
use solana_system_interface::program as system_program;
use solana_transaction::versioned::VersionedTransaction;
use spl_associated_token_account_interface::address as associated_token_address;
use spl_token_2022_interface::{extension::StateWithExtensions, state::Mint};

use super::DefaultSolanaDepositSettlementPreparerConfig;

pub struct DefaultSolanaDepositSettlementPreparer {
    rpc_client: RpcClient,
    config: DefaultSolanaDepositSettlementPreparerConfig,
}

impl DefaultSolanaDepositSettlementPreparer {
    pub fn new(
        rpc_client: RpcClient,
        config: DefaultSolanaDepositSettlementPreparerConfig,
    ) -> Self {
        Self { rpc_client, config }
    }

    fn create_transaction(
        &self,
        token_account_owner: Pubkey,
        instruction: Instruction,
        blockhash: Hash,
    ) -> Result<VersionedTransaction, DepositSettlementPreparerError> {
        let message = Message::try_compile_with_config(
            &self.config.payer.pubkey(),
            &[instruction],
            blockhash,
            TransactionConfig::empty()
                .with_compute_unit_limit(self.config.compute_unit_limit)
                .with_loaded_accounts_data_size_limit(self.config.loaded_accounts_data_size_limit),
        )
        .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let token_account_owner_signer = NullSigner::new(&token_account_owner);
        let mut signers: Vec<&dyn Signer> = vec![self.config.payer.as_ref()];
        if self.config.operator.pubkey() != self.config.payer.pubkey() {
            signers.push(self.config.operator.as_ref());
        }
        if token_account_owner != self.config.payer.pubkey()
            && token_account_owner != self.config.operator.pubkey()
        {
            signers.push(&token_account_owner_signer);
        }
        VersionedTransaction::try_new(VersionedMessage::V1(message), &signers)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))
    }

    fn receipt_address(&self, deposit_id: &[u8; 16]) -> Pubkey {
        Pubkey::find_program_address(
            &[DepositSettlementReceipt::SEED, deposit_id],
            &self.config.program_id,
        )
        .0
    }

    fn pool_authority_address(&self) -> Pubkey {
        Pubkey::find_program_address(&[PoolAuthority::SEED], &self.config.program_id).0
    }
}

impl SolanaDepositSettlementPreparer for DefaultSolanaDepositSettlementPreparer {
    async fn prepare(
        &self,
        request: SolanaDepositSettlementPrepareRequest,
    ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError> {
        let mint = Pubkey::new_from_array(*request.token_address().address().as_bytes());
        let token_account_owner =
            Pubkey::new_from_array(*request.token_account_owner_address().address().as_bytes());
        let mint_account = self
            .rpc_client
            .get_account(&mint)
            .await
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let token_program = mint_account.owner;
        let deposit_id = request.deposit_id().value().into_bytes();
        let authority = self.pool_authority_address();
        let mint_state = StateWithExtensions::<Mint>::unpack(&mint_account.data)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let domain_token_amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(mint_state.base.decimals),
            )
            .map_err(|_| DepositSettlementPreparerError::InvalidAmount)?;
        let solana_token_amount = u64::try_from(domain_token_amount.value())
            .map_err(|_| DepositSettlementPreparerError::InvalidAmount)?;
        let instruction = Instruction {
            program_id: self.config.program_id,
            accounts: accounts::DepositSettleInstructionAccounts {
                payer: self.config.payer.pubkey(),
                banking_settlement_config: Pubkey::find_program_address(
                    &[BankingSettlementConfig::SEED],
                    &self.config.program_id,
                )
                .0,
                operator: self.config.operator.pubkey(),
                deposit_settlement_receipt: self.receipt_address(&deposit_id),
                pool_authority: authority,
                mint,
                pool_token_account:
                    associated_token_address::get_associated_token_address_with_program_id(
                        &authority,
                        &mint,
                        &token_program,
                    ),
                token_account_owner,
                source_token_account:
                    associated_token_address::get_associated_token_address_with_program_id(
                        &token_account_owner,
                        &mint,
                        &token_program,
                    ),
                system_program: system_program::id(),
                token_program,
                associated_token_program: spl_associated_token_account_interface::program::id(),
            }
            .to_account_metas(None),
            data: instruction::SettleDeposit {
                deposit_id,
                token_amount: solana_token_amount,
            }
            .data(),
        };
        let blockhash = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;
        let transaction = self.create_transaction(token_account_owner, instruction, blockhash)?;
        let bytes = wincode::serialize(&transaction)
            .map_err(|error| DepositSettlementPreparerError::Backend(Box::new(error)))?;

        Ok(SolanaDepositSettlementPreparation {
            transaction: SolanaPreparedDepositTransaction::from_bytes(bytes),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use solana_sdk::{instruction::AccountMeta, signature::Keypair};

    use super::*;

    #[test]
    fn prepares_v1_transaction_with_configured_payer_and_token_account_owner_signature_slot() {
        for (payer_index, operator_index, token_account_owner_index) in
            [(0, 1, 2), (0, 0, 1), (0, 1, 0), (0, 1, 1), (0, 0, 0)]
        {
            let keypairs = [
                Arc::new(Keypair::new()),
                Arc::new(Keypair::new()),
                Arc::new(Keypair::new()),
            ];
            let payer = &keypairs[payer_index];
            let operator = &keypairs[operator_index];
            let token_account_owner = &keypairs[token_account_owner_index];
            let program_id = Pubkey::new_unique();
            let preparer = DefaultSolanaDepositSettlementPreparer::new(
                RpcClient::new("http://127.0.0.1:8899".to_owned()),
                DefaultSolanaDepositSettlementPreparerConfig {
                    program_id,
                    payer: Arc::clone(payer),
                    operator: Arc::clone(operator),
                    compute_unit_limit: 350_000,
                    loaded_accounts_data_size_limit: 32 * 1024 * 1024,
                },
            );
            let instruction = Instruction {
                program_id,
                accounts: vec![
                    AccountMeta::new(payer.pubkey(), true),
                    AccountMeta::new_readonly(operator.pubkey(), true),
                    AccountMeta::new_readonly(token_account_owner.pubkey(), true),
                ],
                data: vec![1, 2, 3],
            };
            let blockhash = Hash::new_unique();
            let transaction = preparer
                .create_transaction(token_account_owner.pubkey(), instruction, blockhash)
                .unwrap();
            let bytes = wincode::serialize(&transaction).unwrap();
            assert_eq!(bytes[0], 0x81);
            let mut decoded: VersionedTransaction = wincode::deserialize(&bytes).unwrap();
            decoded.sanitize().unwrap();
            let VersionedMessage::V1(message) = &decoded.message else {
                panic!("expected a V1 message");
            };
            assert_eq!(message.lifetime_specifier, blockhash);
            assert_eq!(message.fee_payer(), Some(&payer.pubkey()));
            assert_eq!(message.config.compute_unit_limit, Some(350_000));
            assert_eq!(
                message.config.loaded_accounts_data_size_limit,
                Some(32 * 1024 * 1024)
            );
            assert_eq!(message.instructions[0].data, vec![1, 2, 3]);
            let token_account_owner_signature_index = message
                .account_keys
                .iter()
                .position(|key| *key == token_account_owner.pubkey())
                .unwrap();
            let message_bytes = decoded.message.serialize();
            for signer in [payer, operator] {
                let signature_index = decoded
                    .message
                    .static_account_keys()
                    .iter()
                    .position(|key| *key == signer.pubkey())
                    .unwrap();
                assert!(
                    decoded.signatures[signature_index]
                        .verify(signer.pubkey().as_ref(), &message_bytes)
                );
            }
            if token_account_owner.pubkey() != payer.pubkey()
                && token_account_owner.pubkey() != operator.pubkey()
            {
                assert_eq!(
                    decoded.signatures[token_account_owner_signature_index],
                    Default::default()
                );
                decoded.signatures[token_account_owner_signature_index] = token_account_owner
                    .try_sign_message(&message_bytes)
                    .unwrap();
            }
            for (signature, key) in decoded
                .signatures
                .iter()
                .zip(decoded.message.static_account_keys())
            {
                assert!(signature.verify(key.as_ref(), &message_bytes));
            }
            let signed_bytes = wincode::serialize(&decoded).unwrap();
            let signed_transaction: VersionedTransaction =
                wincode::deserialize(&signed_bytes).unwrap();
            assert_eq!(signed_transaction, decoded);
        }
    }
}
