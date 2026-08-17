use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};
use appletheia::domain::EventOccurredAt;

use crate::projection::{
    InternalOrganizationDetailsPart, InternalUserSummaryPart, OrganizationFragment, UserFragment,
};

mod organization_management_info_reader;
mod organization_management_info_reader_error;

pub use organization_management_info_reader::OrganizationManagementInfoReader;
pub use organization_management_info_reader_error::OrganizationManagementInfoReaderError;

/// Organization information visible to its administrators.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrganizationManagementInfo {
    pub organization: InternalOrganizationDetailsPart,
    pub owner: InternalUserSummaryPart,
    pub owner_since: EventOccurredAt,
    pub owner_observation: ReadModelObservation,
}

impl ReadModelObservationSource for OrganizationManagementInfo {
    fn observations(&self) -> Vec<ReadModelObservation> {
        let mut observations = vec![self.owner_observation];
        observations.extend(self.organization.observations());
        observations.extend(self.owner.observations());
        observations
    }
}

impl ReadModel for OrganizationManagementInfo {
    const NAME: ReadModelName = ReadModelName::new("organization_management_info");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] =
        &[ReadModelPartChangeRoute::from_fragment::<
            OrganizationFragment,
        >(map_organization_to_management_info)];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field_with_explicit_route::<InternalOrganizationDetailsPart>(
                "organization",
                read_model.map(|read_model| &read_model.organization),
            ),
            ReadModelPartTree::field::<InternalUserSummaryPart>(
                "owner",
                read_model.map(|read_model| &read_model.owner),
            ),
        ]
    }
}

