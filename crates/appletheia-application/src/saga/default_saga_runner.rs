use crate::event::EventEnvelope;
use crate::outbox::command::{CommandEnvelope, CommandOutboxEnqueuer};
use crate::request_context::{CausationId, MessageId};
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::{
    Saga, SagaNameOwned, SagaPredecessor, SagaProcessedEventStore, SagaRun, SagaRunId,
    SagaRunReport, SagaRunStore, SagaRunner, SagaRunnerError, SagaSpec,
};

pub struct DefaultSagaRunner<S, P, Q, U> {
    saga_run_store: S,
    saga_processed_event_store: P,
    command_outbox_enqueuer: Q,
    uow_factory: U,
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U> {
    pub fn new(
        saga_run_store: S,
        saga_processed_event_store: P,
        command_outbox_enqueuer: Q,
        uow_factory: U,
    ) -> Self {
        Self {
            saga_run_store,
            saga_processed_event_store,
            command_outbox_enqueuer,
            uow_factory,
        }
    }
}

impl<S, P, Q, U> DefaultSagaRunner<S, P, Q, U>
where
    S: SagaRunStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event_inner<SG: Saga>(
        &self,
        uow: &mut S::Uow,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let descriptor = <SG::Spec as SagaSpec>::DESCRIPTOR;
        let saga_name = SagaNameOwned::from(descriptor.name);
        let correlation_id = event.correlation_id;
        let trigger_event_id = event.event_id;

        if self
            .saga_run_store
            .read_by_trigger_event::<SG::Context>(uow, saga_name.clone(), trigger_event_id)
            .await?
            .is_some()
        {
            return Ok(SagaRunReport::AlreadyRun);
        }

        let context = match descriptor.predecessor {
            SagaPredecessor::Required(predecessor) => {
                let predecessor_command_message_id = MessageId::from(event.causation_id.value());
                let predecessor = self
                    .saga_run_store
                    .read_by_dispatched_command_message::<SG::Context>(
                        uow,
                        SagaNameOwned::from(predecessor.name),
                        predecessor_command_message_id,
                    )
                    .await?;
                let Some(predecessor) = predecessor else {
                    return Ok(SagaRunReport::PredecessorRunMissing);
                };
                Some(predecessor.context)
            }
            SagaPredecessor::None => None,
        };

        let inserted = self
            .saga_processed_event_store
            .mark_processed(uow, saga_name.clone(), event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaRunReport::EventAlreadyProcessed);
        }

        let domain_event = event.try_into_domain_event::<SG::EventAggregate>()?;

        let transition = saga
            .on_event(context, &domain_event)
            .map_err(|source| SagaRunnerError::Handler(Box::new(source)))?;

        let command = transition
            .command
            .map(|command| {
                CommandEnvelope::new(
                    &command.command,
                    correlation_id,
                    CausationId::from(event.event_id),
                    command.options,
                )
            })
            .transpose()?;
        let dispatched_command_message_id = command.as_ref().map(|command| command.message_id);

        let run = SagaRun {
            saga_run_id: SagaRunId::new(),
            saga_name: saga_name.clone(),
            trigger_event_id,
            dispatched_command_message_id,
            context: transition.context,
        };

        self.saga_run_store.write(uow, &run).await?;

        if let Some(command) = command {
            self.command_outbox_enqueuer
                .enqueue_command(uow, &command)
                .await?;

            Ok(SagaRunReport::CommandDispatched)
        } else {
            Ok(SagaRunReport::NoCommandDispatched)
        }
    }
}

impl<S, P, Q, U> SagaRunner for DefaultSagaRunner<S, P, Q, U>
where
    S: SagaRunStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event<SG: Saga>(
        &self,
        saga: &SG,
        event: &EventEnvelope,
    ) -> Result<SagaRunReport, SagaRunnerError> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self.handle_event_inner(&mut uow, saga, event).await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}
