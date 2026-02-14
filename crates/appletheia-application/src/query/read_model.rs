use crate::projection::ProjectorName;

pub trait ReadModel: Send + Sync + 'static {
    const PROJECTOR: ProjectorName;
}

