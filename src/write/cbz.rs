use std::io::BufWriter;

use zip::ZipWriter;

pub struct CbzWriter<W> {
    zip_inner: ZipWriter<BufWriter<W>>,
    num: usize,
}