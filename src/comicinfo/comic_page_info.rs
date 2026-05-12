//! [`ComicPageInfo` complex type](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L94-L103)
//!

use std::num::NonZeroUsize;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::comicinfo::comic_page_type::ComicPageType;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Builder, Serialize, Deserialize)]
#[non_exhaustive]
/// Describes each page of the book.
///
pub struct ComicPageInfo {
    /// Page number.
    #[serde(rename = "@Image")]
    pub image: usize,
    /// Type of the page
    #[serde(rename = "@Type", default = "default_types")]
    #[builder(default = "default_types()", setter(each = "add_type"))]
    pub type_: Vec<ComicPageType>,
    /// Whether the page is a double spread.
    #[serde(rename = "@DoublePage", default)]
    #[builder(default)]
    pub double_page: bool,
    /// File size of the image, supposedly in bytes.
    #[serde(rename = "@ImageSize", default)]
    #[builder(default)]
    pub image_size: usize,
    /// ???
    #[serde(rename = "@Key", default)]
    #[builder(default)]
    pub key: String,
    /// ComicRack uses this field when adding a bookmark in a book.
    #[serde(rename = "@Bookmark", default)]
    #[builder(default)]
    pub bookmark: String,
    /// Width of the image in pixels.
    #[serde(rename = "@ImageWidth", default)]
    #[builder(default)]
    pub image_width: Option<NonZeroUsize>,
    /// Height of the image in pixels.
    #[serde(rename = "@ImageHeight", default)]
    #[builder(default)]
    pub image_height: Option<NonZeroUsize>,
}

impl ComicPageInfo {
    pub fn builder() -> ComicPageInfoBuilder {
        ComicPageInfoBuilder::default()
    }
}

fn default_types() -> Vec<ComicPageType> {
    vec![ComicPageType::default()]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_default() -> anyhow::Result<()> {
        let res = serde_xml_rs::to_string(&ComicPageInfo::builder().image(1).build()?)?;
        assert_eq!(
            res.as_str(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ComicPageInfo Image=\"1\" Type=\"Story\" DoublePage=\"false\" ImageSize=\"0\" Key=\"\" Bookmark=\"\" />"
        );
        Ok(())
    }
    #[test]
    fn test_ser_1() -> anyhow::Result<()> {
        let res = serde_xml_rs::to_string(
            &ComicPageInfo::builder()
                .image(1)
                .image_width(Some(NonZeroUsize::new(120).unwrap()))
                .image_height(Some(NonZeroUsize::new(500).unwrap()))
                .build()?,
        )?;
        assert_eq!(
            res.as_str(),
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ComicPageInfo Image=\"1\" Type=\"Story\" DoublePage=\"false\" ImageSize=\"0\" Key=\"\" Bookmark=\"\" ImageWidth=\"120\" ImageHeight=\"500\" />"
        );
        Ok(())
    }
    #[test]
    fn test_deser() -> anyhow::Result<()> {
        let res = ComicPageInfo::builder()
            .image(1)
            .add_type(ComicPageType::Story)
            .add_type(ComicPageType::Other)
            .build()?;
        assert_eq!(
            res,
            serde_xml_rs::from_str(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ComicPageInfo Image=\"1\" Type=\"Story Other\" DoublePage=\"false\" ImageSize=\"0\" Key=\"\" Bookmark=\"\" />"
            )?
        );
        Ok(())
    }
    #[test]
    fn test_deser_image_only() -> anyhow::Result<()> {
        let res = ComicPageInfo::builder().image(1).build()?;
        assert_eq!(
            res,
            serde_xml_rs::from_str(
                "<?xml version=\"1.0\" encoding=\"UTF-8\"?><ComicPageInfo Image=\"1\" />"
            )?
        );
        Ok(())
    }
}
