pub mod read;
#[cfg(test)]
pub(crate) mod test_utils;

pub const SUPPORTED_IMAGES_TYPES: &[&str] = &["png", "jpg", "jpeg", "webp", "avif"];
