use std::collections::{HashMap, HashSet};

use super::{
    ReadModelWatch, ReadModelWatchEndpoint, ReadModelWatchId, ReadModelWatchIndex,
    ReadModelWatchRegistry, ReadModelWatchRegistryConfig, ReadModelWatchRegistryError,
    ReadModelWatchSelector, ReadModelWatchSession, ReadModelWatchSessionId,
    ReadModelWatchSessionStatus, ReadModelWatchStatus,
};

/// Owns local sessions and serializes their shared-index updates.
///
/// Keep this registry for the entire endpoint lifetime. Mutations require exclusive
/// access. Registration adds a local watch only after the index accepts it; failed
/// removals retain their desired local state for retry.
/// Query execution and notification delivery are outside this registry.
pub struct DefaultReadModelWatchRegistry<I>
where
    I: ReadModelWatchIndex,
{
    index: I,
    endpoint: ReadModelWatchEndpoint,
    config: ReadModelWatchRegistryConfig,
    sessions: HashMap<ReadModelWatchSessionId, ReadModelWatchSession>,
}

impl<I> DefaultReadModelWatchRegistry<I>
where
    I: ReadModelWatchIndex,
{
    /// Creates a registry with a fresh endpoint retained for its lifetime.
    pub fn new(index: I, config: ReadModelWatchRegistryConfig) -> Self {
        Self {
            index,
            endpoint: ReadModelWatchEndpoint::new(),
            config,
            sessions: HashMap::new(),
        }
    }
}

