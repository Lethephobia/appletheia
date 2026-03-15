#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandDispatchResult<O, R> {
    Executed(O),
    Replayed(R),
}

impl<O, R> CommandDispatchResult<O, R> {
    pub fn is_executed(&self) -> bool {
        matches!(self, Self::Executed(_))
    }

    pub fn is_replayed(&self) -> bool {
        matches!(self, Self::Replayed(_))
    }
}

#[cfg(test)]
mod tests {
    use super::CommandDispatchResult;

    #[test]
    fn reports_executed_variant() {
        let result = CommandDispatchResult::<u32, u32>::Executed(1);

        assert!(result.is_executed());
        assert!(!result.is_replayed());
    }

    #[test]
    fn reports_replayed_variant() {
        let result = CommandDispatchResult::<u32, u32>::Replayed(1);

        assert!(!result.is_executed());
        assert!(result.is_replayed());
    }
}
