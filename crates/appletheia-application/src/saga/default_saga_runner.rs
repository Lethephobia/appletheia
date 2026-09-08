use super::{SagaContext, SagaRoute, SagaRouteError, SagaState, SagaStep};
use crate::command::CommandFailureEnvelope;
use std::error::Error;

use crate::event::EventEnvelope;
use crate::outbox::command::CommandOutboxEnqueuer;
use crate::request_context::{CausationId, MessageId};
use crate::unit_of_work::UnitOfWork;
use crate::unit_of_work::UnitOfWorkFactory;

use super::SagaInstance;
use super::{
    EnqueuedCommandCount, SagaCommandFailureRunReport, SagaDefinition, SagaEventRunReport,
    SagaInstanceStore, SagaNameOwned, SagaProcessedCommandFailureStore, SagaProcessedEventStore,
    SagaRunner, SagaRunnerError,
};

pub struct DefaultSagaRunner<S, P, F, Q, U> {
    saga_instance_store: S,
    processed_event_store: P,
    processed_command_failure_store: F,
    command_outbox_enqueuer: Q,
    uow_factory: U,
}

impl<S, P, F, Q, U> DefaultSagaRunner<S, P, F, Q, U> {
    pub fn new(
        saga_instance_store: S,
        processed_event_store: P,
        processed_command_failure_store: F,
        command_outbox_enqueuer: Q,
        uow_factory: U,
    ) -> Self {
        Self {
            saga_instance_store,
            processed_event_store,
            processed_command_failure_store,
            command_outbox_enqueuer,
            uow_factory,
        }
    }
}

