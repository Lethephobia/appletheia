use banking_ledger_domain::core::{ChainNetwork, TokenAddress, TokenOwnerAddress};

use super::{
    DepositSettlementPreparation, DepositSettlementPrepareRequest, DepositSettlementPreparer,
    DepositSettlementPreparerError, EthereumDepositSettlementTransactionPrepareRequest,
    EthereumDepositSettlementTransactionPreparer, SolanaDepositSettlementPrepareRequest,
    SolanaDepositSettlementPreparer,
};
use crate::settlement::{EthereumUserOperationPrepareRequest, EthereumUserOperationPreparer};

pub struct DefaultDepositSettlementPreparer<S, T, U>
where
    S: SolanaDepositSettlementPreparer,
    T: EthereumDepositSettlementTransactionPreparer,
    U: EthereumUserOperationPreparer,
{
    solana: S,
    ethereum_transaction: T,
    ethereum_user_operation: U,
}

impl<S, T, U> DefaultDepositSettlementPreparer<S, T, U>
where
    S: SolanaDepositSettlementPreparer,
    T: EthereumDepositSettlementTransactionPreparer,
    U: EthereumUserOperationPreparer,
{
    pub fn new(solana: S, ethereum_transaction: T, ethereum_user_operation: U) -> Self {
        Self {
            solana,
            ethereum_transaction,
            ethereum_user_operation,
        }
    }
}

impl<S, T, U> DepositSettlementPreparer for DefaultDepositSettlementPreparer<S, T, U>
where
    S: SolanaDepositSettlementPreparer,
    T: EthereumDepositSettlementTransactionPreparer,
    U: EthereumUserOperationPreparer,
{
    async fn prepare(
        &self,
        request: DepositSettlementPrepareRequest,
    ) -> Result<DepositSettlementPreparation, DepositSettlementPreparerError> {
        let preparation = match (
            request.chain_network(),
            *request.token_address(),
            *request.token_owner_address(),
        ) {
            (
                ChainNetwork::Solana,
                TokenAddress::Solana(token_address),
                TokenOwnerAddress::Solana(token_owner_address),
            ) => {
                if request.evm_authorization().is_some() {
                    return Err(DepositSettlementPreparerError::UnexpectedEvmAuthorization);
                }
                let solana_preparation = self
                    .solana
                    .prepare(SolanaDepositSettlementPrepareRequest::new(
                        request.deposit_id(),
                        request.currency_decimals(),
                        token_address,
                        token_owner_address,
                        request.amount(),
                    ))
                    .await?;
                DepositSettlementPreparation::Solana(solana_preparation.transaction)
            }
            (
                ChainNetwork::Ethereum,
                TokenAddress::Ethereum(token_address),
                TokenOwnerAddress::Ethereum(token_owner_address),
            ) => {
                let transaction_preparation = self
                    .ethereum_transaction
                    .prepare(EthereumDepositSettlementTransactionPrepareRequest::new(
                        request.deposit_id(),
                        request.currency_decimals(),
                        token_address,
                        token_owner_address,
                        request.amount(),
                        request.evm_authorization(),
                    ))
                    .await?;
                let user_operation_preparation = self
                    .ethereum_user_operation
                    .prepare(EthereumUserOperationPrepareRequest::new(
                        transaction_preparation.into_transaction_request(),
                    ))
                    .await?;
                DepositSettlementPreparation::Ethereum(Box::new(
                    user_operation_preparation.into_user_operation_request(),
                ))
            }
            _ => return Err(DepositSettlementPreparerError::InconsistentChainValues),
        };

        Ok(preparation)
    }
}

#[cfg(test)]
mod tests {
    use banking_ledger_domain::core::{
        ChainNetwork, CurrencyAmount, CurrencyDecimals, EvmAddress, EvmChainId,
        EvmTokenContractAddress, EvmTokenOwnerAddress, SolanaAccountAddress,
        SolanaMintAccountAddress, SolanaTokenOwnerAddress, TokenAddress, TokenOwnerAddress,
    };
    use banking_ledger_domain::deposit::DepositId;

