use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource,
    ReadModelPartChange, ReadModelPartChangeError, ReadModelPartChangeRoute,
    ReadModelPartPathResolver, ReadModelPartTree, SerializedPartition,
    SerializedReadModelFragmentChange,
};

use crate::projection::{
    OrganizationFragment, OrganizationMembershipFragment, PrivateUserDetailsPart,
    PrivateUserIdentityPart, PrivateUserOrganizationMembershipPart, UserFragment,
    UserIdentityFragment,
};

mod user_private_info_reader;
mod user_private_info_reader_error;

pub use user_private_info_reader::UserPrivateInfoReader;
pub use user_private_info_reader_error::UserPrivateInfoReaderError;

/// Read model containing private information for the owning user.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserPrivateInfo {
    pub user: PrivateUserDetailsPart,
    pub identities: Vec<PrivateUserIdentityPart>,
    pub organization_memberships: Vec<PrivateUserOrganizationMembershipPart>,
}

impl ReadModelObservationSource for UserPrivateInfo {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.user
            .observations()
            .into_iter()
            .chain(self.identities.iter().map(|identity| identity.observation))
            .chain(
                self.organization_memberships
                    .iter()
                    .flat_map(ReadModelObservationSource::observations),
            )
            .collect()
    }
}

impl ReadModel for UserPrivateInfo {
    const NAME: ReadModelName = ReadModelName::new("user_private_info");
    const PART_CHANGE_ROUTES: &'static [ReadModelPartChangeRoute] = &[
        ReadModelPartChangeRoute::from_fragment::<UserIdentityFragment>(
            map_identity_to_private_info,
        ),
        ReadModelPartChangeRoute::from_fragment::<OrganizationMembershipFragment>(
            map_membership_to_private_info,
        ),
    ];

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![
            ReadModelPartTree::field::<PrivateUserDetailsPart>(
                "user",
                read_model.map(|read_model| &read_model.user),
            ),
            ReadModelPartTree::collection_with_explicit_route::<PrivateUserIdentityPart>(
                "identities",
                read_model.map(|read_model| read_model.identities.as_slice()),
            ),
            ReadModelPartTree::collection_with_explicit_route::<
                PrivateUserOrganizationMembershipPart,
            >(
                "organization_memberships",
                read_model.map(|read_model| read_model.organization_memberships.as_slice()),
            ),
        ]
    }
}

fn map_identity_to_private_info(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<UserIdentityFragment, PrivateUserIdentityPart>(
        change,
        path_resolver,
        |fragment| user_audience(fragment.user_id),
        |key| user_audience(key.user_id),
        |_| Ok(Vec::new()),
    )
}

fn map_membership_to_private_info(
    change: &SerializedReadModelFragmentChange,
    path_resolver: ReadModelPartPathResolver,
) -> Result<Vec<ReadModelPartChange>, ReadModelPartChangeError> {
    ReadModelPartChange::map_one::<
        OrganizationMembershipFragment,
        PrivateUserOrganizationMembershipPart,
    >(
        change,
        path_resolver,
        |fragment| user_audience(fragment.user.id),
        |key| user_audience(key.user_id),
        |fragment| {
            Ok(vec![SerializedPartition::try_from_fragment_key::<
                OrganizationFragment,
            >(&fragment.organization.id)?])
        },
    )
}

fn user_audience(
    user_id: banking_iam_domain::UserId,
) -> Result<Vec<SerializedPartition>, ReadModelPartChangeError> {
    Ok(vec![SerializedPartition::try_from_fragment_key::<
        UserFragment,
    >(&user_id)?])
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::ReadModelFragmentChange;
    use banking_iam_domain::{OrganizationId, UserId, UserIdentityProvider, UserIdentitySubject};

    use crate::projection::{OrganizationMembershipFragmentKey, UserIdentityFragmentKey};

    use super::*;

    #[test]
    fn explicit_private_routes_preserve_complete_fragment_keys() {
        let identity_key = UserIdentityFragmentKey {
            user_id: UserId::new(),
            provider: UserIdentityProvider::try_from("https://accounts.example.com")
                .expect("provider should be valid"),
            subject: UserIdentitySubject::try_from("user-123").expect("subject should be valid"),
        };
        let identity_change =
            ReadModelFragmentChange::<UserIdentityFragment>::try_removed(&identity_key)
                .expect("identity fragment change should be valid")
                .try_into_serialized()
                .expect("identity fragment should serialize");
        let membership_key = OrganizationMembershipFragmentKey {
            user_id: UserId::new(),
            organization_id: OrganizationId::new(),
        };
        let membership_change =
            ReadModelFragmentChange::<OrganizationMembershipFragment>::try_removed(&membership_key)
                .expect("membership fragment change should be valid")
                .try_into_serialized()
                .expect("membership fragment should serialize");

        let path_resolver = ReadModelPartPathResolver::new(UserPrivateInfo::parts(None));
        let identity_changes =
            map_identity_to_private_info(&identity_change, path_resolver.clone())
                .expect("identity route should succeed");
        let membership_changes = map_membership_to_private_info(&membership_change, path_resolver)
            .expect("membership route should succeed");

        assert_removed_partition_key_matches(&identity_changes[0], &identity_key);
        assert_removed_partition_key_matches(&membership_changes[0], &membership_key);
    }

    fn assert_removed_partition_key_matches(
        change: &ReadModelPartChange,
        expected_key: &impl serde::Serialize,
    ) {
        let ReadModelPartChange::Removed {
            source_partition,
            path,
            ..
        } = change
        else {
            panic!("route should produce a removed part change");
        };

        let expected_key = serde_json::to_value(expected_key).expect("key should serialize");
        assert_eq!(source_partition.value()["key"], expected_key);
        assert_eq!(
            serde_json::to_value(path).expect("path should serialize")[1]["value"],
            expected_key
        );
    }
}