impl<S, P, F, Q, U> DefaultSagaRunner<S, P, F, Q, U>
where
    S: SagaInstanceStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    F: SagaProcessedCommandFailureStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event_inner<SS: SagaState, ST: SagaStep, SE: Error + Send + Sync + 'static>(
        &self,
        uow: &mut S::Uow,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError<SE>> {
        if !saga_definition.is_subscribed(event) {
            return Ok(SagaEventRunReport::NotSubscribed);
        }

        let saga_name = SagaNameOwned::from(saga_definition.name());
        let correlation_id = event.correlation_id;

        let command_message_id = MessageId::from(event.causation_id.value());
        let owned_instance = self
            .saga_instance_store
            .find_by_dispatched_command_message_id::<SS, ST>(
                uow,
                saga_name.clone(),
                command_message_id,
            )
            .await?;
        let (mut instance, causative_step) = if let Some(instance) = owned_instance {
            let Some(dispatched_command) = instance
                .dispatched_commands
                .iter()
                .find(|command| command.message_id == command_message_id)
            else {
                return Ok(SagaEventRunReport::CommandNotOwned);
            };
            let causative_step = dispatched_command.step;
            (instance, Some(causative_step))
        } else {
            if !saga_definition.starts_on(event) {
                return Ok(SagaEventRunReport::InstanceNotFound);
            }
            if self
                .saga_instance_store
                .find_by_correlation_id::<SS, ST>(uow, saga_name.clone(), correlation_id)
                .await?
                .is_some()
            {
                return Ok(SagaEventRunReport::AlreadyStarted);
            }
            (
                SagaInstance::new(saga_name.clone(), correlation_id, event.event_id),
                None,
            )
        };

        let Some(
            SagaRoute::StartsOn { step, handler, .. } | SagaRoute::OnEvent { step, handler, .. },
        ) = saga_definition.find_event_route(event, causative_step)
        else {
            return Ok(SagaEventRunReport::NoMatchingRoute);
        };

        let inserted = self
            .processed_event_store
            .mark_processed(uow, saga_name.clone(), correlation_id, event.event_id)
            .await?;
        if !inserted {
            return Ok(SagaEventRunReport::AlreadyProcessed);
        }

        let mut context = SagaContext::new(&mut instance, CausationId::from(event.event_id), *step);
        handler(&mut context, event)?;

        self.saga_instance_store.save(uow, &instance).await?;

        let commands = instance.uncommitted_commands().to_vec();
        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        if !commands.is_empty() {
            self.command_outbox_enqueuer
                .enqueue_commands(uow, &commands)
                .await?;
        }

        let report = SagaEventRunReport::Processed {
            enqueued_command_count,
        };

        Ok(report)
    }

    async fn handle_command_failure_inner<
        SS: SagaState,
        ST: SagaStep,
        SE: Error + Send + Sync + 'static,
    >(
        &self,
        uow: &mut S::Uow,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError<SE>> {
        if failure.origin.saga_name.value() != saga_definition.name().value() {
            return Ok(SagaCommandFailureRunReport::NotSubscribed);
        }

        let saga_name = SagaNameOwned::from(saga_definition.name());
        let Some(mut instance) = self
            .saga_instance_store
            .find_by_dispatched_command_message_id::<SS, ST>(
                uow,
                saga_name.clone(),
                failure.command_message_id,
            )
            .await?
        else {
            return Ok(SagaCommandFailureRunReport::InstanceNotFound);
        };
        let Some(dispatched_command) = instance
            .dispatched_commands
            .iter()
            .find(|command| command.message_id == failure.command_message_id)
        else {
            return Ok(SagaCommandFailureRunReport::CommandNotOwned);
        };
        if failure.origin.saga_instance_id != instance.saga_instance_id
            || failure.command_name != dispatched_command.command_name
            || failure.origin.step.try_into_step::<ST>().ok() != Some(dispatched_command.step)
            || failure.correlation_id != instance.correlation_id
        {
            return Ok(SagaCommandFailureRunReport::CommandNotOwned);
        }
        let causative_step = dispatched_command.step;

        let inserted = self
            .processed_command_failure_store
            .mark_processed(
                uow,
                instance.saga_instance_id,
                failure.failure_id,
                failure.command_message_id,
            )
            .await?;
        if !inserted {
            return Ok(SagaCommandFailureRunReport::AlreadyProcessed);
        }

        let Some(SagaRoute::OnCommandFailed { step, handler, .. }) =
            saga_definition.find_command_failure_route(causative_step)
        else {
            return Ok(SagaCommandFailureRunReport::NoMatchingRoute);
        };
        let mut context =
            SagaContext::new(&mut instance, CausationId::from(failure.failure_id), *step);
        handler(&mut context, failure).map_err(SagaRouteError::Handler)?;
        self.saga_instance_store.save(uow, &instance).await?;
        let commands = instance.uncommitted_commands().to_vec();
        let enqueued_command_count = EnqueuedCommandCount::from_usize_saturating(commands.len());
        if !commands.is_empty() {
            self.command_outbox_enqueuer
                .enqueue_commands(uow, &commands)
                .await?;
        }
        let report = SagaCommandFailureRunReport::Processed {
            enqueued_command_count,
        };
        Ok(report)
    }
}

impl<S, P, F, Q, U> SagaRunner for DefaultSagaRunner<S, P, F, Q, U>
where
    S: SagaInstanceStore,
    P: SagaProcessedEventStore<Uow = S::Uow>,
    F: SagaProcessedCommandFailureStore<Uow = S::Uow>,
    Q: CommandOutboxEnqueuer<Uow = S::Uow>,
    U: UnitOfWorkFactory<Uow = S::Uow>,
{
    async fn handle_event<SS: SagaState, ST: SagaStep, SE: Error + Send + Sync + 'static>(
        &self,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        event: &EventEnvelope,
    ) -> Result<SagaEventRunReport, SagaRunnerError<SE>> {
        let mut uow = self.uow_factory.begin().await?;

        let result = self
            .handle_event_inner(&mut uow, saga_definition, event)
            .await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }

    async fn handle_command_failure<
        SS: SagaState,
        ST: SagaStep,
        SE: Error + Send + Sync + 'static,
    >(
        &self,
        saga_definition: &SagaDefinition<'_, SS, ST, SE>,
        failure: &CommandFailureEnvelope,
    ) -> Result<SagaCommandFailureRunReport, SagaRunnerError<SE>> {
        let mut uow = self.uow_factory.begin().await?;
        let result = self
            .handle_command_failure_inner(&mut uow, saga_definition, failure)
            .await;
        match result {
            Ok(report) => {
                uow.commit().await?;
                Ok(report)
            }
            Err(error) => Err(uow.rollback_with_operation_error(error).await?),
        }
    }
}
