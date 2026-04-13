use super::Selector;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum Subscription<'a, S> {
    All,
    AnyOf(&'a [S]),
    One(&'a S),
}

impl<'a, S> Subscription<'a, S> {
    pub fn matches<M>(&self, message: &M) -> bool
    where
        S: Selector<M>,
    {
        match self {
            Subscription::All => true,
            Subscription::AnyOf(selectors) => {
                selectors.iter().any(|selector| selector.matches(message))
            }
            Subscription::One(selector) => selector.matches(message),
        }
    }
}
