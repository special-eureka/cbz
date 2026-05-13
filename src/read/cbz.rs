use std::{
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use zip::{
    read::{
        ZipArchive,
        // ZipFile,
        ZipFile
    },
    result::ZipResult,
};

use crate::read::ComicBookReader;

// pub struct CbzImagesIter<'a, R> {
//     images: std::vec::IntoIter<String>,
//     reader: &'a mut ZipArchive<R>,
// }

// impl<'a, R> Iterator for CbzImagesIter<'a, R>
// where
//     R: Read + Seek,
// {
//     type Item = ZipResult<ZipFile<'a, R>>;
//     fn next(&mut self) -> Option<Self::Item> {
//         let path = self.images.next()?;
//         Some(self.reader.by_path(path))
//     }
// }

/// A generic cbz reader
///
/// Mostly a wrapper around [`zip::read::ZipArchive`]
pub struct CbzReader<R> {
    inner_zip: ZipArchive<R>,
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[error(transparent)]
pub enum CbzReaderError {
    Zip(#[from] zip::result::ZipError),
    Io(#[from] io::Error),
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    Xml(#[from] serde_xml_rs::Error),
}

impl<R> CbzReader<R> {
    /// Get the underlying [`zip::read::ZipArchive`]
    pub fn into_inner(self) -> ZipArchive<R> {
        self.inner_zip
    }
    /// Construct a cbz reader
    pub fn from_zip_reader(zip: ZipArchive<R>) -> Self {
        Self { inner_zip: zip }
    }
    pub fn from_reader(reader: R) -> ZipResult<Self>
    where
        R: Read + Seek,
    {
        Ok(Self {
            inner_zip: ZipArchive::new(reader)?,
        })
    }
    pub fn get_zip_file(&mut self,)
}

impl CbzReader<BufReader<File>> {
    pub fn from_path<P>(path: P) -> ZipResult<Self>
    where
        P: AsRef<Path>,
    {
        let zip = zip::ZipArchive::new(BufReader::new(File::open(path)?))?;
        Ok(CbzReader { inner_zip: zip })
    }
}

// TODO impl
// impl<R> CbzReader<R>
// where
//     R: Read + Seek,
// {
//     pub fn iter_images(&mut self) -> CbzImagesIter<'_, R> {
//         CbzImagesIter {
//             images: self.images().into_iter(),
//             reader: &mut self.inner_zip,
//         }
//     }
// }

impl<R> ComicBookReader for CbzReader<R>
where
    R: Read + Seek,
{
    type Error = CbzReaderError;
    fn images_unordered(&self) -> Vec<String> {
        let mut images = self
            .inner_zip
            .file_names()
            .filter_map(|e| -> Option<String> {
                let e_p = Path::new(e);
                let e_p_extension = e_p.extension()?.to_str()?;
                if crate::SUPPORTED_IMAGES_TYPES.contains(&e_p_extension) {
                    e_p.file_name().and_then(|d| d.to_str().map(String::from))
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        images.dedup();
        images
    }

    fn get_file(&mut self, file_path: &str) -> Result<Vec<u8>, Self::Error> {
        let mut maybe_file = BufReader::new(self.inner_zip.by_path(file_path)?);
        let mut buf = Vec::<u8>::with_capacity(maybe_file.get_ref().size() as _);
        io::copy(&mut maybe_file, &mut buf)?;
        buf.shrink_to_fit();
        Ok(buf)
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::{self, BufReader},
    };

    use anyhow::anyhow;

    use crate::{
        read::ComicBookReader,
        test_utils::{no_order_images, ordered_2_images, ordered_images},
    };

    use super::CbzReader;

    #[test]
    fn test_ordered_read() -> anyhow::Result<()> {
        let mut reader = CbzReader::from_path("test-data/archives/archived-ordered.cbz")?;
        let images = reader.images();
        assert_eq!(&images, &ordered_images());
        for (index, image_name) in images.iter().enumerate() {
            let initial_file_buf = {
                let mut buf = Vec::<u8>::new();
                let mut reader = BufReader::new(File::open(format!(
                    "test-data/images/ordered/{image_name}"
                ))?);
                io::copy(&mut reader, &mut buf)?;
                buf
            };
            // Test images path
            let archive_buf = reader.get_image_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_image_by_index(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
    #[test]
    fn test_ordered_read_with_metadata() -> anyhow::Result<()> {
        let mut reader =
            CbzReader::from_path("test-data/archives/archived-ordered-with-metadata.cbz")?;
        let images = reader.images();
        assert_eq!(&images, &ordered_images());
        for (index, image_name) in images.iter().enumerate() {
            let initial_file_buf = {
                let mut buf = Vec::<u8>::new();
                let mut reader = BufReader::new(File::open(format!(
                    "test-data/images/ordered/{image_name}"
                ))?);
                io::copy(&mut reader, &mut buf)?;
                buf
            };
            // Test images path
            let archive_buf = reader.get_image_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_image_by_index(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        assert!(reader.get_file("test-metadata.txt").is_ok());
        assert!(reader.get_file("nothingasdasdasdasdasd.cbor").is_err());
        Ok(())
    }
    #[test]
    fn test_no_order_read() -> anyhow::Result<()> {
        let mut reader = CbzReader::from_path("test-data/archives/md-test.cbz")?;
        let images = reader.images_unordered();
        assert_eq!(&images, &no_order_images());
        for (index, image_name) in images.iter().enumerate() {
            let initial_file_buf = {
                let mut buf = Vec::<u8>::new();
                let mut reader = BufReader::new(File::open(format!(
                    "test-data/images/no-order/{image_name}"
                ))?);
                io::copy(&mut reader, &mut buf)?;
                buf
            };
            // Test images path
            let archive_buf = reader.get_image_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_image_by_index_unordered(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
    #[test]
    fn test_ordered_2_read() -> anyhow::Result<()> {
        let mut reader = CbzReader::from_path("test-data/archives/ordered-2.cbz")?;
        let images = reader.images();
        assert_eq!(&images, &ordered_2_images());
        for (index, image_name) in images.iter().enumerate() {
            let initial_file_buf = {
                let mut buf = Vec::<u8>::new();
                let mut reader = BufReader::new(File::open(format!(
                    "test-data/images/ordered-2/{image_name}"
                ))?);
                io::copy(&mut reader, &mut buf)?;
                buf
            };
            // Test images path
            let archive_buf = reader.get_image_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_image_by_index(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
}
