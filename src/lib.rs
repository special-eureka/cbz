#![cfg_attr(docsrs, feature(doc_cfg))]

//! # cbz
//!
//! A Rust crate that allows you to read and write `cbz`, `cbt`, `cb7` files, with `ComicInfo.xml` metadata.
//!
//! For readers, please check the [`cbz::read`] module.
//!
//! For writers, please check the [`cbz::write`] module.
//!
#[cfg(feature = "comicinfo")]
#[cfg_attr(docsrs, doc(feature = "comicinfo"))]
pub mod comicinfo;
pub mod read;
#[cfg(test)]
pub(crate) mod test_utils;
pub mod write;

pub const SUPPORTED_IMAGES_TYPES: &[&str] = &["png", "jpg", "jpeg", "webp", "avif", "gif"];
