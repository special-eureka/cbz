use std::{
    io::{self, Cursor, Seek, Write},
    num::NonZeroUsize,
};

use image::ImageFormat;
use zip::{ZipWriter, write::FileOptions};

#[cfg(feature = "comicinfo")]
use crate::comicinfo::{
    COMIC_INFO_XML, ComicInfoBuilder, ComicInfoBuilderError,
    comic_page_info::{ComicPageInfoBuilder, ComicPageInfoBuilderError},
};

use crate::write::ComicBookWriter;

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum CbzWriterError {
    Zip(#[from] zip::result::ZipError),
    Image(#[from] image::ImageError),
    Io(#[from] std::io::Error),
    #[error("the inner zip writer was dropped early")]
    ZipInnerDroppedEarly,
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    SerdeXml(#[from] serde_xml_rs::Error),
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    ComicInfoBuilder(#[from] ComicInfoBuilderError),
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    ComicPageInfoBuilder(#[from] ComicPageInfoBuilderError),
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    IntCast(#[from] std::num::TryFromIntError),
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

/// A generic cbz writer
///
/// Please look up to [`ComicBookWriter`] if you want more information on how to add pages.
///
/// Mostly a wrapper around [`zip::write::ZipWriter`]
#[derive(Debug)]
pub struct CbzWriter<W>
where
    W: Write + Seek,
{
    zip_inner: Option<ZipWriter<W>>,
    count: usize,
    suffix: Option<String>,
    width: usize,
    images_format: CbzWriterImageFormat,
    #[cfg(feature = "comicinfo")]
    comicinfo_builder: Option<ComicInfoBuilder>,
    #[cfg(feature = "comicinfo")]
    auto_double_page: bool,
}

impl<W> CbzWriter<W>
where
    W: Write + Seek,
{
    /// Make a new CbzWriter from a writer.
    pub fn new(writer: W) -> Self {
        Self {
            zip_inner: Some(ZipWriter::new(writer)),
            count: 1,
            suffix: None,
            width: 4,
            images_format: Default::default(),
            #[cfg(feature = "comicinfo")]
            comicinfo_builder: None,
            #[cfg(feature = "comicinfo")]
            auto_double_page: true,
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
    fn zip_inner_(&mut self) -> Result<&mut ZipWriter<W>, CbzWriterError> {
        self.zip_inner
            .as_mut()
            .ok_or(CbzWriterError::ZipInnerDroppedEarly)
    }
    /// Automatically set [`ComicPageInfo::double_page`] value on [`<Pages/>`](ComicInfo::pages).
    ///
    /// If this is enabled (which it is), every pages where its [`width`](image::DynamicImage::width) is bigger than its [`height`](image::DynamicImage::height) are set to `true`.
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    pub fn auto_double_page(mut self, auto_double_page: bool) -> Self {
        self.auto_double_page = auto_double_page;
        self
    }
    /// Finish the actual zip.
    ///
    /// It just call [ZipWriter::finish].
    pub fn finish(mut self) -> Result<W, CbzWriterError> {
        #[cfg(feature = "comicinfo")]
        {
            self.write_comicinfo()?;
        }
        let write = self
            .zip_inner
            .take()
            .ok_or(CbzWriterError::ZipInnerDroppedEarly)?
            .finish()?;
        Ok(write)
    }
    #[cfg(feature = "comicinfo")]
    fn write_comicinfo(&mut self) -> Result<(), CbzWriterError> {
        if let Some(comicinfo) = self.comicinfo_builder.take() {
            self.zip_inner_()?
                .start_file(COMIC_INFO_XML, FileOptions::DEFAULT)?;
            serde_xml_rs::to_writer(&mut self.zip_inner_()?, &(comicinfo.build()?))?;
            self.zip_inner_()?.flush()?;
        } else {
            #[cfg(feature = "log")]
            {
                log::warn!("no comic info found... moving on");
            }
        }
        Ok(())
    }
    /// Set a comicbook builder for this writer.
    ///
    /// It is worth noting that the `ComicInfo.xml` file coming out of this builder will be written
    ///
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    pub fn set_comicinfo_builder(mut self, builder: ComicInfoBuilder) -> Self {
        self.comicinfo_builder = Some(builder);
        self
    }
    /// [Take](Option::take) the internal [comicinfo builder](ComicInfoBuilder)
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    pub fn take_comicinfo_builder(&mut self) -> Option<ComicInfoBuilder> {
        self.comicinfo_builder.take()
    }
    /// Remove the internal [comicinfo builder](ComicInfoBuilder)
    ///
    /// If you want to take the builder, use [`Self::take_comicinfo_builder`];
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    pub fn unset_comicinfo_builder(mut self) -> Self {
        let _ = self.take_comicinfo_builder();
        self
    }
    /// Get the current comicinfo builder
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    pub fn get_comicinfo_builder(&mut self) -> Option<&ComicInfoBuilder> {
        self.comicinfo_builder.as_ref()
    }
    #[cfg(feature = "comicinfo")]
    fn get_current_page_info_builder(&self) -> ComicPageInfoBuilder {
        let mut builder = ComicPageInfoBuilder::default();
        builder.image(self.count);
        builder
    }
    fn inner_add_image(&mut self, add_page: AddPage) -> Result<(), CbzWriterError> {
        let (mut img_buf, filename): (Cursor<Vec<u8>>, String) = {
            let width = self.width;
            let mut buf = Cursor::new(Vec::<u8>::new());
            let name = if matches!(add_page._format, Some(ImageFormat::Gif)) {
                add_page.image.write_to(&mut buf, ImageFormat::Gif)?;
                format!(
                    "{}{:0>width$}.gif",
                    self.suffix.as_deref().unwrap_or(""),
                    self.count
                )
            } else {
                add_page
                    .image
                    .write_to(&mut buf, self.images_format.into())?;
                format!(
                    "{}{:0>width$}.{}",
                    self.suffix.as_deref().unwrap_or(""),
                    self.count,
                    match self.images_format {
                        CbzWriterImageFormat::Jpeg => "jpg",
                        CbzWriterImageFormat::Png => "png",
                    }
                )
            };
            (buf, name)
        };
        img_buf.rewind()?;
        self.zip_inner_()?
            .start_file(filename, FileOptions::DEFAULT)?;
        io::copy(&mut img_buf, self.zip_inner_()?)?;
        self.zip_inner_()?.flush()?;
        #[cfg(feature = "comicinfo")]
        {
            if self.comicinfo_builder.is_some() {
                let mut page_builder = self.get_current_page_info_builder();
                page_builder
                    .image(self.count)
                    .image_height(NonZeroUsize::new(add_page.image.height().try_into()?))
                    .image_width(NonZeroUsize::new(add_page.image.width().try_into()?));
                if self.auto_double_page {
                    page_builder.double_page(add_page.image.width() > add_page.image.height());
                }
                if let Some(bookmark) = add_page._bookmark {
                    page_builder.bookmark(bookmark);
                }
                if !add_page._page_type.is_empty() {
                    page_builder.type_(add_page._page_type);
                }
                if let Some(builder) = self.comicinfo_builder.as_mut() {
                    builder.add_page(page_builder.build()?);
                }
            }
        }

        self.count += 1;
        Ok(())
    }
}

struct AddPage {
    image: image::DynamicImage,
    _format: Option<image::ImageFormat>,
    #[cfg(feature = "comicinfo")]
    _page_type: Vec<crate::comicinfo::comic_page_type::ComicPageType>,
    #[cfg(feature = "comicinfo")]
    _bookmark: Option<String>,
}

impl<W> Drop for CbzWriter<W>
where
    W: Write + Seek,
{
    fn drop(&mut self) {
        #[cfg(feature = "comicinfo")]
        {
            let _a = self.write_comicinfo();
            #[cfg(feature = "log")]
            {
                let _ = _a.inspect_err(|err| {
                    log::error!("cannot write the `ComicInfo.xml` on drop : [{}]", err);
                });
            }
        }
    }
}

impl<W> ComicBookWriter for CbzWriter<W>
where
    W: Write + Seek,
{
    type Error = CbzWriterError;
    fn add_page(
        &mut self,
        image: image::DynamicImage,
        format: Option<image::ImageFormat>,
    ) -> Result<(), Self::Error> {
        self.inner_add_image(AddPage {
            image,
            _format: format,
            #[cfg(feature = "comicinfo")]
            _page_type: Vec::new(),
            #[cfg(feature = "comicinfo")]
            _bookmark: None,
        })
    }
    #[cfg(feature = "comicinfo")]
    #[cfg_attr(docsrs, doc(feature = "comicinfo"))]
    fn add_page_with_metadata(
        &mut self,
        image: image::DynamicImage,
        _format: Option<image::ImageFormat>,
        _page_type: Vec<crate::comicinfo::comic_page_type::ComicPageType>,
        _bookmark: Option<String>,
    ) -> Result<(), Self::Error> {
        self.inner_add_image(AddPage {
            image,
            _format,
            #[cfg(feature = "comicinfo")]
            _page_type,
            #[cfg(feature = "comicinfo")]
            _bookmark,
        })
    }
    fn add_file<P, R>(&mut self, path: P, mut file: R) -> Result<(), Self::Error>
    where
        P: AsRef<std::path::Path>,
        R: std::io::Read,
    {
        self.zip_inner_()?
            .start_file_from_path(path, FileOptions::DEFAULT)?;
        io::copy(&mut file, self.zip_inner_()?)?;
        self.zip_inner_()?.flush()?;
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
                writer.add_page(
                    image::open(img.path())?,
                    image::ImageFormat::from_path(img.path()).ok(),
                )?;
            }
            writer.finish()?.flush()?;
        }

        file_to_use.rewind()?;

        {
            let reader = CbzReader::from_reader(BufReader::new(&mut file_to_use))?;
            let images = reader.pages();
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
    #[cfg(feature = "comicinfo")]
    #[test]
    fn test_with_comic_info() -> anyhow::Result<()> {
        let to_import = read_dir("test-data/images/no-order")?.collect::<io::Result<Vec<_>>>()?;

        let mut file_to_use = tempfile::tempfile()?;

        let writed_comic_info = {
            use fake::{Fake, faker};

            use crate::comicinfo::ComicInfoBuilder;

            let mut writer = CbzWriter::new(BufWriter::new(&mut file_to_use))
                .set_comicinfo_builder({
                    let mut builer = ComicInfoBuilder::default();
                    builer
                        .title(faker::name::ja_jp::Title().fake())
                        .summary(faker::lorem::en::Paragraph(1..3).fake())
                        .series(faker::name::ja_jp::NameWithTitle().fake())
                        .writer(faker::name::ja_jp::Name().fake())
                        .colorist(faker::name::ja_jp::Name().fake())
                        .translator(faker::name::en::Name().fake())
                        .language_iso("en".into());
                    builer
                })
                .width(NonZero::new(4).ok_or(anyhow::anyhow!("Unreachable"))?);
            for img in &to_import {
                writer.add_page(
                    image::open(img.path())?,
                    image::ImageFormat::from_path(img.path()).ok(),
                )?;
            }
            let comic_info = writer.get_comicinfo_builder().cloned().unwrap().build()?;
            writer.finish()?.flush()?;
            comic_info
        };

        file_to_use.rewind()?;

        {
            use crate::read::comicinfo::GetComicInfo;

            let mut reader = CbzReader::from_reader(BufReader::new(&mut file_to_use))?;
            let images = reader.pages();
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
            assert_eq!(reader.get_comic_info()?, writed_comic_info);
        }

        Ok(())
    }
}
