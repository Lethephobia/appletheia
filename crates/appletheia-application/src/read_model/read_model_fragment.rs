use serde::{Serialize, de::DeserializeOwned};

use super::ReadModelFragmentName;

/// Defines one independently stored read model fragment.
pub trait ReadModelFragment: Send + Sync + Sized + 'static {
    /// Identifies the physical fragment shared by read models.
    const NAME: ReadModelFragmentName;

    /// Identifies one stored fragment value.
    type Key: Clone + Serialize + DeserializeOwned + Send + Sync + Sized + 'static;

    /// Returns this fragment's physical key.
    fn key(&self) -> Self::Key;
}
