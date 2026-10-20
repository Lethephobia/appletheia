use tokio::sync::watch;

use super::{ReadModelWatchReceiver, ReadModelWatchReceiverError, ReadModelWatchSignal};

pub struct DefaultReadModelWatchReceiver {
    receiver: watch::Receiver<()>,
}

impl DefaultReadModelWatchReceiver {
    pub fn new(receiver: watch::Receiver<()>) -> Self {
        Self { receiver }
    }
}

impl ReadModelWatchReceiver for DefaultReadModelWatchReceiver {
    async fn receive(&mut self) -> Result<ReadModelWatchSignal, ReadModelWatchReceiverError> {
        match self.receiver.changed().await {
            Ok(()) => Ok(ReadModelWatchSignal::Changed),
            Err(_) => Ok(ReadModelWatchSignal::Closed),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{future::Future, future::poll_fn, task::Poll};

    use super::*;

    #[tokio::test]
    async fn notifications_are_coalesced_and_changes_between_calls_are_retained() {
        let (sender, channel_receiver) = watch::channel(());
        let mut receiver = DefaultReadModelWatchReceiver::new(channel_receiver);
        sender.send(()).unwrap();
        sender.send(()).unwrap();
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );

        {
            let waiting = receiver.receive();
            tokio::pin!(waiting);
            poll_fn(|context| {
                assert!(waiting.as_mut().poll(context).is_pending());
                Poll::Ready(())
            })
            .await;
        }

        sender.send(()).unwrap();
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );
    }

    #[tokio::test]
    async fn cancelling_a_wait_does_not_consume_a_notification() {
        let (sender, channel_receiver) = watch::channel(());
        let mut receiver = DefaultReadModelWatchReceiver::new(channel_receiver);
        {
            let waiting = receiver.receive();
            tokio::pin!(waiting);
            poll_fn(|context| {
                assert!(waiting.as_mut().poll(context).is_pending());
                Poll::Ready(())
            })
            .await;
            sender.send(()).unwrap();
        }
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );
    }

    #[tokio::test]
    async fn closing_the_sender_ends_reception_after_pending_changes() {
        let (sender, channel_receiver) = watch::channel(());
        let mut receiver = DefaultReadModelWatchReceiver::new(channel_receiver);
        sender.send(()).unwrap();
        drop(sender);
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Changed
        );
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
        assert_eq!(
            receiver.receive().await.unwrap(),
            ReadModelWatchSignal::Closed
        );
    }

    #[tokio::test]
    async fn closing_the_sender_wakes_a_pending_wait() {
        let (sender, channel_receiver) = watch::channel(());
        let mut receiver = DefaultReadModelWatchReceiver::new(channel_receiver);
        let waiting = receiver.receive();
        tokio::pin!(waiting);
        poll_fn(|context| {
            assert!(waiting.as_mut().poll(context).is_pending());
            Poll::Ready(())
        })
        .await;
        drop(sender);
        assert_eq!(waiting.await.unwrap(), ReadModelWatchSignal::Closed);
    }

    #[tokio::test]
    async fn dropping_the_receiver_is_visible_to_the_sender() {
        let (sender, channel_receiver) = watch::channel(());
        let receiver = DefaultReadModelWatchReceiver::new(channel_receiver);
        assert!(!sender.is_closed());
        drop(receiver);
        sender.closed().await;
        assert!(sender.is_closed());
        assert!(sender.send(()).is_err());
    }
}
