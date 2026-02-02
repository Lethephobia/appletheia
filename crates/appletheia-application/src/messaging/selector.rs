pub trait Selector<M> {
    fn matches(&self, message: &M) -> bool;
}
