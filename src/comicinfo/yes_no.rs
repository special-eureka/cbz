//! [`Yes` and `No` Enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L48-L53)

use derive_more::Display;
use serde::{Deserialize, Serialize};

/// Yes No Enum
/// [Ref](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L48)
#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
pub enum YesNo {
    Yes,
    No,
    #[serde(other)]
    Unknown,
}

impl From<&str> for YesNo {
    fn from(value: &str) -> Self {
        match value {
            "Yes" => Self::Yes,
            "No" => Self::No,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::YesNo;

    #[test]
    fn yes_no_display() -> anyhow::Result<()> {
        assert_eq!(YesNo::Yes.to_string().as_str(), "Yes");
        assert_eq!(YesNo::No.to_string().as_str(), "No");
        assert_eq!(YesNo::Unknown.to_string().as_str(), "Unknown");
        Ok(())
    }
    #[test]
    fn yes_no_from_str() -> anyhow::Result<()> {
        assert_eq!(YesNo::Yes, "Yes".into());
        assert_eq!(YesNo::No, "No".into());
        assert_eq!(YesNo::Unknown, "unknown".into());
        Ok(())
    }
    #[test]
    fn yes_no_deser() -> anyhow::Result<()> {
        assert_eq!(YesNo::Yes, serde_json::from_str("\"Yes\"")?);
        assert_eq!(YesNo::No, serde_json::from_str("\"No\"")?);
        assert_eq!(YesNo::Unknown, serde_json::from_str("\"Unknown\"")?);
        assert_eq!(YesNo::Unknown, serde_json::from_str("\"null\"")?);
        Ok(())
    }
    #[test]
    fn yes_no_ser() -> anyhow::Result<()> {
        assert_eq!(serde_json::to_string(&YesNo::Yes)?.as_str(), "\"Yes\"");
        assert_eq!(serde_json::to_string(&YesNo::No)?.as_str(), "\"No\"");
        assert_eq!(
            serde_json::to_string(&YesNo::Unknown)?.as_str(),
            "\"Unknown\""
        );
        Ok(())
    }
}
