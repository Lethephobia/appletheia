use super::TopicId;

pub trait TopicIdAccess {
    fn topic_id(&self) -> &TopicId;
}

