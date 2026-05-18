use std::{
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
    path::{Component, Path, PathBuf},
};

use tar::Archive;

use crate::{SUPPORTED_IMAGES_TYPES, read::ComicBookReader};

fn normalize_current_dir<P: AsRef<Path>>(path: &P) -> PathBuf {
    path.as_ref()
        .to_path_buf()
        .components()
        .filter(|c| c != &Component::CurDir)
        .collect()
}

pub struct CbtReader<R>
where
    R: Read + Seek,
{
    inner_tar: Option<Archive<R>>,
    images: Vec<String>,
}

impl<R> CbtReader<R>
where
    R: Read + Seek,
{
    pub fn new(read: R) -> io::Result<Self> {
        Self::from_tar(Archive::new(read))
    }
    pub fn from_tar(mut tar: Archive<R>) -> io::Result<Self> {
        let images = tar
            .entries_with_seek()?
            .flat_map(|maybe_entry| -> Option<io::Result<String>> {
                match maybe_entry {
                    Ok(entry) => match entry.path() {
                        Ok(path) => {
                            let path = normalize_current_dir(&path);
                            // println!("{path:?}");
                            if path
                                .parent()
                                .filter(|e| !(e.as_os_str().is_empty() || e.as_os_str() == "."))
                                .is_none()
                                && path
                                    .extension()
                                    // .inspect(|d| println!("{d:?}"))
                                    .and_then(|d| d.to_str())
                                    // .inspect(|d| println!("{d}"))
                                    .is_some_and(|ext| SUPPORTED_IMAGES_TYPES.contains(&ext))
                            {
                                Some(Ok(path.file_name().and_then(|d| d.to_str())?.to_string()))
                            } else {
                                None
                            }
                        }
                        Err(err) => Some(Err(err)),
                    },
                    Err(_err) => Some(Err(_err)),
                }
            })
            .collect::<io::Result<Vec<String>>>()?;
        Ok(Self {
            inner_tar: Some(tar),
            images,
        })
    }
    pub fn into_inner_tar(self) -> Option<Archive<R>> {
        self.inner_tar
    }
}

impl CbtReader<BufReader<File>> {
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Self::new(BufReader::new(File::open(path)?))
    }
}

impl<R> ComicBookReader for CbtReader<R>
where
    R: Read + Seek,
{
    type Error = io::Error;

    fn pages_unordered(&self) -> Vec<String> {
        self.images.clone()
    }

    fn get_file(&mut self, file: &str) -> Result<Vec<u8>, Self::Error> {
        self.inner_tar = {
            let mut inne_read = self
                .inner_tar
                .take()
                .ok_or(io::Error::new(
                    io::ErrorKind::NotFound,
                    "inner tar not found",
                ))?
                .into_inner();
            inne_read.rewind()?;
            Some(Archive::new(inne_read))
        };
        for maybe_entry in self
            .inner_tar
            .as_mut()
            .ok_or(io::Error::new(
                io::ErrorKind::NotFound,
                "inner tar not found",
            ))?
            .entries_with_seek()?
        {
            let mut entry = maybe_entry?;
            let file_ = normalize_current_dir(&file);
            let entry_path = normalize_current_dir(&entry.path()?);

            if entry_path == file_ {
                let mut buf = Cursor::<Vec<u8>>::new({
                    if let Ok(size) = <u64 as TryInto<usize>>::try_into(entry.size()) {
                        Vec::with_capacity(size)
                    } else {
                        Vec::new()
                    }
                });
                io::copy(&mut entry, &mut buf)?;
                return Ok(buf.into_inner());
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("the file {} is not found", file),
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{no_order_images, ordered_2_images, ordered_images};
    use anyhow::anyhow;

    use super::*;
    #[test]
    fn test_ordered_read0() -> anyhow::Result<()> {
        let mut reader = CbtReader::from_path("test-data/archives/ordered.cbt")?;
        let images = reader.pages();
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
            let archive_buf = reader.get_page_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_page_by_index(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
    #[test]
    fn test_ordered_read_with_metadata() -> anyhow::Result<()> {
        let mut reader = CbtReader::from_path("test-data/archives/ordered.cbt")?;
        let images = reader.pages();
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
            let archive_buf = reader.get_page_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_page_by_index(index)? else {
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
        let mut reader = CbtReader::from_path("test-data/archives/md-test.cbt")?;
        let images = reader.pages_unordered();
        assert_eq!(&images, &{ no_order_images() });
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
            let archive_buf = reader.get_page_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_page_by_index_unordered(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
    #[test]
    fn test_ordered_2_read() -> anyhow::Result<()> {
        let mut reader = CbtReader::from_path("test-data/archives/ordered-2.cbt")?;
        let images = reader.pages();
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
            let archive_buf = reader.get_page_by_path(image_name)?;
            assert_eq!(&initial_file_buf, &archive_buf);
            // Test image index
            let Some(archive_buf) = reader.get_page_by_index(index)? else {
                return Err(anyhow!("There should be something at this index {index}"));
            };
            assert_eq!(&initial_file_buf, &archive_buf);
        }
        Ok(())
    }
}
