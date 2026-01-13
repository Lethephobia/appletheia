pub trait RequestHasher: Send + Sync {
    fn request_hash(&self, value: serde_json::Value) -> String;
}
