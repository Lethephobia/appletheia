#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct JwtLeewaySeconds(u64);

impl JwtLeewaySeconds {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn value(&self) -> u64 {
        self.0
    }
}

impl Default for JwtLeewaySeconds {
    fn default() -> Self {
        Self::new(60)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_60_seconds() {
        assert_eq!(JwtLeewaySeconds::default().value(), 60);
    }

    #[test]
    fn new_stores_value() {
        assert_eq!(JwtLeewaySeconds::new(123).value(), 123);
    }
}