    use crate::settlement::{
        DepositSettlementPreparation, Erc2612Permit, Erc2612PermitDeadline, Erc2612PermitSignature,
        Erc3009ReceiveAuthorization, Erc3009ReceiveAuthorizationNonce,
        Erc3009ReceiveAuthorizationSignature, Erc3009ReceiveAuthorizationValidAfter,
        Erc3009ReceiveAuthorizationValidBefore, EthereumDepositSettlementTransactionPreparation,
        EthereumDepositSettlementTransactionPreparerError, EthereumUserOperationPreparation,
        EthereumUserOperationPrepareRequest, EthereumUserOperationPreparer,
        EthereumUserOperationPreparerError, EvmCallData, EvmDepositAuthorization, EvmQuantity,
        EvmTransactionRequest, EvmUserOperation, EvmUserOperationRequest,
        SolanaDepositSettlementPreparation, SolanaPreparedDepositTransaction,
    };

    use super::{
        DefaultDepositSettlementPreparer, DepositSettlementPrepareRequest,
        DepositSettlementPreparer, DepositSettlementPreparerError,
        EthereumDepositSettlementTransactionPrepareRequest,
        EthereumDepositSettlementTransactionPreparer, SolanaDepositSettlementPrepareRequest,
        SolanaDepositSettlementPreparer,
    };

    struct StubSolanaDepositSettlementPreparer;

    impl SolanaDepositSettlementPreparer for StubSolanaDepositSettlementPreparer {
        async fn prepare(
            &self,
            _request: SolanaDepositSettlementPrepareRequest,
        ) -> Result<SolanaDepositSettlementPreparation, DepositSettlementPreparerError> {
            Ok(SolanaDepositSettlementPreparation {
                transaction: SolanaPreparedDepositTransaction::from_bytes(b"solana".to_vec()),
            })
        }
    }

    struct StubEthereumDepositSettlementTransactionPreparer;

    impl EthereumDepositSettlementTransactionPreparer
        for StubEthereumDepositSettlementTransactionPreparer
    {
        async fn prepare(
            &self,
            request: EthereumDepositSettlementTransactionPrepareRequest,
        ) -> Result<
            EthereumDepositSettlementTransactionPreparation,
            EthereumDepositSettlementTransactionPreparerError,
        > {
            Ok(EthereumDepositSettlementTransactionPreparation::new(
                EvmTransactionRequest::new(
                    EvmChainId::new(11_155_111),
                    *request.token_owner_address().address(),
                    EvmAddress::from_bytes([5; 20]),
                    EvmCallData::from_bytes(b"transaction".to_vec()),
                ),
            ))
        }
    }

    struct StubEthereumUserOperationPreparer;

    impl EthereumUserOperationPreparer for StubEthereumUserOperationPreparer {
        async fn prepare(
            &self,
            request: EthereumUserOperationPrepareRequest,
        ) -> Result<EthereumUserOperationPreparation, EthereumUserOperationPreparerError> {
            assert_eq!(
                request.transaction_request().chain_id(),
                EvmChainId::new(11_155_111)
            );
            assert_eq!(
                request.transaction_request().call_data().as_bytes(),
                b"transaction"
            );
            Ok(EthereumUserOperationPreparation::new(
                EvmUserOperationRequest::new(
                    EvmChainId::new(11_155_111),
                    EvmAddress::from_bytes([5; 20]),
                    EvmUserOperation {
                        sender: EvmTokenOwnerAddress::new(EvmAddress::from_bytes([4; 20])),
                        nonce: EvmQuantity::default(),
                        call_data: EvmCallData::from_bytes(b"ethereum".to_vec()),
                        call_gas_limit: EvmQuantity::default(),
                        verification_gas_limit: EvmQuantity::default(),
                        pre_verification_gas: EvmQuantity::default(),
                        max_fee_per_gas: EvmQuantity::default(),
                        max_priority_fee_per_gas: EvmQuantity::default(),
                        paymaster: EvmAddress::from_bytes([6; 20]),
                        paymaster_verification_gas_limit: EvmQuantity::default(),
                        paymaster_post_op_gas_limit: EvmQuantity::default(),
                        paymaster_data: EvmCallData::from_bytes(Vec::new()),
                    },
                ),
            ))
        }
    }

