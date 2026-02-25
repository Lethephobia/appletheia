use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub(crate) struct JwtAuthTokenClaims {
    #[serde(rename = "iss")]
    pub issuer: String,

    #[serde(rename = "aud", deserialize_with = "deserialize_audiences")]
    pub audiences: Vec<String>,

    #[serde(rename = "sub")]
    pub subject: String,

    #[serde(rename = "sub_type")]
    pub subject_type: String,

    #[serde(rename = "iat")]
    pub issued_at: u64,

    #[serde(rename = "exp")]
    pub expires_at: u64,

    #[serde(rename = "jti")]
    pub token_id: String,
}

fn deserialize_audiences<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;
    match value {
        Value::String(s) => Ok(vec![s]),
        Value::Array(items) => items
            .into_iter()
            .map(|item| match item {
                Value::String(s) => Ok(s),
                other => Err(serde::de::Error::custom(format!(
                    "audience item must be string but got {other:?}"
                ))),
            })
            .collect(),
        other => Err(serde::de::Error::custom(format!(
            "audience must be string or array but got {other:?}"
        ))),
    }
}
