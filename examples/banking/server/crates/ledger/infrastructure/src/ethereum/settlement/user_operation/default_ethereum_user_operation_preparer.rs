use alloy::primitives::{Address, B256, Bytes, U256, Uint};
use alloy::providers::{DynProvider, Provider};
use alloy::rpc::types::PackedUserOperation;
use alloy::signers::SignerSync;
use alloy::sol_types::SolValue;
use banking_ledger_application::{
    EthereumUserOperationPreparation, EthereumUserOperationPrepareRequest,
    EthereumUserOperationPreparer, EthereumUserOperationPreparerError, EvmCallData, EvmQuantity,
    EvmUserOperation, EvmUserOperationRequest,
};
use banking_ledger_domain::core::{EvmAddress, EvmChainId, EvmTokenOwnerAddress};
use chrono::Utc;

use super::{
    DefaultEthereumUserOperationPreparerConfig, DefaultEthereumUserOperationPreparerError,
};
use crate::ethereum::bundler::EvmUserOperationGasEstimation;
use crate::ethereum::contract::{
    BankingSettlementAccount, BankingSettlementPaymaster, IEntryPoint,
};

pub struct DefaultEthereumUserOperationPreparer {
    chain_provider: DynProvider,
    bundler_provider: DynProvider,
    config: DefaultEthereumUserOperationPreparerConfig,
}

impl DefaultEthereumUserOperationPreparer {
    pub fn new(
        chain_provider: DynProvider,
        bundler_provider: DynProvider,
        config: DefaultEthereumUserOperationPreparerConfig,
    ) -> Self {
        Self {
            chain_provider,
            bundler_provider,
            config,
        }
    }

    fn packed_pair(high: U256, low: U256) -> Result<B256, EthereumUserOperationPreparerError> {
        let high_value = u128::try_from(high).map_err(|_| {
            EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::GasLimitOverflow,
            ))
        })?;
        let low_value = u128::try_from(low).map_err(|_| {
            EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::GasLimitOverflow,
            ))
        })?;
        let mut packed = [0; 32];
        packed[..16].copy_from_slice(&high_value.to_be_bytes());
        packed[16..].copy_from_slice(&low_value.to_be_bytes());
        Ok(B256::from(packed))
    }

    fn paymaster_and_data(
        paymaster: Address,
        verification_gas_limit: U256,
        post_op_gas_limit: U256,
        paymaster_data: &[u8],
    ) -> Result<Bytes, EthereumUserOperationPreparerError> {
        let verification_gas_limit_value =
            u128::try_from(verification_gas_limit).map_err(|_| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::GasLimitOverflow,
                ))
            })?;
        let post_op_gas_limit_value = u128::try_from(post_op_gas_limit).map_err(|_| {
            EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::GasLimitOverflow,
            ))
        })?;
        let mut packed = Vec::with_capacity(52 + paymaster_data.len());
        packed.extend_from_slice(paymaster.as_slice());
        packed.extend_from_slice(&verification_gas_limit_value.to_be_bytes());
        packed.extend_from_slice(&post_op_gas_limit_value.to_be_bytes());
        packed.extend_from_slice(paymaster_data);
        Ok(Bytes::from(packed))
    }

    fn timestamp_bytes(value: u64) -> [u8; 6] {
        let bytes = value.to_be_bytes();
        [bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]
    }
}

