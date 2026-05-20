use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufReader, Cursor, Read, Seek},
    path::{Component, Path, PathBuf},
};

use crate::SUPPORTED_IMAGES_TYPES;

use super::ComicBookReader;

fn normalize_current_dir<P: AsRef<Path>>(path: &P) -> PathBuf {
    path.as_ref()
        .to_path_buf()
        .components()
        .filter(|c| c != &Component::CurDir)
        .collect()
}

use sevenz_rust2::{ArchiveReader, Error as S7Error, Password};
use tempfile::SpooledTempFile;

struct SolidArchivesTempEntries {
    entries: HashMap<String, SpooledTempFile>,
}

pub struct Cb7Reader<R>
where
    R: Read + Seek,
{
    inner_7z: ArchiveReader<R>,
    use_spooled_for_solid_archives: bool,
    temp_entries: Option<SolidArchivesTempEntries>,
    spooled_max_size: usize,
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum Cb7ReaderError {
    Io(#[from] io::Error),
    S7(#[from] S7Error),
    #[error("the inner spooled temp for solid archives is not found")]
    TempEntriesNotFound,
}

impl<R> Cb7Reader<R>
where
    R: Read + Seek,
{
    /// Create a [`Cb7Reader`] with an empty password
    pub fn new(read: R) -> Result<Self, S7Error> {
        Self::with_password(read, Password::empty())
    }
    /// Create a [`Cb7Reader`] with a password
    pub fn with_password(read: R, password: Password) -> Result<Self, S7Error> {
        Ok(Self::from_archive_reader(ArchiveReader::new(
            read, password,
        )?))
    }
    /// Create a [`Cb7Reader`] from a [`ArchiveReader`]
    pub fn from_archive_reader(seven_z: ArchiveReader<R>) -> Self {
        Self {
            inner_7z: seven_z,
            use_spooled_for_solid_archives: true,
            temp_entries: None,
            spooled_max_size: 1_048_576usize, // 1MB
        }
    }
    /// Get the inner 7z archive reader
    pub fn into_seven7z_inner(self) -> ArchiveReader<R> {
        self.inner_7z
    }
    /// Since solid archives are inefficent with [`ArchiveReader::read_file`],
    /// we use [`for_each_entries`](ArchiveReader::for_each_entries) on the first run of [`ComicBookReader::get_file`]
    /// and store each entry in a [`SpooledTempFile`],
    /// which will be used in next [`ComicBookReader::get_file`].
    ///
    /// This "parameter" will allow you to controll that. By default, it is `true`.
    ///
    /// Setting this to `false` will call [`ArchiveReader::read_file`] for both solid and non-solid archives.
    ///
    /// __This behavior only applies for solid archives, non solid archive will use the [`ArchiveReader::read_file`].__
    ///
    pub fn use_spooled_for_solid_archives(mut self, use_spooled_for_solid_archives: bool) -> Self {
        self.use_spooled_for_solid_archives = use_spooled_for_solid_archives;
        if !use_spooled_for_solid_archives {
            self.temp_entries.take();
        }
        self
    }
    /// Set the [`SpooledTempFile`] max_size of solid archives temp entries.
    ///
    /// By default, this is `1_048_576usize` _(roughly around 1MB)_
    ///
    /// See [`use_spooled_for_solid_archives`] for more information
    pub fn spooled_max_size(mut self, spooled_max_size: usize) -> Self {
        self.spooled_max_size = spooled_max_size;
        self
    }
    fn seed_temp_entries(&mut self) -> Result<(), Cb7ReaderError> {
        let max_size = self.spooled_max_size;
        let mut entries =
            HashMap::<String, SpooledTempFile>::with_capacity(self.inner_7z.archive().files.len());
        self.inner_7z.for_each_entries(|entry, read| {
            entries.entry(entry.name.clone()).insert_entry({
                let mut spooled = tempfile::spooled_tempfile(max_size);
                io::copy(read, &mut spooled)?;
                spooled
            });
            Ok(true)
        })?;
        self.temp_entries = Some(SolidArchivesTempEntries { entries });
        Ok(())
    }
}

impl Cb7Reader<BufReader<File>> {
    /// Open a `cb7` reader from a [`Path`]
    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, S7Error> {
        Self::new(BufReader::new(File::open(path)?))
    }
    /// Open a `cb7` reader from a [`Path`] with a password
    pub fn from_path_with_password<P: AsRef<Path>>(
        path: P,
        password: Password,
    ) -> Result<Self, S7Error> {
        Self::with_password(BufReader::new(File::open(path)?), password)
    }
}

impl<R> ComicBookReader for Cb7Reader<R>
where
    R: Read + Seek,
{
    type Error = Cb7ReaderError;
    fn pages_unordered(&self) -> Vec<String> {
        self.inner_7z
            .archive()
            .files
            .iter()
            .flat_map(|e| {
                let path = normalize_current_dir(&e.name);
                if path
                    .parent()
                    .filter(|e| !(e.as_os_str().is_empty() || e.as_os_str() == "."))
                    .is_none()
                    && path
                        .extension()
                        .and_then(|d| d.to_str())
                        .is_some_and(|ext| SUPPORTED_IMAGES_TYPES.contains(&ext))
                {
                    Some(path.file_name().and_then(|d| d.to_str())?.to_string())
                } else {
                    None
                }
            })
            .collect()
    }

    fn get_file(&mut self, file: &str) -> Result<Vec<u8>, Self::Error> {
        if self.use_spooled_for_solid_archives && self.inner_7z.archive().is_solid {
            if self.temp_entries.is_none() {
                self.seed_temp_entries()?;
            }
            let mut buf = Cursor::new({
                self.inner_7z
                    .archive()
                    .files
                    .iter()
                    .find(|a| a.name() == file)
                    .and_then(|e| Some(Vec::<u8>::with_capacity(e.size.try_into().ok()?)))
                    .unwrap_or_default()
            });
            let temp = self
                .temp_entries
                .as_mut()
                .ok_or(Cb7ReaderError::TempEntriesNotFound)?;

            let spool = temp
                .entries
                .get_mut(file)
                .ok_or(Cb7ReaderError::Io(io::Error::new(
                    io::ErrorKind::NotFound,
                    format!("the file `{file}` is not found"),
                )))?;
            spool.rewind()?;
            io::copy(spool, &mut buf)?;
            Ok(buf.into_inner())
        } else {
            Ok(self.inner_7z.read_file(file)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{no_order_images, ordered_2_images, ordered_images};
    use anyhow::anyhow;

    use super::*;
    #[test]
    fn test_ordered_read0() -> anyhow::Result<()> {
        let mut reader = Cb7Reader::from_path("test-data/archives/ordered.cb7")?;
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
        let mut reader = Cb7Reader::from_path("test-data/archives/ordered.cb7")?;
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
        let mut reader = Cb7Reader::from_path("test-data/archives/md-test.cb7")?;
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
        let mut reader = Cb7Reader::from_path("test-data/archives/ordered-2.cb7")?;
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
