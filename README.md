# cbz

A Rust crate that allows you to read and write `cbz`, `cbt`, `cb7` files, with `ComicInfo.xml` metadata.

## Features flag

- `cbz`: support for `cbz` reader and writer
- `cbt`: support for `cbt` reader and writer
- `cb7`: support for `cb7` reader and writer
- `log`: logging...
- `comicinfo`: support for `ComicInfo.xml` metadata

None of these features is enabled by default.

## Examples

### Reading a `cbz` file

```rust
use cbz::read::{cbz::CbzReader, ComicBookReader};

fn main() {
    let mut cbz = CbzReader::from_path("fufuu-ijou-en-70.cbz").unwrap();
    /// get images names
    let pages = cbz.pages();
    for page in page {
      let file = cbz.get_page_by_path(&page).unwrap();
      /// DO whatever you want with it....
    }
}
```

### Writing a `cbz` file

```rust
use cbz::write::{cbz::CbzWriter, ComicBookWriter, ComicBookWriterExt};

use std::{io::{BufWriter}, fs::File};

fn main() {
    let mut my_file = CbzWriter::new(BufWriter::new(File::create("new.cbz")?));
    my_file.add_page_from_path("first.jpg")?;
    my_file.add_page_from_path("second.jpg")?;
    let _ = my_file.finish()?;
}
```

## Custom reader and writer implementations.

You can make your own reader via the `cbz::read::ComicBookReader` trait. For writer, check the `cbz::read::ComicBookWriter` trait.

## References and acknowledgement:

- [cbz pip](https://github.com/hyugogirubato/cbz) created by @hyugogirubato
- [anansi-project/comicinfo](https://github.com/anansi-project/comicinfo) for the `ComicInfo.xml` specification

## License

MIT or Apache 2.0 (at your choice)
