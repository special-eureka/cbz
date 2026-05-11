//! [`ComicPageType` enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L106-L120)
//!
use derive_more::{Display, FromStr};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    Display,
    Serialize,
    Deserialize,
    Hash,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    FromStr,
)]
/// [`ComicPageType` enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L106-L120)
///
pub enum ComicPageType {
    FrontCover,
    InnerCover,
    Roundup,
    #[default]
    Story,
    Advertisement,
    Editorial,
    Letters,
    Preview,
    BackCover,
    Other,
    Deleted,
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(ComicPageType::Story.to_string().as_str(), "Story");
        assert_eq!(ComicPageType::BackCover.to_string().as_str(), "BackCover");
    }
    #[test]
    fn test_from_str() -> anyhow::Result<()> {
        assert_eq!(ComicPageType::Advertisement, "Advertisement".parse()?);
        assert_eq!(ComicPageType::FrontCover, "FrontCover".parse()?);
        Ok(())
    }
    #[test]
    fn test_ser() -> anyhow::Result<()> {
        assert_eq!(
            to_string(&ComicPageType::Advertisement)?,
            "\"Advertisement\""
        );
        Ok(())
    }
    #[test]
    fn test_deser() -> anyhow::Result<()> {
        assert_eq!(ComicPageType::FrontCover, from_str("\"FrontCover\"")?);
        assert_eq!(ComicPageType::Story, from_str("\"Story\"")?);
        Ok(())
    }
}