impl<I> ReadModelWatchRegistry for DefaultReadModelWatchRegistry<I>
where
    I: ReadModelWatchIndex,
{
    fn endpoint(&self) -> ReadModelWatchEndpoint {
        self.endpoint
    }

    fn open_session(&mut self) -> ReadModelWatchSessionId {
        let session = ReadModelWatchSession::new();
        let id = session.id();
        self.sessions.insert(id, session);
        id
    }

    fn session(&self, id: ReadModelWatchSessionId) -> Option<&ReadModelWatchSession> {
        self.sessions.get(&id)
    }

    /// Index registration succeeds before the watch is added to the local session.
    async fn register_watch(
        &mut self,
        session_id: ReadModelWatchSessionId,
        watch: ReadModelWatch,
    ) -> Result<(), ReadModelWatchRegistryError> {
        if self
            .sessions
            .iter()
            .any(|(id, session)| *id != session_id && session.watches().contains_key(&watch.id()))
        {
            return Err(ReadModelWatchRegistryError::WatchAlreadyRegistered(
                watch.id(),
            ));
        }
        let selectors: HashSet<ReadModelWatchSelector> = self
            .sessions
            .values()
            .flat_map(|session| session.watches().values())
            .filter(|existing| existing.status() != ReadModelWatchStatus::Removing)
            .flat_map(|existing| existing.selectors().iter().cloned())
            .chain(watch.selectors().iter().cloned())
            .collect();
        let session = self
            .sessions
            .get_mut(&session_id)
            .ok_or(ReadModelWatchRegistryError::SessionNotFound(session_id))?;
        session.ensure_watch_registration_allowed(watch.id())?;
        self.index
            .replace(self.endpoint, &selectors, self.config.lease_duration)
            .await?;
        session.register_watch(watch)?;
        self.sessions.retain(|_, session| {
            session.complete_index_synchronization();
            session.status() == ReadModelWatchSessionStatus::Open
        });
        Ok(())
    }

    /// Removal is idempotent and preserves other watches using the same selectors.
    async fn unregister_watch(
        &mut self,
        session_id: ReadModelWatchSessionId,
        watch_id: ReadModelWatchId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.begin_watch_removal(watch_id);
        }
        self.synchronize_index().await
    }

    async fn close_session(
        &mut self,
        session_id: ReadModelWatchSessionId,
    ) -> Result<(), ReadModelWatchRegistryError> {
        if let Some(session) = self.sessions.get_mut(&session_id) {
            session.begin_close();
        }
        self.synchronize_index().await
    }

    /// Retries desired registrations and renews the endpoint lease.
    ///
    /// Call periodically before the lease expires. This republishes the complete
    /// selector union, so renewal also repairs an expired or uncertain registration.
    async fn synchronize_index(&mut self) -> Result<(), ReadModelWatchRegistryError> {
        let selectors: HashSet<ReadModelWatchSelector> = self
            .sessions
            .values()
            .flat_map(|session| session.watches().values())
            .filter(|watch| watch.status() != ReadModelWatchStatus::Removing)
            .flat_map(|watch| watch.selectors().iter().cloned())
            .collect();
        self.index
            .replace(self.endpoint, &selectors, self.config.lease_duration)
            .await?;
        self.sessions.retain(|_, session| {
            session.complete_index_synchronization();
            session.status() == ReadModelWatchSessionStatus::Open
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Duration, Utc};
    use std::future::{pending, poll_fn};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU8, Ordering};
    use std::task::Poll;
    use tokio::sync::Mutex;

    use crate::read_model::ReadModelFragmentNameOwned;
    use crate::read_model::watch::ReadModelWatchLeaseDuration;

    use super::*;
    use crate::read_model::watch::{ReadModelWatchIndexError, ReadModelWatchIndexExpiresAt};

    #[derive(Debug, thiserror::Error)]
    #[error("injected index failure after applying registration")]
    struct TestIndexError;

    #[derive(Clone, Default)]
    struct TestIndex {
        registrations: Arc<Mutex<HashMap<ReadModelWatchEndpoint, TestRegistration>>>,
        mode: Arc<AtomicU8>,
    }

    struct TestRegistration {
        selectors: HashSet<ReadModelWatchSelector>,
        expires_at: ReadModelWatchIndexExpiresAt,
    }

    impl TestRegistration {
        fn new(
            selectors: HashSet<ReadModelWatchSelector>,
            expires_at: ReadModelWatchIndexExpiresAt,
        ) -> Self {
            Self {
                selectors,
                expires_at,
            }
        }

        fn is_expired_at(&self, now: DateTime<Utc>) -> bool {
            self.expires_at.is_expired_at(now)
        }

        fn contains_selector(&self, selector: &ReadModelWatchSelector) -> bool {
            self.selectors.contains(selector)
        }
    }

    impl TestIndex {
        fn new() -> Self {
            Self::default()
        }
    }

    impl ReadModelWatchIndex for TestIndex {
        async fn replace(
            &self,
            endpoint: ReadModelWatchEndpoint,
            selectors: &HashSet<ReadModelWatchSelector>,
            lease_duration: ReadModelWatchLeaseDuration,
        ) -> Result<(), ReadModelWatchIndexError> {
            let mut registrations = self.registrations.lock().await;
            let now = Utc::now();
            registrations.retain(|_, registration| !registration.is_expired_at(now));
            if selectors.is_empty() {
                registrations.remove(&endpoint);
            } else {
                let expires_at = ReadModelWatchIndexExpiresAt::after(now, lease_duration)?;
                registrations.insert(
                    endpoint,
                    TestRegistration::new(selectors.clone(), expires_at),
                );
            }
            drop(registrations);
            match self.mode.load(Ordering::SeqCst) {
                1 => Err(ReadModelWatchIndexError::Replace(Box::new(TestIndexError))),
                2 => pending().await,
                _ => Ok(()),
            }
        }

        async fn find_endpoints(
            &self,
            selector: &ReadModelWatchSelector,
        ) -> Result<HashSet<ReadModelWatchEndpoint>, ReadModelWatchIndexError> {
            let mut registrations = self.registrations.lock().await;
            let now = Utc::now();
            registrations.retain(|_, registration| !registration.is_expired_at(now));
            Ok(registrations
                .iter()
                .filter(|(_, registration)| registration.contains_selector(selector))
                .map(|(endpoint, _)| *endpoint)
                .collect())
        }
    }

    fn selector() -> ReadModelWatchSelector {
        ReadModelWatchSelector::Fragment(
            ReadModelFragmentNameOwned::new("test_fragment".to_owned()).expect("valid name"),
        )
    }

    fn lease_duration() -> ReadModelWatchLeaseDuration {
        ReadModelWatchLeaseDuration::new(Duration::seconds(60)).expect("positive duration")
    }

    #[tokio::test]
    async fn shared_selectors_survive_until_the_last_session_closes() {
        let index = TestIndex::new();
        let mut registry = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig {
                lease_duration: lease_duration(),
            },
        );
        let endpoint = registry.endpoint();
        let first_session = registry.open_session();
        let second_session = registry.open_session();
        let first_watch = ReadModelWatch::new([selector(), selector()]).expect("valid watch");
        assert_eq!(first_watch.selectors().len(), 1);
        let first_id = first_watch.id();
        registry
            .register_watch(first_session, first_watch)
            .await
            .expect("register");
        registry
            .register_watch(
                second_session,
                ReadModelWatch::new([selector()]).expect("valid watch"),
            )
            .await
            .expect("register");
        assert_eq!(
            index.find_endpoints(&selector()).await.expect("lookup"),
            HashSet::from([endpoint])
        );

        registry
            .unregister_watch(first_session, first_id)
            .await
            .expect("unregister");
        registry
            .unregister_watch(first_session, first_id)
            .await
            .expect("idempotent removal");
        assert_eq!(
            index.find_endpoints(&selector()).await.expect("lookup"),
            HashSet::from([endpoint])
        );
        registry.close_session(second_session).await.expect("close");
        assert!(
            index
                .find_endpoints(&selector())
                .await
                .expect("lookup")
                .is_empty()
        );
        assert!(registry.session(second_session).is_none());
    }

    #[tokio::test]
    async fn closing_one_endpoint_preserves_another_endpoint() {
        let index = TestIndex::new();
        let mut first = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig {
                lease_duration: lease_duration(),
            },
        );
        let first_endpoint = first.endpoint();
        let mut second = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig {
                lease_duration: lease_duration(),
            },
        );
        let second_endpoint = second.endpoint();
        assert_ne!(first_endpoint, second_endpoint);
        let first_session = first.open_session();
        let second_session = second.open_session();
        first
            .register_watch(
                first_session,
                ReadModelWatch::new([selector()]).expect("watch"),
            )
            .await
            .expect("register");
        second
            .register_watch(
                second_session,
                ReadModelWatch::new([selector()]).expect("watch"),
            )
            .await
            .expect("register");
        assert_eq!(
            index.find_endpoints(&selector()).await.expect("lookup"),
            HashSet::from([first_endpoint, second_endpoint])
        );
        first.close_session(first_session).await.expect("close");
        assert_eq!(
            index.find_endpoints(&selector()).await.expect("lookup"),
            HashSet::from([second_endpoint])
        );
    }

    #[tokio::test]
    async fn uncertain_registration_and_removal_can_be_retried() {
        let index = TestIndex {
            mode: Arc::new(AtomicU8::new(1)),
            ..TestIndex::default()
        };
        let mut registry = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig {
                lease_duration: lease_duration(),
            },
        );
        let endpoint = registry.endpoint();
        let session = registry.open_session();
        let watch = ReadModelWatch::new([selector()]).expect("watch");
        let watch_id = watch.id();
        assert!(
            registry
                .register_watch(session, watch.clone())
                .await
                .is_err()
        );
        assert!(
            !registry
                .session(session)
                .expect("session")
                .watches()
                .contains_key(&watch_id)
        );
        assert_eq!(
            index.find_endpoints(&selector()).await.expect("lookup"),
            HashSet::from([endpoint])
        );

        index.mode.store(0, Ordering::SeqCst);
        registry
            .register_watch(session, watch)
            .await
            .expect("retry");
        assert_eq!(
            registry.session(session).expect("session").watches()[&watch_id].status(),
            ReadModelWatchStatus::Registered
        );

        index.mode.store(1, Ordering::SeqCst);
        assert!(registry.close_session(session).await.is_err());
        assert_eq!(
            registry
                .session(session)
                .expect("retained session")
                .status(),
            ReadModelWatchSessionStatus::Closing,
        );
        index.mode.store(0, Ordering::SeqCst);
        registry.synchronize_index().await.expect("retry close");
        assert!(registry.session(session).is_none());
        assert!(
            index
                .find_endpoints(&selector())
                .await
                .expect("lookup")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn cancelled_registration_does_not_add_a_local_watch() {
        let index = TestIndex {
            mode: Arc::new(AtomicU8::new(2)),
            ..TestIndex::default()
        };
        let mut registry = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig {
                lease_duration: lease_duration(),
            },
        );
        let session = registry.open_session();
        let watch = ReadModelWatch::new([selector()]).expect("watch");
        let watch_id = watch.id();
        {
            let mut registration = Box::pin(registry.register_watch(session, watch));
            poll_fn(|cx| {
                assert!(registration.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            })
            .await;
        }
        assert!(
            !registry
                .session(session)
                .expect("session")
                .watches()
                .contains_key(&watch_id)
        );
        index.mode.store(0, Ordering::SeqCst);
        registry
            .synchronize_index()
            .await
            .expect("remove stale index registration");
        assert!(
            index
                .find_endpoints(&selector())
                .await
                .expect("lookup")
                .is_empty()
        );
    }

    #[tokio::test]
    async fn expired_endpoints_are_excluded_and_registry_renewal_restores_selectors() {
        let index = TestIndex::new();
        let selector = ReadModelWatchSelector::Fragment(
            ReadModelFragmentNameOwned::new("test_fragment".to_owned()).expect("valid name"),
        );
        let lease_duration =
            ReadModelWatchLeaseDuration::new(Duration::seconds(60)).expect("duration");
        let mut registry = DefaultReadModelWatchRegistry::new(
            index.clone(),
            ReadModelWatchRegistryConfig { lease_duration },
        );
        let endpoint = registry.endpoint();
        let session = registry.open_session();
        registry
            .register_watch(
                session,
                ReadModelWatch::new([selector.clone()]).expect("watch"),
            )
            .await
            .expect("register");
        index.registrations.lock().await.insert(
            endpoint,
            TestRegistration::new(
                HashSet::from([selector.clone()]),
                ReadModelWatchIndexExpiresAt::new(Utc::now()),
            ),
        );
        assert!(
            index
                .find_endpoints(&selector)
                .await
                .expect("lookup")
                .is_empty()
        );
        registry
            .synchronize_index()
            .await
            .expect("renew after expiry");
        assert_eq!(
            index.find_endpoints(&selector).await.expect("lookup"),
            HashSet::from([endpoint])
        );
        assert!(!index.registrations.lock().await[&endpoint].is_expired_at(Utc::now()));
    }
}
