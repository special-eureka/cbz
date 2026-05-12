//! [`Manga` enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L55-L62)
//!
use derive_more::Display;
use serde::{Deserialize, Serialize};

/// [Ref](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L55-L62)
#[derive(
    Debug,
    Clone,
    Copy,
    Deserialize,
    Serialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Display,
    Default,
)]
pub enum Manga {
    Yes,
    YesAndRightToLeft,
    No,
    #[default]
    #[serde(other)]
    Unknown,
}

impl From<&str> for Manga {
    fn from(value: &str) -> Self {
        match value {
            "Yes" => Self::Yes,
            "YesAndRightToLeft" => Self::YesAndRightToLeft,
            "No" => Self::No,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Manga;

    #[test]
    fn manga_display() -> anyhow::Result<()> {
        assert_eq!(Manga::Yes.to_string().as_str(), "Yes");
        assert_eq!(
            Manga::YesAndRightToLeft.to_string().as_str(),
            "YesAndRightToLeft"
        );
        assert_eq!(Manga::No.to_string().as_str(), "No");
        assert_eq!(Manga::Unknown.to_string().as_str(), "Unknown");
        Ok(())
    }
    #[test]
    fn manga_from_str() -> anyhow::Result<()> {
        assert_eq!(Manga::Yes, "Yes".into());
        assert_eq!(Manga::No, "No".into());
        assert_eq!(Manga::YesAndRightToLeft, "YesAndRightToLeft".into());
        assert_eq!(Manga::Unknown, "unknown".into());
        Ok(())
    }
    #[test]
    fn manga_deser() -> anyhow::Result<()> {
        assert_eq!(Manga::Yes, serde_json::from_str("\"Yes\"")?);
        assert_eq!(
            Manga::YesAndRightToLeft,
            serde_json::from_str("\"YesAndRightToLeft\"")?
        );
        assert_eq!(Manga::No, serde_json::from_str("\"No\"")?);
        assert_eq!(Manga::Unknown, serde_json::from_str("\"Unknown\"")?);
        assert_eq!(Manga::Unknown, serde_json::from_str("\"null\"")?);
        Ok(())
    }
    #[test]
    fn manga_ser() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&Manga::Yes)?.as_str(), "\"Yes\"");
        assert_eq!(
            serde_json::to_string(&Manga::YesAndRightToLeft)?.as_str(),
            "\"YesAndRightToLeft\""
        );
        assert_eq!(serde_json::to_string(&Manga::No)?.as_str(), "\"No\"");
        assert_eq!(
            serde_json::to_string(&Manga::Unknown)?.as_str(),
            "\"Unknown\""
        );
        Ok(())
    }
}
