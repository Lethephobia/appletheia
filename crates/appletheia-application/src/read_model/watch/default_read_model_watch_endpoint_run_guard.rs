use std::sync::Mutex;

use super::{
    default_read_model_watch_endpoint_state::DefaultReadModelWatchEndpointState,
    default_read_model_watch_endpoint_status::DefaultReadModelWatchEndpointStatus,
};

// Closing local reception must also happen when the run future is cancelled.
pub(super) struct DefaultReadModelWatchEndpointRunGuard<'a> {
    pub state: &'a Mutex<DefaultReadModelWatchEndpointState>,
}

impl Drop for DefaultReadModelWatchEndpointRunGuard<'_> {
    fn drop(&mut self) {
        let mut state = self
            .state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        state
            .status
            .send_replace(DefaultReadModelWatchEndpointStatus::Closed);
        state.watch_senders.clear();
    }
}
