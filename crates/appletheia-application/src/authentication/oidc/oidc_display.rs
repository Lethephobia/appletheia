use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcDisplay {
    Page,
    Popup,
    Touch,
    Wap,
}

impl OidcDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Page => "page",
            Self::Popup => "popup",
            Self::Touch => "touch",
            Self::Wap => "wap",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_str_matches_spec_values() {
        assert_eq!(OidcDisplay::Page.as_str(), "page");
        assert_eq!(OidcDisplay::Popup.as_str(), "popup");
        assert_eq!(OidcDisplay::Touch.as_str(), "touch");
        assert_eq!(OidcDisplay::Wap.as_str(), "wap");
    }
}
