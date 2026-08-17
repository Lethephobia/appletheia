use appletheia::application::read_model::{
    ReadModel, ReadModelName, ReadModelObservation, ReadModelObservationSource, ReadModelPartTree,
};

use crate::projection::PublicUserListItemPart;

mod public_user_list_criteria;
mod public_user_list_cursor;
mod public_user_list_matcher;
mod public_user_list_reader;
mod public_user_list_reader_error;
mod public_user_list_sort_key;
mod public_user_list_watch_query;

pub use public_user_list_criteria::PublicUserListCriteria;
pub use public_user_list_cursor::PublicUserListCursor;
pub use public_user_list_matcher::PublicUserListMatcher;
pub use public_user_list_reader::PublicUserListReader;
pub use public_user_list_reader_error::PublicUserListReaderError;
pub use public_user_list_sort_key::PublicUserListSortKey;
pub use public_user_list_watch_query::PublicUserListWatchQuery;

/// Read model for public user list reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicUserList {
    pub items: Vec<PublicUserListItemPart>,
    pub next_cursor: Option<PublicUserListCursor>,
}

impl ReadModel for PublicUserList {
    const NAME: ReadModelName = ReadModelName::new("public_user_list");

    fn parts(read_model: Option<&Self>) -> Vec<ReadModelPartTree> {
        vec![ReadModelPartTree::collection::<PublicUserListItemPart>(
            "items",
            read_model.map(|read_model| read_model.items.as_slice()),
        )]
    }
}

impl ReadModelObservationSource for PublicUserList {
    fn observations(&self) -> Vec<ReadModelObservation> {
        self.items
            .iter()
            .flat_map(ReadModelObservationSource::observations)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use appletheia::application::read_model::{
        ReadModelObservation, ReadModelPart, watch::ReadModelWatchSelection,
    };
    use appletheia::domain::{EventId, EventOccurredAt};
    use banking_iam_domain::UserId;

    use crate::projection::{MaterializedUserStatus, UserFragment};

    use super::*;

    #[test]
    fn watch_selection_uses_each_list_item_as_a_root_fragment() {
        let user_id = UserId::new();
        let event_id = EventId::new();
        let read_model = PublicUserList {
            items: vec![PublicUserListItemPart {
                user_id,
                username: None,
                display_name: None,
                picture: None,
                status: MaterializedUserStatus::Active,
                created_at: EventOccurredAt::now(),
                observation: ReadModelObservation::new(event_id, event_id),
            }],
            next_cursor: None,
        };

        let selection = ReadModelWatchSelection::try_from_read_model(&read_model)
            .expect("public user list selection should serialize");

        assert_eq!(selection.read_model_name.value(), "public_user_list");
        assert_eq!(selection.partitions.len(), 1);
        assert!(selection.partition_dependencies.is_empty());
        let partition = read_model.items[0]
            .partition()
            .try_into_serialized::<UserFragment>()
            .expect("partition should serialize");

        assert_eq!(selection.partitions[0], partition);
    }
}