impl EthereumUserOperationPreparer for DefaultEthereumUserOperationPreparer {
    async fn prepare(
        &self,
        request: EthereumUserOperationPrepareRequest,
    ) -> Result<EthereumUserOperationPreparation, EthereumUserOperationPreparerError> {
        let transaction_request = request.transaction_request();
        let provider_chain_id = self.chain_provider.get_chain_id().await.map_err(|error| {
            EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::Rpc(error),
            ))
        })?;
        if EvmChainId::new(provider_chain_id) != transaction_request.chain_id() {
            return Err(EthereumUserOperationPreparerError::InconsistentChainId);
        }
        let sender = Address::from(*transaction_request.sender().as_bytes());
        let target = Address::from(*transaction_request.target().as_bytes());
        let transaction_call_data =
            Bytes::copy_from_slice(transaction_request.call_data().as_bytes());
        let account_call_data = {
            let mut mode = [0; 32];
            mode[0] = 1;
            let executions = vec![(target, U256::ZERO, transaction_call_data)];
            let execution_data = executions.abi_encode();
            BankingSettlementAccount::new(sender, &self.chain_provider)
                .execute(B256::from(mode), Bytes::from(execution_data))
                .calldata()
                .clone()
        };
        let paymaster_address = Address::from(*self.config.paymaster_contract.as_bytes());
        let paymaster = BankingSettlementPaymaster::new(paymaster_address, &self.chain_provider);
        let account_implementation =
            paymaster
                .accountImplementation()
                .call()
                .await
                .map_err(|error| {
                    EthereumUserOperationPreparerError::Backend(Box::new(
                        DefaultEthereumUserOperationPreparerError::Contract(error),
                    ))
                })?;
        let delegated_code = self
            .chain_provider
            .get_code_at(sender)
            .await
            .map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Rpc(error),
                ))
            })?;
        let mut expected_delegated_code = Vec::with_capacity(23);
        expected_delegated_code.extend_from_slice(&[0xef, 0x01, 0x00]);
        expected_delegated_code.extend_from_slice(account_implementation.as_slice());
        if delegated_code.as_ref() != expected_delegated_code {
            return Err(EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::AccountNotDelegated,
            )));
        }
        let entry_point_address = paymaster.entryPoint().call().await.map_err(|error| {
            EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::Contract(error),
            ))
        })?;
        let nonce = IEntryPoint::new(entry_point_address, &self.chain_provider)
            .getNonce(sender, Uint::<192, 3>::ZERO)
            .call()
            .await
            .map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Contract(error),
                ))
            })?;
        let max_priority_fee_per_gas = U256::from(
            self.chain_provider
                .get_max_priority_fee_per_gas()
                .await
                .map_err(|error| {
                    EthereumUserOperationPreparerError::Backend(Box::new(
                        DefaultEthereumUserOperationPreparerError::Rpc(error),
                    ))
                })?,
        );
        let max_fee_per_gas =
            U256::from(self.chain_provider.get_gas_price().await.map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Rpc(error),
                ))
            })?)
            .max(max_priority_fee_per_gas);
        let configured_paymaster_verification_gas_limit =
            U256::from(self.config.paymaster_verification_gas_limit);
        let paymaster_post_op_gas_limit = U256::from(self.config.paymaster_post_op_gas_limit);
        let sponsorship_current_date_time = Utc::now();
        let sponsorship_expires_at = sponsorship_current_date_time
            .checked_add_signed(self.config.sponsorship_signature_ttl)
            .ok_or_else(|| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::SponsorshipDeadlineOverflow,
                ))
            })?;
        if sponsorship_expires_at <= sponsorship_current_date_time {
            return Err(EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::InvalidSponsorshipSignatureTtl,
            )));
        }
        let sponsorship_valid_until =
            u64::try_from(sponsorship_expires_at.timestamp()).map_err(|_| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::SponsorshipDeadlineBeforeUnixEpoch,
                ))
            })?;
        if sponsorship_valid_until > (1_u64 << 48) - 1 {
            return Err(EthereumUserOperationPreparerError::Backend(Box::new(
                DefaultEthereumUserOperationPreparerError::SponsorshipDeadlineExceedsUint48,
            )));
        }
        let sponsorship_valid_after = 0_u64;
        let mut validity_data = Vec::with_capacity(12);
        validity_data.extend_from_slice(&Self::timestamp_bytes(sponsorship_valid_after));
        validity_data.extend_from_slice(&Self::timestamp_bytes(sponsorship_valid_until));
        let stub_signature = Bytes::from(vec![0xff; 65]);
        let mut stub_paymaster_data = validity_data.clone();
        stub_paymaster_data.extend_from_slice(&[0xff; 65]);
        let estimated_user_operation = PackedUserOperation {
            sender,
            nonce,
            factory: None,
            factory_data: None,
            call_data: account_call_data.clone(),
            call_gas_limit: U256::ZERO,
            verification_gas_limit: U256::ZERO,
            pre_verification_gas: U256::ZERO,
            max_fee_per_gas,
            max_priority_fee_per_gas,
            paymaster: Some(paymaster_address),
            paymaster_verification_gas_limit: Some(configured_paymaster_verification_gas_limit),
            paymaster_post_op_gas_limit: Some(paymaster_post_op_gas_limit),
            paymaster_data: Some(Bytes::from(stub_paymaster_data)),
            signature: stub_signature,
        };
        let gas_estimation: EvmUserOperationGasEstimation = self
            .bundler_provider
            .raw_request(
                "eth_estimateUserOperationGas".into(),
                (estimated_user_operation, entry_point_address),
            )
            .await
            .map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Rpc(error),
                ))
            })?;
        let paymaster_verification_gas_limit = gas_estimation
            .paymaster_verification_gas
            .max(configured_paymaster_verification_gas_limit);
        let paymaster_and_data = Self::paymaster_and_data(
            paymaster_address,
            paymaster_verification_gas_limit,
            paymaster_post_op_gas_limit,
            &validity_data,
        )?;
        let sponsorship_user_operation = BankingSettlementPaymaster::PackedUserOperation {
            sender,
            nonce,
            initCode: Bytes::new(),
            callData: account_call_data.clone(),
            accountGasLimits: Self::packed_pair(
                gas_estimation.verification_gas,
                gas_estimation.call_gas_limit,
            )?,
            preVerificationGas: gas_estimation.pre_verification_gas,
            gasFees: Self::packed_pair(max_priority_fee_per_gas, max_fee_per_gas)?,
            paymasterAndData: paymaster_and_data,
            signature: Bytes::new(),
        };
        let sponsorship_digest = paymaster
            .sponsorshipDigest(
                sponsorship_user_operation,
                Uint::<48, 1>::from(sponsorship_valid_after),
                Uint::<48, 1>::from(sponsorship_valid_until),
            )
            .call()
            .await
            .map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Contract(error),
                ))
            })?;
        let sponsorship_signature = self
            .config
            .sponsorship_signer
            .sign_hash_sync(&sponsorship_digest)
            .map_err(|error| {
                EthereumUserOperationPreparerError::Backend(Box::new(
                    DefaultEthereumUserOperationPreparerError::Signer(error),
                ))
            })?;
        let mut paymaster_data = validity_data;
        paymaster_data.extend_from_slice(&sponsorship_signature.as_bytes());
        Ok(EthereumUserOperationPreparation::new(
            EvmUserOperationRequest::new(
                transaction_request.chain_id(),
                EvmAddress::from_bytes(entry_point_address.into_array()),
                EvmUserOperation {
                    sender: EvmTokenOwnerAddress::new(transaction_request.sender()),
                    nonce: EvmQuantity::from_bytes(nonce.to_be_bytes()),
                    call_data: EvmCallData::from_bytes(account_call_data.to_vec()),
                    call_gas_limit: EvmQuantity::from_bytes(
                        gas_estimation.call_gas_limit.to_be_bytes(),
                    ),
                    verification_gas_limit: EvmQuantity::from_bytes(
                        gas_estimation.verification_gas.to_be_bytes(),
                    ),
                    pre_verification_gas: EvmQuantity::from_bytes(
                        gas_estimation.pre_verification_gas.to_be_bytes(),
                    ),
                    max_fee_per_gas: EvmQuantity::from_bytes(max_fee_per_gas.to_be_bytes()),
                    max_priority_fee_per_gas: EvmQuantity::from_bytes(
                        max_priority_fee_per_gas.to_be_bytes(),
                    ),
                    paymaster: self.config.paymaster_contract,
                    paymaster_verification_gas_limit: EvmQuantity::from_bytes(
                        paymaster_verification_gas_limit.to_be_bytes(),
                    ),
                    paymaster_post_op_gas_limit: EvmQuantity::from_bytes(
                        paymaster_post_op_gas_limit.to_be_bytes(),
                    ),
                    paymaster_data: EvmCallData::from_bytes(paymaster_data),
                },
            ),
        ))
    }
}
