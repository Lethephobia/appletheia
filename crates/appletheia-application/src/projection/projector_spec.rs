use super::ProjectorDescriptor;

/// Defines the stable descriptor for a projector.
pub trait ProjectorSpec {
    const DESCRIPTOR: ProjectorDescriptor;
}
