// Copyright (c) The camino-tempfile Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

#![warn(missing_docs)]
#![cfg_attr(doc_cfg, feature(doc_auto_cfg))]

//! Quality-of-life extensions for [`camino-tempfile`].
//!
//! camino-tempfile-ext provides utilities for:
//!
//! * Creating files and directories within a [`Utf8TempDir`].
//! * Asserting on file and directory contents.
//!
//! This crate is geared primarily towards testing and development, but it may
//! be of use in production environments as well.
//!
//! # Examples
//!
//! ```
//! use camino_tempfile_ext::prelude::*;
//!
//! // Create a temporary directory.
//! let dir = Utf8TempDir::new().unwrap();
//!
//! // Create a nested file within this directory. Creation of intermediate
//! // directories is automatic.
//! let file = dir.child("foo/bar/baz.txt");
//! file.write_str("Hello, world!").unwrap();
//!
//! // Assert on the file's contents (requires the assert feature)
//! # #[cfg(feature = "assert")]
//! file.assert("Hello, world!");
//! ```
//!
//! # Features
//!
//! - **assert**: Enable assertions on file and directory contents. *Not enabled by default.*
//! - **assert-color**: Enable colored output for assertions: enables **assert**. *Not enabled by default.*
//!
//! # Minimum supported Rust version (MSRV)
//!
//! camino-tempfile-ext's MSRV is **Rust 1.74**. At any time, at least the last
//! 6 months of Rust releases will be supported.
//!
//! # Credits
//!
//! Portions of camino-tempfile-ext have been adapted from [`assert_fs`] (thank
//! you to the upstream maintainers!). If you need to work with
//! [`std::path::Path`] rather than [`camino::Utf8Path`], check out
//! [`assert_fs`].
//!
//! Upstream code is used under the terms of the MIT and Apache 2.0 licenses.
//!
//! [`camino-tempfile`]: camino_tempfile
//! [`assert_fs`]: https://crates.io/crates/assert_fs
//! [`Utf8TempDir`]: camino_tempfile::Utf8TempDir

#[cfg(feature = "assert")]
pub mod assert;
#[cfg(feature = "assert")]
mod color;
pub mod fixture;

/// Extension traits and types that are useful to have available.
pub mod prelude {
    #[cfg(feature = "assert")]
    pub use crate::assert::PathAssert;
    pub use crate::fixture::{
        FileTouch, FileWriteBin, FileWriteFile, FileWriteStr, PathChild, PathCopy, PathCreateDir,
        SymlinkToDir, SymlinkToFile,
    };
    pub use camino_tempfile::{NamedUtf8TempFile, Utf8TempDir};
}

// Re-exports of public dependencies.
pub use camino_tempfile;
#[cfg(feature = "assert")]
pub use predicates_core;
