use appletheia::application::saga::SagaStep;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserOldPictureObjectDeletionSagaStep {
    DeletePictureObject,
}

impl SagaStep for UserOldPictureObjectDeletionSagaStep {}
