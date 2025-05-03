# camino-tempfile

![License: MIT OR Apache-2.0](https://img.shields.io/crates/l/camino-tempfile.svg?)
[![Rust: ^1.74.0](https://img.shields.io/badge/rust-^1.74.0-93450a.svg?logo=rust)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)

This repository contains the source code for temporary file and directory management with [camino](https://docs.rs/camino).

* [**camino-tempfile**](crates/camino-tempfile): Temporary files and directories that return [`Utf8Path`] instances; a wrapper around [tempfile].
* [**camino-tempfile-ext**](crates/camino-tempfile-ext): Quality-of-life extensions for camino-tempfile: easily create and assert on contents within tests and elsewhere.

For more about these crates, see the above links.

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).

camino-tempfile contains code adapted from [`tempfile`]. `tempfile` is copyright (c)
The tempfile Contributors. Code used under the terms of the MIT and Apache 2.0
licenses.

camino-tempfile-ext contains code adapted from [`assert_fs`]. `assert_fs` is
copyright (c) The assert_fs Contributors. Code used under the terms of the MIT
and Apache 2.0 licenses.

[`Utf8Path`]: https://docs.rs/camino/latest/camino/struct.Utf8Path.html
[`tempfile`]: https://docs.rs/tempfile
[`assert_fs`]: https://docs.rs/assert_fs
