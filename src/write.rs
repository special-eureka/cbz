#[cfg(feature = "cbz")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbz")))]
pub mod cbz;

use std::{io::Read, path::Path};

pub trait ComicBookWriter {
    type Error;
    /// Add a page at the Comic book archive
    ///
    /// If the `comicinfo` feature flag is enabled,
    /// all writers should normally provide the [`Image`](crate::comicinfo::comic_page_info::image),
    /// [`ImageWidth`](crate::comicinfo::comic_page_info::image_width),
    /// [`ImageHeight`](crate::comicinfo::comic_page_info::image_height) attributes on `ComicInfo` `Page` elements.
    fn add_page(&mut self, image: image::DynamicImage) -> Result<(), Self::Error>;
    /// Similar to the [`Self::add_image`]
    /// but take additional [`image::ImageFormat`] as an argument which is the initial format the image. (Since we can't extract format via [`image::DynamicImage`]).
    ///
    /// The default implementation just call [`Self::add_image`] but there are some edge cases where you might need this. (Like handling GIF images for example)
    fn add_page_with_format(
        &mut self,
        image: image::DynamicImage,
        _format: image::ImageFormat,
    ) -> Result<(), Self::Error> {
        self.add_page(image)
    }
    /// Add an additional file to the archive
    fn add_file<P, R>(&mut self, path: P, file: R) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        R: Read;

    /// Similiar to [`Self::add_image_with_format`] but will allow you add
    ///
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    fn add_page_with_page_type(
        &mut self,
        image: image::DynamicImage,
        _format: image::ImageFormat,
        _page_type: crate::comicinfo::comic_page_type::ComicPageType,
        _doublepage: bool,
    ) -> Result<(), Self::Error> {
        self.add_page(image)
    }
}
