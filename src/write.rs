#[cfg(feature = "cb7")]
#[cfg_attr(docsrs, doc(cfg(feature = "cb7")))]
pub mod cb7;
#[cfg(feature = "cbt")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbt")))]
pub mod cbt;
#[cfg(feature = "cbz")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbz")))]
pub mod cbz;

use std::{io::Read, path::Path};

pub trait ComicBookWriter {
    type Error;
    /// Add a page at the Comic book archive.
    ///
    /// The additional [`image::ImageFormat`] argument is the initial format the image. (Since we can't extract format via [`image::DynamicImage`]).
    /// There are some edge cases where you might need this. (Like handling GIF images for example)
    ///
    /// If the `comicinfo` feature flag is enabled,
    /// all writers should normally provide the [`Image`](crate::comicinfo::comic_page_info::ComicPageInfo::image),
    /// [`ImageWidth`](crate::comicinfo::comic_page_info::ComicPageInfo::image_width),
    /// [`ImageHeight`](crate::comicinfo::comic_page_info::ComicPageInfo::image_height) attributes on `ComicInfo` `Page` elements.
    /// The [`DoublePage`](crate::comicinfo::comic_page_info::ComicPageInfo::double_page) attributes depends on writers implementation
    ///
    fn add_page(
        &mut self,
        image: image::DynamicImage,
        _format: Option<image::ImageFormat>,
    ) -> Result<(), Self::Error>;
    /// Add an additional file to the archive
    ///
    /// You might need if you want
    ///
    fn add_file<P, R>(&mut self, path: P, file: R) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        R: Read;

    /// Similiar to [`Self::add_page`] but will allow you to specify some `ComixInfo.xml` page type, and bookmark.
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    fn add_page_with_metadata(
        &mut self,
        image: image::DynamicImage,
        _format: Option<image::ImageFormat>,
        _page_type: Vec<crate::comicinfo::comic_page_type::ComicPageType>,
        _bookmark: Option<String>,
    ) -> Result<(), Self::Error> {
        self.add_page(image, _format)
    }
}
