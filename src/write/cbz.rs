use std::{
    io::{self, Cursor, Seek, Write},
    num::NonZeroUsize,
};

use zip::{ZipWriter, write::FileOptions};

use crate::write::ComicBookWriter;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum CbzWriterError {
    Zip(#[from] zip::result::ZipError),
    Image(#[from] image::ImageError),
    Io(#[from] std::io::Error),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CbzWriterImageFormat {
    #[default]
    Png,
    Jpeg,
    // TODO Avif
    // TODO Webp
}

impl From<CbzWriterImageFormat> for image::ImageFormat {
    fn from(value: CbzWriterImageFormat) -> Self {
        match value {
            CbzWriterImageFormat::Jpeg => Self::Jpeg,
            CbzWriterImageFormat::Png => Self::Png,
        }
    }
}

#[derive(Debug)]
pub struct CbzWriter<W>
where
    W: Write + Seek,
{
    zip_inner: ZipWriter<W>,
    count: usize,
    suffix: Option<String>,
    width: usize,
    images_format: CbzWriterImageFormat,
}

impl<W> CbzWriter<W>
where
    W: Write + Seek,
{
    /// Make a new CbzWriter from a writer.
    pub fn new(writer: W) -> Self {
        Self {
            zip_inner: ZipWriter::new(writer),
            count: 1,
            suffix: None,
            width: 4,
            images_format: Default::default(),
        }
    }
    /// Add a suffix to the page name.
    ///
    /// Please call this before adding any images to archive or your image archive "nomenclature" will break/.
    pub fn suffix(mut self, suffix: String) -> Self {
        self.suffix = Some(suffix);
        self
    }
    /// Remove suffixes.
    ///
    /// Please call this before adding any images to archive or your image archive "nomenclature" will break/.
    pub fn no_suffix(mut self) -> Self {
        let _ = self.suffix.take();
        self
    }
    /// Set the number width for the page name
    /// *aka the number length*.
    ///
    /// Default: __4__
    pub fn width(mut self, width: NonZeroUsize) -> Self {
        self.width = width.into();
        self
    }
    pub fn finish(self) -> zip::result::ZipResult<W> {
        self.zip_inner.finish()
    }
}

impl<W> ComicBookWriter for CbzWriter<W>
where
    W: Write + Seek,
{
    type Error = CbzWriterError;
    fn add_image(&mut self, image: image::DynamicImage) -> Result<(), Self::Error> {
        let width = self.width;
        self.zip_inner.start_file(
            format!(
                "{}{:0>width$}.{}",
                self.suffix.as_deref().unwrap_or(""),
                self.count,
                match self.images_format {
                    CbzWriterImageFormat::Jpeg => "jpg",
                    CbzWriterImageFormat::Png => "png",
                }
            ),
            FileOptions::DEFAULT,
        )?;
        let mut buf = Cursor::new(Vec::<u8>::new());
        image.write_to(&mut buf, self.images_format.into())?;
        io::copy(&mut buf, &mut self.zip_inner)?;
        self.zip_inner.flush()?;
        self.count += 1;
        Ok(())
    }
    fn add_image_with_format(
        &mut self,
        image: image::DynamicImage,
        _format: image::ImageFormat,
    ) -> Result<(), Self::Error> {
        if _format == image::ImageFormat::Gif {
            let width = self.width;
            self.zip_inner.start_file(
                format!(
                    "{}{:0>width$}.gif",
                    self.suffix.as_deref().unwrap_or(""),
                    self.count
                ),
                FileOptions::DEFAULT,
            )?;
            let mut buf = Cursor::new(Vec::<u8>::new());
            image.write_to(&mut buf, image::ImageFormat::Gif)?;
            io::copy(&mut buf, &mut self.zip_inner)?;
            self.zip_inner.flush()?;
            self.count += 1;
        } else {
            self.add_image(image)?;
        }
        Ok(())
    }
    fn add_file<P, R>(&mut self, path: P, mut file: R) -> Result<(), Self::Error>
    where
        P: AsRef<std::path::Path>,
        R: std::io::Read,
    {
        self.zip_inner
            .start_file_from_path(path, FileOptions::DEFAULT)?;
        io::copy(&mut file, &mut self.zip_inner)?;
        self.zip_inner.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::read_dir,
        io::{self, BufReader, BufWriter, Seek, Write},
        num::NonZero,
        path::Path,
    };

    use crate::{
        read::{ComicBookReader, cbz::CbzReader},
        write::ComicBookWriter,
    };

    use super::CbzWriter;
    #[test]
    fn test_write() -> anyhow::Result<()> {
        let to_import = read_dir("test-data/images/no-order")?.collect::<io::Result<Vec<_>>>()?;

        let mut file_to_use = tempfile::tempfile()?;

        {
            let mut writer = CbzWriter::new(BufWriter::new(&mut file_to_use))
                .width(NonZero::new(4).ok_or(anyhow::anyhow!("Unreachable"))?);
            for img in &to_import {
                writer.add_image(image::open(img.path())?)?;
            }
            writer.finish()?.flush()?;
        }

        file_to_use.rewind()?;

        {
            let reader = CbzReader::from_reader(BufReader::new(&mut file_to_use))?;
            let images = reader.images();
            assert_eq!(images.len(), to_import.len());
            for (index, image) in images.into_iter().enumerate() {
                assert_eq!(
                    format!("{:0>4}", index + 1).as_str(),
                    Path::new(&image)
                        .file_prefix()
                        .and_then(|d| d.to_str())
                        .ok_or(anyhow::anyhow!("No filename"))?
                );
            }
        }

        Ok(())
    }
}
