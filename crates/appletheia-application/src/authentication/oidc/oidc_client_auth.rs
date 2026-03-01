use super::OidcClientSecret;

#[derive(Clone, Debug)]
pub enum OidcClientAuth {
    None,
    ClientSecretBasic { client_secret: OidcClientSecret },
    ClientSecretPost { client_secret: OidcClientSecret },
}
