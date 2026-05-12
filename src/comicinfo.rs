//! [`ComicInfo.xml`](https://github.com/anansi-project/comicinfo) implementation
//!
//! This is based out of the schema provided by [`anansi-project`](https://github.com/anansi-project).
//!
//! If `comicinfo` feature flag is enabled,
//! this crate readers and writers should automatically handle them.
//!
//! Only [`v2`](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd) is supported.
//!
pub mod age_rating;
pub mod comic_page_info;
pub mod comic_page_type;
pub mod manga;
pub mod rating;
pub mod yes_no;
