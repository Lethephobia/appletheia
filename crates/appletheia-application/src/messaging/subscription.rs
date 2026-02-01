#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Subscription<'a, S> {
    All,
    Only(&'a [S]),
}

impl<'a, S> Subscription<'a, S> {
    pub fn matches<M>(&self, message: &M) -> bool
    where
        S: super::Selector<M>,
    {
        match self {
            Subscription::All => true,
            Subscription::Only(selectors) => {
                selectors.iter().any(|selector| selector.matches(message))
            }
        }
    }
}
