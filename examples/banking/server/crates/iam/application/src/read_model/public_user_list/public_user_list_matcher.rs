use appletheia::application::read_model::list::DefaultReadModelListMatcher;

use super::PublicUserListWatchQuery;

/// Matches public-user changes through the standard list membership algorithm.
pub type PublicUserListMatcher = DefaultReadModelListMatcher<PublicUserListWatchQuery>;

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::list::{
        ReadModelListChangeDecision, ReadModelListCoverage, ReadModelListMatcher,
    };
    use appletheia::application::read_model::pagination::{Sort, SortDirection};
    use appletheia::application::read_model::{
        ReadModelObservation, ReadModelPart, ReadModelPartChange, ReadModelPartPath,
        ReadModelPartPathSegment,
    };
    use appletheia::domain::{AggregateId, EventId, EventOccurredAt};
    use banking_iam_domain::{UserId, Username};
    use banking_shared_kernel_application::read_model::SearchTerm;
    use uuid::Uuid;

    use super::*;
    use crate::projection::{MaterializedUserStatus, PublicUserListItemPart, UserFragment};
    use crate::read_model::{PublicUserListCriteria, PublicUserListCursor, PublicUserListSortKey};

    fn user_id(value: &str) -> UserId {
        let uuid = Uuid::parse_str(value).expect("test UUID should be valid");
        UserId::try_from_uuid(uuid).expect("user ID should accept UUID")
    }

    fn fragment(user_id: UserId, username: &str) -> UserFragment {
        let event_id = EventId::new();

        UserFragment {
            id: user_id,
            username: Some(Username::try_from(username).expect("username should be valid")),
            display_name: None,
            bio: None,
            picture: None,
            status: MaterializedUserStatus::Active,
            created_at: EventOccurredAt::now(),
            observation: ReadModelObservation::new(event_id, event_id),
        }
    }

    fn search_term(value: &str) -> SearchTerm {
        SearchTerm::try_from(value).expect("search term should be valid")
    }

    fn changed(fragment: UserFragment) -> ReadModelPartChange {
        let item = PublicUserListItemPart::from(fragment.clone());
        let key = serde_json::to_value(item.key()).expect("part key should serialize");
        let path = ReadModelPartPath::new(vec![
            ReadModelPartPathSegment::Attribute("items".to_owned()),
            ReadModelPartPathSegment::Key(key),
        ]);
        ReadModelPartChange::try_changed(&fragment, &item, path, Vec::new(), Vec::new())
            .expect("part change should serialize")
    }

    #[test]
    fn includes_a_changed_item_when_all_normalized_terms_match() {
        let matcher = PublicUserListMatcher::new();
        let changed_fragment =
            fragment(user_id("00000000-0000-0000-0000-000000000002"), "alice_123");
        let query = PublicUserListWatchQuery {
            criteria: PublicUserListCriteria {
                username_contains: vec![search_term(" ALI "), search_term("ce_1")],
                ..PublicUserListCriteria::default()
            },
            ..PublicUserListWatchQuery::default()
        };
        let change = changed(changed_fragment);

        let decision = matcher
            .evaluate(&query, &ReadModelListCoverage::Complete, &change, false)
            .expect("part change should evaluate");

        assert_eq!(decision, ReadModelListChangeDecision::Included);
    }

    #[test]
    fn excludes_a_changed_item_beyond_the_loaded_cursor() {
        let matcher = PublicUserListMatcher::new();
        let changed_fragment =
            fragment(user_id("00000000-0000-0000-0000-000000000002"), "alice_123");
        let query = PublicUserListWatchQuery {
            criteria: PublicUserListCriteria::default(),
            sort: Sort {
                key: PublicUserListSortKey::UserId,
                direction: SortDirection::Asc,
            },
        };
        let coverage = ReadModelListCoverage::Through {
            cursor: PublicUserListCursor {
                created_at: changed_fragment.created_at,
                user_id: user_id("00000000-0000-0000-0000-000000000001"),
            },
        };
        let change = changed(changed_fragment);

        let decision = matcher
            .evaluate(&query, &coverage, &change, false)
            .expect("part change should evaluate");

        assert_eq!(decision, ReadModelListChangeDecision::Ignored);
    }

    #[test]
    fn applies_status_criteria_when_username_criteria_is_empty() {
        let matcher = PublicUserListMatcher::new();
        let mut changed_fragment =
            fragment(user_id("00000000-0000-0000-0000-000000000002"), "alice_123");
        changed_fragment.status = MaterializedUserStatus::Inactive;
        let change = changed(changed_fragment);

        let decision = matcher
            .evaluate(
                &PublicUserListWatchQuery::default(),
                &ReadModelListCoverage::Complete,
                &change,
                false,
            )
            .expect("part change should evaluate");

        assert_eq!(decision, ReadModelListChangeDecision::Ignored);
    }

    #[test]
    fn username_and_status_changes_invalidate_list_membership() {
        let matcher = PublicUserListMatcher::new();
        let query = PublicUserListWatchQuery {
            criteria: PublicUserListCriteria {
                username_contains: vec![search_term("bob")],
                ..PublicUserListCriteria::default()
            },
            ..PublicUserListWatchQuery::default()
        };
        let test_user_id = user_id("00000000-0000-0000-0000-000000000002");
        let username_change = changed(fragment(test_user_id, "alice_123"));
        let mut inactive_fragment = fragment(test_user_id, "alice_123");
        inactive_fragment.status = MaterializedUserStatus::Inactive;
        let status_change = changed(inactive_fragment);

        let username_decision = matcher
            .evaluate(
                &query,
                &ReadModelListCoverage::Complete,
                &username_change,
                true,
            )
            .expect("part change should evaluate");
        let status_decision = matcher
            .evaluate(
                &query,
                &ReadModelListCoverage::Complete,
                &status_change,
                true,
            )
            .expect("part change should evaluate");

        assert_eq!(username_decision, ReadModelListChangeDecision::Invalidated);
        assert_eq!(status_decision, ReadModelListChangeDecision::Invalidated);
    }

    #[test]
    fn removal_invalidates_a_list_that_contains_the_item() {
        let matcher = PublicUserListMatcher::new();
        let query = PublicUserListWatchQuery::default();
        let removed_user_id = user_id("00000000-0000-0000-0000-000000000002");
        let change = ReadModelPartChange::try_removed::<PublicUserListItemPart>(
            &removed_user_id,
            ReadModelPartPath::new(vec![
                ReadModelPartPathSegment::Attribute("items".to_owned()),
                ReadModelPartPathSegment::Key(
                    serde_json::to_value(removed_user_id).expect("part key should serialize"),
                ),
            ]),
            Vec::new(),
        )
        .expect("part tombstone should serialize");

        let decision = matcher
            .evaluate(&query, &ReadModelListCoverage::Complete, &change, true)
            .expect("part change should evaluate");

        assert_eq!(decision, ReadModelListChangeDecision::Invalidated);
    }
}
