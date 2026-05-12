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
    /// Title of the book.
    pub title: Option<String>,
    /// Title of the series the book is part of.
    pub series: Option<String>,
    /// Number of the book in the series.
    pub number: Option<String>,
    /// The total number of books in the series.
    ///
    /// The `Count` could be different on each book in a series. Consuming applications should consider using only the value for the latest book in the series.
    pub count: Option<usize>,
    /// Volume containing the book. Volume is a notion that is specific to US Comics, where the same series can have multiple volumes. Volumes can be referenced by number (1, 2, 3…) or by year (2018, 2020…).
    pub volume: Option<usize>,
    /// Quite specific to US comics, some books can be part of cross-over story arcs. This field can be used to specify an alternate series.
    pub alternate_series: Option<String>,
    /// Quite specific to US comics, some books can be part of cross-over story arcs. This fields can be used to specify an alternate series number.
    pub alternate_number: Option<String>,
    /// Quite specific to US comics, some books can be part of cross-over story arcs. Those fields can be used to specify an alternate series count of books.
    pub alternate_count: Option<usize>,
    /// A description or summary of the book.
    pub summary: Option<String>,
    /// A free text field, usually used to store information about the application that created the `ComicInfo.xml` file.
    pub notes: Option<String>,
    /// Usually contains the release year of the book.
    pub year: Option<usize>,
    /// Usually contains the release month of the book.
    pub month: Option<usize>,
    /// Usually contains the release day of the book.
    pub day: Option<usize>,

    // NOTE Creators fields
    // According to the schema, each creator element can only be present once. In order to cater for multiple creator with the same role, it is accepted that values are comma separated.
    //
    /// Person or organization responsible for creating the scenario.
    pub writer: Option<String>,
    /// Person or organization responsible for drawing the art.
    pub penciller: Option<String>,
    /// Person or organization responsible for inking the pencil art.
    pub inker: Option<String>,
    /// Person or organization responsible for applying color to drawings.
    pub colorist: Option<String>,
    /// Person or organization responsible for drawing text and speech bubbles.
    pub letterer: Option<String>,
    /// Person or organization responsible for drawing the cover art.
    pub cover_artist: Option<String>,
    /// A person or organization contributing to a resource by revising or elucidating the content, e.g., adding an introduction, notes, or other critical matter. An editor may also prepare a resource for production, publication, or distribution.
    pub editor: Option<String>,
    /// A person or organization who renders a text from one language into another, or from an older form of a language into the modern form.
    ///
    /// This can also be used for fan translations ("scanlator").
    pub translator: Option<String>,

    /// A person or organization responsible for publishing, releasing, or issuing a resource.
    pub publisher: Option<String>,
    /// An imprint is a group of publications under the umbrella of a larger imprint or a Publisher. For example, Vertigo is an Imprint of DC Comics.
    pub imprint: Option<String>,
    /// Genre of the book or series. For example, _Science-Fiction_ or _Shonen_.
    pub genre: Option<String>,
    /// Tags of the book or series. For example, _ninja_ or _school life_.
    pub tags: Option<String>,
    // TODO this should be an [`url::Url`] but versatility i guess.
    /// A URL pointing to a reference website for the book.
    ///
    /// It is accepted that multiple values are space separated.
    /// If a space is a part of the url it must be [percent encoded](https://datatracker.ietf.org/doc/html/rfc2396#section-2.4.1).
    pub web: Option<url::Url>,
    /// The number of pages in the book.
    pub page_count: Option<usize>,
    /// A language code describing the language of the book.
    ///
    /// Without any information on what kind of code this element is supposed to contain, it is recommended to use the [IETF BCP 47 language tag](https://en.wikipedia.org/wiki/IETF_language_tag), which can describe the language but also the script used. This helps to differentiate languages with multiple scripts, like Traditional and Simplified Chinese.
    ///
    /// See also:
    ///
    /// - [Choosing a language tag - W3C](https://www.w3.org/International/questions/qa-choosing-language-tags)
    /// - [Language subtag lookup app](https://r12a.github.io/app-subtags/)
    #[serde(rename = "LanguageISO")]
    pub language_iso: Option<String>,
    /// The original publication's binding format for scanned physical books or presentation format for digital sources.
    ///
    /// "TBP", "HC", "Web", "Digital" are common designators.
    pub format: Option<String>,
    /// Whether the book is in black and white.
    #[serde(with = "serde_from_to_str_enum")]
    pub black_and_white: Option<YesNo>,
    /// Whether the book is a manga. This also defines the reading direction as right-to-left when set to `YesAndRightToLeft`.
    #[serde(with = "serde_from_to_str_enum")]
    pub manga: Option<Manga>,
    /// Characters present in the book.
    ///
    /// It is accepted that multiple values are comma separated.
    pub characters: Option<String>,
    /// Teams present in the book. Usually refer to super-hero teams (e.g. Avengers).
    ///
    /// It is accepted that multiple values are comma separated.
    pub teams: Option<String>,
    /// Locations mentioned in the book.
    ///
    /// It is accepted that multiple values are comma separated.
    pub locations: Option<String>,
    /// A free text field, usually used to store information about who scanned the book.
    pub scan_information: Option<String>,
    /// The story arc that books belong to.
    ///
    /// For example, for [Undiscovered Country](https://comicvine.gamespot.com/undiscovered-country/4050-122630/), issues 1-6 are part of the _Destiny_ story arc, issues 7-12 are part of the _Unity_ story arc.
    pub story_arc: Option<String>,
    /// While [`StoryArc`](story-arc) was originally designed to store the arc _within a series_,
    /// it was often used to indicate that a book was part of a reading order,
    /// composed of books from multiple series.
    /// Mylar for instance was using the field as such.
    ///
    /// Since [`StoryArc`](story-arc) itself wasn't able to carry the information about ordering of books within a reading order,
    /// [`StoryArcNumber`](story-arc-number) was added.
    ///
    /// [`StoryArc`](story-arc) and [`StoryArcNumber`](story-arc-number) can work in combination,
    /// to indicate in which position the book is located at for a specific reading order.
    ///
    /// It is accepted that multiple values can be specified for both [`StoryArc`](story-arc) and [`StoryArcNumber`](story-arc-number).
    /// Multiple values are comma separated.
    ///
    /// [story-arc]: Self::story_arc
    /// [story-arc-number]: Self::story_arc_number
    pub story_arc_number: Option<String>,
    /// A group or collection the series belongs to.
    ///
    /// It is accepted that multiple values are comma separated.
    pub series_group: Option<String>,
    /// Age rating of the book.
    #[serde(with = "serde_from_to_str_enum")]
    pub age_rating: Option<AgeRating>,
    /// Describes each page of the book.
    #[builder(setter(each = "add_page"))]
    #[serde(with = "serde_pages", skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<ComicPageInfo>,
    /// Community rating of the book, from `0.0` to `5.0`.
    pub community_rating: Option<Rating>,
    /// Main character or team mentioned in the book.
    ///
    /// It is accepted that a single value should be present.
    pub main_character_or_team: Option<String>,
    /// Review of the book.
    pub review: Option<String>,
    /// A [Global Trade Item Number](https://en.wikipedia.org/wiki/Global_Trade_Item_Number) identifying the book. GTIN incorporates other standards like ISBN, ISSN, EAN, or JAN.
    #[serde(rename = "GTIN")]
    pub gtin: Option<String>,
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

mod serde_from_to_str_enum {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<V, S>(value: &Option<V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        V: ToString,
        S: Serializer,
    {
        value.as_ref().map(|v| v.to_string()).serialize(serializer)
    }
    pub fn deserialize<'de, V, D>(deserializer: D) -> Result<Option<V>, D::Error>
    where
        D: Deserializer<'de>,
        V: From<&'de str>,
    {
        let str_: &'de str = Deserialize::<'de>::deserialize(deserializer)?;
        Ok(Some(str_.into()))
    }
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
            val_xml.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?>", ""),
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
