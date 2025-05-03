// Copyright (c) The camino-tempfile Contributors
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Support for easy creation of temporary files and directories inside a
//! [`Utf8TempDir`].
//!
//! Creation of files inside temporary directories, especially ones inside
//! subdirectories, can be a bit bothersome to do by hand. This module provides
//! convenient ways to create these files.
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
//! // Now do something with the file...
//! # file.assert("Hello, world!");
//! ```
//!
//! [`Utf8TempDir`]: camino_tempfile::Utf8TempDir

mod child;
mod errors;
mod tools;

pub use child::*;
pub use errors::*;
pub use tools::*;
