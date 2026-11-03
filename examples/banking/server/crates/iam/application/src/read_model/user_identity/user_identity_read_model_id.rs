use std::fmt::{self, Display};

use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};
use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::Serialize;

use super::UserIdentityReadModelIdError;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize)]
#[serde(transparent)]
pub struct UserIdentityReadModelId(String);

impl UserIdentityReadModelId {
    pub fn new(
        provider: &UserIdentityProvider,
        subject: &UserIdentitySubject,
    ) -> Result<Self, UserIdentityReadModelIdError> {
        let json = serde_json::to_vec(&[provider.value(), subject.value()])?;
        Ok(Self(URL_SAFE_NO_PAD.encode(json)))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl Display for UserIdentityReadModelId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use banking_iam_domain::{UserIdentityProvider, UserIdentitySubject};
    use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

    use super::UserIdentityReadModelId;

    #[test]
    fn id_is_deterministic_and_encodes_the_pair_as_json() {
        let provider = UserIdentityProvider::try_from("https://issuer.example").unwrap();
        let subject = UserIdentitySubject::try_from("subject/日本語\"").unwrap();
        let id = UserIdentityReadModelId::new(&provider, &subject).unwrap();

        assert_eq!(
            id,
            UserIdentityReadModelId::new(&provider, &subject).unwrap()
        );
        let decoded = URL_SAFE_NO_PAD.decode(id.value()).unwrap();
        assert_eq!(
            serde_json::from_slice::<[String; 2]>(&decoded).unwrap(),
            [provider.value().to_owned(), subject.value().to_owned()]
        );
        assert!(!id.value().contains(['=', '+', '/']));
        assert_eq!(id.to_string(), id.value());
    }

    #[test]
    fn separator_characters_do_not_make_distinct_pairs_collide() {
        let first = UserIdentityReadModelId::new(
            &UserIdentityProvider::try_from("a:b").unwrap(),
            &UserIdentitySubject::try_from("c").unwrap(),
        )
        .unwrap();
        let second = UserIdentityReadModelId::new(
            &UserIdentityProvider::try_from("a").unwrap(),
            &UserIdentitySubject::try_from("b:c").unwrap(),
        )
        .unwrap();

        assert_ne!(first, second);
    }
}
