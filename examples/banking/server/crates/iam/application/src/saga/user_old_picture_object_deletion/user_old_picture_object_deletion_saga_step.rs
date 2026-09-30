use appletheia::saga_step;

#[saga_step]
pub enum UserOldPictureObjectDeletionSagaStep {
    DeletePictureObject,
}