fn map_organization_to_management_info(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    let mut changes =
        ReadModelPartChange::map_one::<OrganizationFragment, InternalOrganizationDetailsPart>(
            change,
            path_resolver.clone(),
            |_| Ok(Vec::new()),
            |_| Ok(Vec::new()),
            |fragment| {
                Ok(vec![SerializedPartition::try_from_fragment_key::<
                    UserFragment,
                >(&fragment.owner.id)?])
            },
        )?;
    let Some(fragment) = change.try_fragment::<OrganizationFragment>()? else {
        let Some(fragment_key) = change.try_removed_key::<OrganizationFragment>()? else {
            return Ok(changes);
        };
        let replacement_path = path_resolver
            .try_for_route_key::<OrganizationFragment, InternalUserSummaryPart>(&fragment_key)?;
        changes.push(ReadModelPartChange::try_removed_from_fragment::<
            OrganizationFragment,
            InternalUserSummaryPart,
        >(&fragment_key, replacement_path, Vec::new())?);

        return Ok(changes);
    };
    let owner = InternalUserSummaryPart::from(fragment.owner.clone());
    let replacement_path = path_resolver.try_for_part(&owner)?;
    changes.push(ReadModelPartChange::try_changed(
        &fragment,
        &owner,
        replacement_path,
        Vec::new(),
        Vec::new(),
    )?);

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::ReadModelFragmentChange;
    use appletheia::application::read_model::ReadModelPartPathSegment;
    use appletheia::application::read_model::watch::ReadModelWatchSelection;
    use appletheia::domain::EventId;
    use banking_iam_domain::{OrganizationDisplayName, OrganizationHandle, OrganizationId, UserId};

    use crate::projection::{MaterializedUserStatus, UserFragment};

    use super::*;

    #[test]
    fn watch_selection_uses_the_organization_and_owner_as_root_fragments() {
        let organization_id = OrganizationId::new();
        let owner_user_id = UserId::new();
        let event_id = EventId::new();
        let observation = ReadModelObservation::new(event_id, event_id);
        let read_model = OrganizationManagementInfo {
            organization: InternalOrganizationDetailsPart {
                organization_id,
                handle: OrganizationHandle::try_from("test_organization")
                    .expect("handle should be valid"),
                display_name: OrganizationDisplayName::try_from("Test Organization")
                    .expect("display name should be valid"),
                picture: None,
                observation,
                description: None,
                website_url: None,
                created_at: EventOccurredAt::now(),
            },
            owner: InternalUserSummaryPart {
                user_id: owner_user_id,
                username: None,
                display_name: None,
                picture: None,
                observation,
            },
            owner_since: EventOccurredAt::now(),
            owner_observation: observation,
        };

        let selection = ReadModelWatchSelection::try_from_read_model(&read_model)
            .expect("organization management selection should serialize");

        assert_eq!(
            selection.read_model_name.value(),
            "organization_management_info"
        );
        assert_eq!(selection.partitions.len(), 2);
        assert!(selection.partition_dependencies.is_empty());
    }

    #[test]
    fn organization_changes_replace_the_organization_and_owner_root_parts() {
        let organization_id = OrganizationId::new();
        let owner_user_id = UserId::new();
        let event_id = EventId::new();
        let observation = ReadModelObservation::new(event_id, event_id);
        let fragment = OrganizationFragment {
            id: organization_id,
            owner: UserFragment {
                id: owner_user_id,
                username: None,
                display_name: None,
                bio: None,
                picture: None,
                status: MaterializedUserStatus::Active,
                created_at: EventOccurredAt::now(),
                observation,
            },
            owner_since: EventOccurredAt::now(),
            owner_observation: observation,
            handle: OrganizationHandle::try_from("test_organization")
                .expect("handle should be valid"),
            display_name: OrganizationDisplayName::try_from("Test Organization")
                .expect("display name should be valid"),
            description: None,
            website_url: None,
            picture: None,
            created_at: EventOccurredAt::now(),
            observation,
        };
        let fragment_change = ReadModelFragmentChange::try_from_fragment(&fragment)
            .expect("organization fragment change should serialize")
            .try_into_serialized()
            .expect("organization fragment should serialize");
        let path_resolver = ReadModelPartPathResolver::new(OrganizationManagementInfo::parts(None));

        let changes = map_organization_to_management_info(&fragment_change, path_resolver)
            .expect("organization management changes should map");

        assert_eq!(changes.len(), 2);
        assert!(
            changes[0]
                .try_part::<InternalOrganizationDetailsPart>()
                .expect("organization details should deserialize")
                .is_some()
        );
        assert_eq!(
            changes[0].path().segments(),
            &[ReadModelPartPathSegment::Attribute(
                "organization".to_owned()
            )]
        );
        assert!(changes[0].audience_partitions().is_empty());
        assert_eq!(changes[0].referenced_partitions().len(), 1);
        assert_eq!(
            changes[0].referenced_partitions()[0],
            SerializedPartition::try_from_fragment_key::<UserFragment>(&owner_user_id)
                .expect("owner partition should serialize")
        );
        assert!(
            changes[1]
                .try_part::<InternalUserSummaryPart>()
                .expect("owner summary should deserialize")
                .is_some()
        );
        assert_eq!(
            changes[1].path().segments(),
            &[ReadModelPartPathSegment::Attribute("owner".to_owned())]
        );
        assert!(changes[1].audience_partitions().is_empty());
        assert!(changes[1].referenced_partitions().is_empty());
        assert_eq!(changes[0].source_partition(), changes[1].source_partition());
    }

    #[test]
    fn organization_removal_removes_the_organization_and_owner_root_parts() {
        let organization_id = OrganizationId::new();
        let fragment_change =
            ReadModelFragmentChange::<OrganizationFragment>::try_removed(&organization_id)
                .expect("organization fragment removal should serialize")
                .try_into_serialized()
                .expect("organization fragment removal should serialize");
        let path_resolver = ReadModelPartPathResolver::new(OrganizationManagementInfo::parts(None));

        let changes = map_organization_to_management_info(&fragment_change, path_resolver)
            .expect("organization management removals should map");

        assert_eq!(changes.len(), 2);
        assert!(changes[0].removes::<InternalOrganizationDetailsPart>());
        assert_eq!(
            changes[0].path().segments(),
            &[ReadModelPartPathSegment::Attribute(
                "organization".to_owned()
            )]
        );
        assert!(changes[1].removes::<InternalUserSummaryPart>());
        assert_eq!(
            changes[1].path().segments(),
            &[ReadModelPartPathSegment::Attribute("owner".to_owned())]
        );
        assert_eq!(changes[0].source_partition(), changes[1].source_partition());
    }
}
