use std::error::Error;

use super::{
    ProjectorDefinition, ProjectorDefinitionBuilderError, ProjectorEventHandlerBuilder,
    ProjectorName, ProjectorRoute,
};
use crate::unit_of_work::UnitOfWork;
use appletheia_domain::{Aggregate, EventName};

pub struct ProjectorDefinitionBuilder<'a, U: UnitOfWork, E: Error + Send + Sync + 'static> {
    name: ProjectorName,
    routes: Vec<ProjectorRoute<'a, U, E>>,
}

impl<'a, U: UnitOfWork + 'a, E: Error + Send + Sync + 'static>
    ProjectorDefinitionBuilder<'a, U, E>
{
    pub fn new(name: ProjectorName) -> Self {
        Self {
            name,
            routes: Vec::new(),
        }
    }

    pub fn on<A: Aggregate>(
        self,
        event_name: EventName,
    ) -> ProjectorEventHandlerBuilder<'a, U, E, A> {
        ProjectorEventHandlerBuilder::new(self, event_name)
    }

    pub fn build(self) -> Result<ProjectorDefinition<'a, U, E>, ProjectorDefinitionBuilderError> {
        Ok(ProjectorDefinition::new(self.name, self.routes)?)
    }

    pub(crate) fn add_route(mut self, route: ProjectorRoute<'a, U, E>) -> Self {
        self.routes.push(route);
        self
    }
}
