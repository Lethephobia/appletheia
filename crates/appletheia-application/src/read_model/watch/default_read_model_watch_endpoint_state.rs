use std::collections::HashMap;

use tokio::sync::watch;

use super::{
    ReadModelWatchId, default_read_model_watch_endpoint_status::DefaultReadModelWatchEndpointStatus,
};

pub(super) struct DefaultReadModelWatchEndpointState {
    pub status: watch::Sender<DefaultReadModelWatchEndpointStatus>,
    pub watch_senders: HashMap<ReadModelWatchId, watch::Sender<()>>,
}
