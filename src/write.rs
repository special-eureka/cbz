pub mod cbz;

use std::{io::Read, path::Path};

pub trait ComicBookWriter {
    type Error;
    fn add_image(&mut self, image: image::DynamicImage) -> Result<(), Self::Error>;
    fn add_file<P, R>(&mut self, path: P, file: R) -> Result<(), Self::Error>
    where
        P: AsRef<Path>,
        R: Read;
}
