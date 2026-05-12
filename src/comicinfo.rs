//! [`ComicInfo.xml`](https://github.com/anansi-project/comicinfo) implementation
//!
//! This is based out of the schema provided by [`anansi-project`](https://github.com/anansi-project).
//!
//! If `comicinfo` feature flag is enabled,
//! this crate readers and writers should automatically handle them.
//!
//! Only [`v2`](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd) and [`v2.1`](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/drafts/v2.1/ComicInfo.xsd) are supported.
//!

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::comicinfo::{
    age_rating::AgeRating, comic_page_info::ComicPageInfo, manga::Manga, rating::Rating,
    yes_no::YesNo,
};
pub mod age_rating;
pub mod comic_page_info;
pub mod comic_page_type;
pub mod manga;
pub mod rating;
pub mod yes_no;

/// [`ComicInfo` complex type](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/drafts/v2.1/ComicInfo.xsd#L4-L51)
///
/// It is worth noting that this type uses the [`v2.1`](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/drafts/v2.1/ComicInfo.xsd#L4-L51) draft schema.
///
#[derive(Debug, Builder, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
#[serde(rename_all = "PascalCase")]
#[builder(setter(strip_option), default)]
pub struct ComicInfo {
    #[serde(rename = "@xmlns:xsi")]
    #[builder(default = "default_xsi()", setter(skip))]
    xsi: Option<String>,
    #[serde(rename = "@xmlns:xsd")]
    #[builder(default = "default_xsd()", setter(skip))]
    xsd: Option<String>,
    pub title: Option<String>,
    pub series: Option<String>,
    pub number: Option<String>,
    pub count: Option<usize>,
    pub volume: Option<usize>,
    pub alternate_series: Option<String>,
    pub alternate_number: Option<String>,
    pub alternate_count: Option<usize>,
    pub summary: Option<String>,
    pub notes: Option<String>,
    pub year: Option<usize>,
    pub month: Option<usize>,
    pub day: Option<usize>,
    pub writer: Option<String>,
    pub inker: Option<String>,
    pub colorist: Option<String>,
    pub letterer: Option<String>,
    pub cover_artist: Option<String>,
    pub editor: Option<String>,
    pub translator: Option<String>,
    pub publisher: Option<String>,
    pub imprint: Option<String>,
    pub genre: Option<String>,
    pub tags: Option<String>,
    // TODO this should be an [`url::Url`] but versatility i guess.
    pub web: Option<String>,
    pub page_count: Option<usize>,
    #[serde(rename = "LanguageISO")]
    pub language_iso: Option<String>,
    pub format: Option<String>,
    // TODO from str
    pub black_and_white: Option<YesNo>,
    // TODO from str
    pub manga: Option<Manga>,
    pub characters: Option<String>,
    pub teams: Option<String>,
    pub locations: Option<String>,
    pub scan_information: Option<String>,
    pub story_arc: Option<String>,
    pub story_ard_number: Option<String>,
    pub series_group: Option<String>,
    pub age_rating: Option<AgeRating>,
    #[builder(setter(each = "add_page"))]
    #[serde(with = "serde_pages")]
    pub pages: Vec<ComicPageInfo>,
    pub community_rating: Option<Rating>,
    pub main_character_or_team: Option<String>,
    pub review: Option<String>,
    #[serde(rename = "GTIN")]
    pub gtin: Option<String>,
}

mod serde_pages {
    use std::borrow::Cow;

    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::comicinfo::comic_page_info::ComicPageInfo;

    #[derive(Debug, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    struct Pages<'a> {
        page: Cow<'a, [ComicPageInfo]>,
    }
    pub fn serialize<S>(pages: &[ComicPageInfo], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Pages {
            page: Cow::Borrowed(pages),
        }
        .serialize(serializer)
    }
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<ComicPageInfo>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Pages::deserialize(deserializer)?.page.into_owned())
    }
}

impl ComicInfo {
    /// Get the the xmlns
    pub fn xmlns_xsi(&self) -> Option<&str> {
        self.xsi.as_deref()
    }
    pub fn xmlns_xsd(&self) -> Option<&str> {
        self.xsd.as_deref()
    }
}

fn default_xsi() -> Option<String> {
    Some("http://www.w3.org/2001/XMLSchema-instance".into())
}

fn default_xsd() -> Option<String> {
    Some("http://www.w3.org/2001/XMLSchema".into())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_and_white() -> anyhow::Result<()> {
        let val = ComicInfoBuilder::create_empty()
            .black_and_white(YesNo::Yes)
            .build()?;
        let val_xml = serde_xml_rs::to_string(&val)?;
        assert_eq!(
            val_xml.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?", ""),
            format!(
                "<ComicInfo xmlns:xsi=\"{}\" xmlns:xsd=\"{}\"><BlackAndWhite>Yes</BlackAndWhite></ComicInfo>",
                default_xsi().unwrap(),
                default_xsd().unwrap()
            )
        );
        Ok(())
    }
    #[test]
    fn test_pages() -> anyhow::Result<()> {
        let val = ComicInfoBuilder::create_empty()
            .add_page(ComicPageInfo::builder().image(1).build()?)
            .build()?;
        let val_xml = serde_xml_rs::to_string(&val)?;
        assert_eq!(
            val_xml.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", ""),
            format!(
                "<ComicInfo xmlns:xsi=\"{}\" xmlns:xsd=\"{}\"><Pages><Page Image=\"1\" Type=\"Story\" DoublePage=\"false\" ImageSize=\"0\" Key=\"\" Bookmark=\"\" /></Pages></ComicInfo>",
                default_xsi().unwrap(),
                default_xsd().unwrap()
            )
        );
        Ok(())
    }
}
