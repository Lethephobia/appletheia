use appletheia::application::event::EventSelector;
use appletheia::application::messaging::Subscription;
use appletheia::application::projection::{ProjectorDescriptor, ProjectorName, ProjectorSpec};
use appletheia::domain::Aggregate;
use banking_ledger_domain::currency_issuance::{CurrencyIssuance, CurrencyIssuanceEventPayload};

/// Declares the subscription for the currency issuance projection projector.
pub struct CurrencyIssuanceProjectorSpec;

impl ProjectorSpec for CurrencyIssuanceProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor = ProjectorDescriptor::new(
        ProjectorName::new("currency_issuance"),
        Subscription::AnyOf(&[
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::ISSUED),
            EventSelector::new(
                CurrencyIssuance::TYPE,
                CurrencyIssuanceEventPayload::COMPLETED,
            ),
            EventSelector::new(CurrencyIssuance::TYPE, CurrencyIssuanceEventPayload::FAILED),
        ]),
    );
}
