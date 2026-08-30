use alloy::primitives::{Address, B256, Bytes, FixedBytes, Signature, U256};
use alloy::providers::{DynProvider, Provider};
use alloy::signers::SignerSync;
use appletheia::domain::AggregateId;
use banking_ledger_application::{
    EthereumDepositSettlementTransactionPreparation,
    EthereumDepositSettlementTransactionPrepareRequest,
    EthereumDepositSettlementTransactionPreparer,
    EthereumDepositSettlementTransactionPreparerError, EvmCallData, EvmDepositAuthorization,
    EvmTransactionRequest,
};
use banking_ledger_domain::core::{EvmChainId, TokenDecimals};
use chrono::Utc;

use super::{
    DefaultEthereumDepositSettlementTransactionPreparerConfig,
    DefaultEthereumDepositSettlementTransactionPreparerError,
};
use crate::ethereum::contract::{BankingSettlement, IERC20Metadata};

pub struct DefaultEthereumDepositSettlementTransactionPreparer {
    chain_provider: DynProvider,
    config: DefaultEthereumDepositSettlementTransactionPreparerConfig,
}

impl DefaultEthereumDepositSettlementTransactionPreparer {
    pub fn new(
        chain_provider: DynProvider,
        config: DefaultEthereumDepositSettlementTransactionPreparerConfig,
    ) -> Self {
        Self {
            chain_provider,
            config,
        }
    }

    fn signature_parts(signature: &Signature) -> (u8, B256, B256) {
        (
            signature.v_byte(),
            B256::from(signature.r().to_be_bytes::<32>()),
            B256::from(signature.s().to_be_bytes::<32>()),
        )
    }
}

impl EthereumDepositSettlementTransactionPreparer
    for DefaultEthereumDepositSettlementTransactionPreparer
{
    async fn prepare(
        &self,
        request: EthereumDepositSettlementTransactionPrepareRequest,
    ) -> Result<
        EthereumDepositSettlementTransactionPreparation,
        EthereumDepositSettlementTransactionPreparerError,
    > {
        let chain_id = self.chain_provider.get_chain_id().await.map_err(|error| {
            EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                DefaultEthereumDepositSettlementTransactionPreparerError::Rpc(error),
            ))
        })?;
        let settlement_address = Address::from(*self.config.settlement_contract.as_bytes());
        let token_address = Address::from(*request.token_address().address().as_bytes());
        let token_owner_address =
            Address::from(*request.token_owner_address().address().as_bytes());
        let deposit_id = FixedBytes::<16>::from(request.deposit_id().value().into_bytes());
        let token_decimals = IERC20Metadata::new(token_address, &self.chain_provider)
            .decimals()
            .call()
            .await
            .map_err(|error| {
                EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                    DefaultEthereumDepositSettlementTransactionPreparerError::Contract(error),
                ))
            })?;
        let token_amount = request
            .amount()
            .to_token_amount(
                request.currency_decimals(),
                TokenDecimals::new(token_decimals),
            )
            .map_err(|_| EthereumDepositSettlementTransactionPreparerError::InvalidAmount)?;
        let amount = U256::from(token_amount.value());
        let current_date_time = Utc::now();
        let operator_signature_expires_at = current_date_time
            .checked_add_signed(self.config.operator_signature_ttl)
            .ok_or_else(|| {
                EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                    DefaultEthereumDepositSettlementTransactionPreparerError::DeadlineOverflow,
                ))
            })?;
        if operator_signature_expires_at <= current_date_time {
            return Err(EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                DefaultEthereumDepositSettlementTransactionPreparerError::InvalidOperatorSignatureTtl,
            )));
        }
        let operator_signature_deadline = u64::try_from(operator_signature_expires_at.timestamp())
            .map_err(|_| {
                EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                    DefaultEthereumDepositSettlementTransactionPreparerError::DeadlineBeforeUnixEpoch,
                ))
            })?;
        let operator_signature_deadline_value = U256::from(operator_signature_deadline);
        let operator = self.config.operator.address();
        let settlement = BankingSettlement::new(settlement_address, &self.chain_provider);
        let deposit_settlement = BankingSettlement::DepositSettlement {
            depositId: deposit_id,
            token: token_address,
            amount,
        };
        let operator_signature_digest = settlement
            .depositOperatorSignatureDigest(
                deposit_settlement.clone(),
                token_owner_address,
                operator_signature_deadline_value,
                operator,
            )
            .call()
            .await
            .map_err(|error| {
                EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                    DefaultEthereumDepositSettlementTransactionPreparerError::Contract(error),
                ))
            })?;
        let signed_operator_signature = self
            .config
            .operator
            .sign_hash_sync(&operator_signature_digest)
            .map_err(|error| {
                EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                    DefaultEthereumDepositSettlementTransactionPreparerError::Signer(error),
                ))
            })?;
        let operator_signature = BankingSettlement::OperatorSignature {
            deadline: operator_signature_deadline_value,
            operator,
            signature: Bytes::copy_from_slice(&signed_operator_signature.as_bytes()),
        };
        let call_data = match request.authorization() {
            None => settlement
                .settleDeposit(deposit_settlement, operator_signature)
                .calldata()
                .clone(),
            Some(EvmDepositAuthorization::Erc2612(permit)) => {
                let signature =
                    Signature::from_raw_array(permit.signature().as_bytes()).map_err(|error| {
                        EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                            DefaultEthereumDepositSettlementTransactionPreparerError::InvalidSignature(error),
                        ))
                    })?;
                let (v, r, s) = Self::signature_parts(&signature);
                settlement
                    .settleDepositWithPermit(
                        deposit_settlement,
                        operator_signature,
                        BankingSettlement::ERC2612Permit {
                            deadline: U256::from(permit.deadline().value()),
                            v,
                            r,
                            s,
                        },
                    )
                    .calldata()
                    .clone()
            }
            Some(EvmDepositAuthorization::Erc3009(authorization)) => {
                let signature = Signature::from_raw_array(authorization.signature().as_bytes())
                    .map_err(|error| {
                        EthereumDepositSettlementTransactionPreparerError::Backend(Box::new(
                            DefaultEthereumDepositSettlementTransactionPreparerError::InvalidSignature(error),
                        ))
                    })?;
                let (v, r, s) = Self::signature_parts(&signature);
                settlement
                    .settleDepositWithAuthorization(
                        deposit_settlement,
                        operator_signature,
                        BankingSettlement::ERC3009ReceiveAuthorization {
                            validAfter: U256::from(authorization.valid_after().value()),
                            validBefore: U256::from(authorization.valid_before().value()),
                            nonce: B256::from(*authorization.nonce().as_bytes()),
                            v,
                            r,
                            s,
                        },
                    )
                    .calldata()
                    .clone()
            }
        };

        Ok(EthereumDepositSettlementTransactionPreparation::new(
            EvmTransactionRequest::new(
                EvmChainId::new(chain_id),
                *request.token_owner_address().address(),
                self.config.settlement_contract,
                EvmCallData::from_bytes(call_data.to_vec()),
            ),
        ))
    }
}