    #[tokio::test]
    async fn routes_preparation_to_the_preparer_for_the_selected_chain() {
        let preparer = DefaultDepositSettlementPreparer::new(
            StubSolanaDepositSettlementPreparer,
            StubEthereumDepositSettlementTransactionPreparer,
            StubEthereumUserOperationPreparer,
        );

        let solana_preparation = preparer
            .prepare(solana_request(None))
            .await
            .expect("Solana deposit should be routed");
        assert!(matches!(
            solana_preparation,
            DepositSettlementPreparation::Solana(ref transaction)
                if transaction.as_bytes() == b"solana"
        ));

        let ethereum_preparation = preparer
            .prepare(ethereum_request(Some(EvmDepositAuthorization::Erc2612(
                erc2612_permit(),
            ))))
            .await
            .expect("Ethereum deposit should be routed");
        assert!(matches!(
            ethereum_preparation,
            DepositSettlementPreparation::Ethereum(ref request)
                if request.user_operation.call_data.as_bytes() == b"ethereum"
        ));
    }

    #[tokio::test]
    async fn supports_allowance_and_signed_authorizations_only_for_ethereum() {
        let preparer = DefaultDepositSettlementPreparer::new(
            StubSolanaDepositSettlementPreparer,
            StubEthereumDepositSettlementTransactionPreparer,
            StubEthereumUserOperationPreparer,
        );

        assert!(preparer.prepare(ethereum_request(None)).await.is_ok());
        assert!(
            preparer
                .prepare(ethereum_request(Some(EvmDepositAuthorization::Erc3009(
                    erc3009_authorization(),
                ))))
                .await
                .is_ok()
        );
        assert!(matches!(
            preparer
                .prepare(solana_request(Some(EvmDepositAuthorization::Erc2612(
                    erc2612_permit(),
                ))))
                .await,
            Err(DepositSettlementPreparerError::UnexpectedEvmAuthorization)
        ));
    }

    #[tokio::test]
    async fn rejects_prepare_values_from_different_chains_before_dispatch() {
        let preparer = DefaultDepositSettlementPreparer::new(
            StubSolanaDepositSettlementPreparer,
            StubEthereumDepositSettlementTransactionPreparer,
            StubEthereumUserOperationPreparer,
        );
        let request = DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana,
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
            None,
        );

        assert!(matches!(
            preparer.prepare(request).await,
            Err(DepositSettlementPreparerError::InconsistentChainValues)
        ));
    }

    fn solana_request(
        evm_authorization: Option<EvmDepositAuthorization>,
    ) -> DepositSettlementPrepareRequest {
        DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Solana,
            TokenAddress::Solana(SolanaMintAccountAddress::new(
                SolanaAccountAddress::from_bytes([3; 32]),
            )),
            TokenOwnerAddress::Solana(SolanaTokenOwnerAddress::new(
                SolanaAccountAddress::from_bytes([4; 32]),
            )),
            CurrencyAmount::new(100),
            evm_authorization,
        )
    }

    fn ethereum_request(
        evm_authorization: Option<EvmDepositAuthorization>,
    ) -> DepositSettlementPrepareRequest {
        DepositSettlementPrepareRequest::new(
            DepositId::new(),
            CurrencyDecimals::new(6),
            ChainNetwork::Ethereum,
            TokenAddress::Ethereum(EvmTokenContractAddress::new(EvmAddress::from_bytes(
                [3; 20],
            ))),
            TokenOwnerAddress::Ethereum(EvmTokenOwnerAddress::new(EvmAddress::from_bytes([4; 20]))),
            CurrencyAmount::new(100),
            evm_authorization,
        )
    }

    fn erc2612_permit() -> Erc2612Permit {
        Erc2612Permit::new(
            Erc2612PermitDeadline::new(u64::MAX),
            Erc2612PermitSignature::from_bytes([5; 65]),
        )
    }

    fn erc3009_authorization() -> Erc3009ReceiveAuthorization {
        Erc3009ReceiveAuthorization::new(
            Erc3009ReceiveAuthorizationValidAfter::new(0),
            Erc3009ReceiveAuthorizationValidBefore::new(u64::MAX),
            Erc3009ReceiveAuthorizationNonce::from_bytes([6; 32]),
            Erc3009ReceiveAuthorizationSignature::from_bytes([7; 65]),
        )
    }
}
