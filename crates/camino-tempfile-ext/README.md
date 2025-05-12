<!-- cargo-sync-rdme title [[ -->
# camino-tempfile-ext
<!-- cargo-sync-rdme ]] -->
[![crates.io](https://img.shields.io/crates/v/camino-tempfile-ext.svg?logo=rust)](https://crates.io/crates/camino-tempfile-ext)
[![Documentation (latest release)](https://img.shields.io/badge/docs-latest%20version-brightgreen.svg)](https://docs.rs/camino-tempfile-ext)
[![Documentation (main)](https://img.shields.io/badge/docs-main-purple.svg)](https://camino-rs.github.io/camino-tempfile/rustdoc/camino_tempfile_ext/)
[![License (Apache 2.0)](https://img.shields.io/badge/license-Apache-green.svg)](LICENSE-APACHE)
[![License (MIT)](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE-MIT)
<!-- cargo-sync-rdme rustdoc [[ -->
Quality-of-life extensions for [`camino-tempfile`].

camino-tempfile-ext provides utilities for:

* Creating files and directories within a [`Utf8TempDir`].
* Asserting on file and directory contents.

This crate is geared primarily towards testing and development, but it may
be of use in production environments as well.

## Examples

````rust
use camino_tempfile_ext::prelude::*;

// Create a temporary directory.
let dir = Utf8TempDir::new().unwrap();

// Create a nested file within this directory. Creation of intermediate
// directories is automatic.
let file = dir.child("foo/bar/baz.txt");
file.write_str("Hello, world!").unwrap();

// Assert on the file's contents (requires the assert feature)
file.assert("Hello, world!");
````

## Features

* **assert**: Enable assertions on file and directory contents. *Not enabled by default.*
* **assert-color**: Enable colored output for assertions: enables **assert**. *Not enabled by default.*

## Minimum supported Rust version (MSRV)

camino-tempfile-ext’s MSRV is **Rust 1.74**. At any time, at least the last
6 months of Rust releases will be supported.

## Credits

Portions of camino-tempfile-ext have been adapted from [`assert_fs`] (thank
you to the upstream maintainers!). If you need to work with
[`std::path::Path`](https://doc.rust-lang.org/nightly/std/path/struct.Path.html) rather than [`camino::Utf8Path`](https://docs.rs/camino/1.1.9/camino/struct.Utf8Path.html), check out
[`assert_fs`].

Upstream code is used under the terms of the MIT and Apache 2.0 licenses.

[`camino-tempfile`]: https://docs.rs/camino-tempfile/1.4.1/camino_tempfile/index.html
[`Utf8TempDir`]: https://docs.rs/camino-tempfile/1.4.1/camino_tempfile/dir/struct.Utf8TempDir.html
[`assert_fs`]: https://crates.io/crates/assert_fs
<!-- cargo-sync-rdme ]] -->

## License

This project is available under the terms of either the [Apache 2.0 license](LICENSE-APACHE) or the [MIT
license](LICENSE-MIT).
