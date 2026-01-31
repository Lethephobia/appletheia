#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Subscription<'a, S> {
    All,
    Only(&'a [S]),
}
