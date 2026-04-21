use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::Path,
};

use zip::{read::ZipArchive, result::ZipResult};

use crate::read::ComicBookReader;

/// A generic cbz reader
///
/// Mostly a wrapper around [`zip::read::ZipArchive`]
pub struct CbzReader<R> {
    inner_zip: ZipArchive<R>,
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
    pub fn from_path<P>(path: P) -> ZipResult<zip::read::ZipArchive<BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        zip::ZipArchive::new(BufReader::new(File::open(path)?))
    }
}

impl<R> ComicBookReader for CbzReader<R>
where
    R: Read + Seek,
{
    type Error = zip::result::ZipError;
    fn images(&self) -> Vec<String> {
        self.inner_zip
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
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect()
    }

    fn get_file(&mut self, file_path: &str) -> Result<Vec<u8>, Self::Error> {
        let mut maybe_file = BufReader::new(self.inner_zip.by_path(file_path)?);
        let mut buf = Vec::<u8>::with_capacity(maybe_file.get_ref().size() as _);
        io::copy(&mut maybe_file, &mut buf)?;
        buf.shrink_to_fit();
        Ok(buf)
    }
}
