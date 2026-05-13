use std::io::Cursor;

use super::ComicBookReader;
use crate::comicinfo::ComicInfo;

pub trait GetComicInfo {
    type Error;
    fn get_comic_info(&mut self) -> Result<ComicInfo, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
#[error(transparent)]
pub enum GetComicInfoDefaultReaderError<R> {
    SerdeXml(#[from] serde_xml_rs::Error),
    Reader(R),
}

impl<R> GetComicInfo for R
where
    R: ComicBookReader,
{
    type Error = GetComicInfoDefaultReaderError<<R as super::ComicBookReader>::Error>;
    fn get_comic_info(&mut self) -> Result<ComicInfo, <Self as GetComicInfo>::Error> {
        let file = self
            .get_file("ComicInfo.xml")
            .map_err(GetComicInfoDefaultReaderError::Reader)?;
        Ok(serde_xml_rs::from_reader(Cursor::new(file))?)
    }
}
