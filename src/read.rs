#[cfg(feature = "cb7")]
#[cfg_attr(docsrs, doc(cfg(feature = "cb7")))]
pub mod cb7;
#[cfg(feature = "cbt")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbt")))]
pub mod cbt;
#[cfg(feature = "cbz")]
#[cfg_attr(docsrs, doc(cfg(feature = "cbz")))]
pub mod cbz;

#[cfg(feature = "comicinfo")]
#[cfg_attr(docsrs, doc(cfg(feature = "comicinfo")))]
pub mod comicinfo;

/// Abstraction trait for reading comic book files
pub trait ComicBookReader {
    type Error;
    /// Get the list of images path as it is ordered from the archive
    fn pages_unordered(&self) -> Vec<String>;
    /// Get list of images paths
    ///
    /// This should return a [`Vec<String>`] of alphabetically (case insensitive) ordered of images path
    fn pages(&self) -> Vec<String> {
        let mut images = self.pages_unordered();
        images.sort_by_key(|s| s.to_lowercase());
        images
    }
    /// Get a file from the archive
    fn get_file(&mut self, file: &str) -> Result<Vec<u8>, Self::Error>;
    /// Get image by its path
    fn get_page_by_path(&mut self, image: &str) -> Result<Vec<u8>, Self::Error> {
        self.get_file(image)
    }
    /// Get image by its index
    ///
    /// This one uses [`Self::pages`] by default.
    ///
    /// Use [`Self::get_page_by_index_unordered`] if you want to use the [`Self::pages_unordered`]
    /// NOTE: The default implementation calls [`Self::images`] and uses [`[T]::get`] to get the image path.
    fn get_page_by_index(&mut self, index: usize) -> Result<Option<Vec<u8>>, Self::Error> {
        let images = self.pages();
        let Some(image_path) = images.get(index) else {
            return Ok(None);
        };
        Ok(Some(self.get_page_by_path(image_path)?))
    }

    /// Get image by its index
    ///
    /// This one uses [`Self::pages_unordered`] by default.
    fn get_page_by_index_unordered(
        &mut self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let images = self.pages_unordered();
        let Some(image_path) = images.get(index) else {
            return Ok(None);
        };
        Ok(Some(self.get_page_by_path(image_path)?))
    }
}
