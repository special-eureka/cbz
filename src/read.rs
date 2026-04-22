mod cbz;

/// Abstraction trait for writing comic book files
pub trait ComicBookReader {
    type Error;
    /// Get the list of images as it is ordered from the archive
    fn images_unordered(&self) -> Vec<String>;
    /// Get list of images paths
    ///
    /// This should be ordered alphabe
    fn images(&self) -> Vec<String> {
        let mut images = self.images_unordered();
        images.sort_by_key(|s| s.to_lowercase());
        images
    }
    /// Get a file from the archive
    fn get_file(&mut self, file: &str) -> Result<Vec<u8>, Self::Error>;
    /// Get image by its path
    fn get_image_by_path(&mut self, image: &str) -> Result<Vec<u8>, Self::Error> {
        self.get_file(image)
    }
    /// Get image by its index
    ///
    /// This one uses [`Self::images`] by default.
    ///
    /// Use [`Self::get_image_by_index_unordered`] if you want to use the [`Self::images_unordered`]
    /// NOTE: The default implementation calls [`Self::images`] and uses [`[T]::get`] to get the image path.
    fn get_image_by_index(&mut self, index: usize) -> Result<Option<Vec<u8>>, Self::Error> {
        let images = self.images();
        let Some(image_path) = images.get(index) else {
            return Ok(None);
        };
        Ok(Some(self.get_image_by_path(image_path)?))
    }
    fn get_image_by_index_unordered(
        &mut self,
        index: usize,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        let images = self.images_unordered();
        let Some(image_path) = images.get(index) else {
            return Ok(None);
        };
        Ok(Some(self.get_image_by_path(image_path)?))
    }
    // TODO comic info support
}

pub use cbz::CbzReader;
