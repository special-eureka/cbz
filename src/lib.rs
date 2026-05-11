#[cfg(feature = "comicinfo")]
#[cfg_attr(docsrs, doc(feature = "comicinfo"))]
pub mod comicinfo;
pub mod read;
#[cfg(test)]
pub(crate) mod test_utils;
pub mod write;

pub const SUPPORTED_IMAGES_TYPES: &[&str] = &["png", "jpg", "jpeg", "webp", "avif", "gif"];
