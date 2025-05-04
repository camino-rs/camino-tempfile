// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use camino::{Utf8Path, Utf8PathBuf};
use camino_tempfile::Utf8TempDir;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

/// Access paths within a [`Utf8TempDir`] for testing.
///
/// See [`ChildPath`] trait implementations.
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
/// temp.close().unwrap();
/// ```
pub trait PathChild {
    /// Access a path within the temporary directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// println!("{}", temp.path());
    /// println!("{}", temp.child("foo/bar.txt").as_path());
    /// temp.close().unwrap();
    /// ```
    fn child<P: AsRef<Utf8Path>>(&self, path: P) -> ChildPath;
}

impl PathChild for Utf8TempDir {
    fn child<P: AsRef<Utf8Path>>(&self, path: P) -> ChildPath {
        ChildPath::new(self.path().join(path.as_ref()))
    }
}

impl PathChild for ChildPath {
    fn child<P: AsRef<Utf8Path>>(&self, path: P) -> ChildPath {
        ChildPath::new(self.as_path().join(path.as_ref()))
    }
}

/// A path within a temporary directory.
///
/// # Examples
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
///
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// temp.child("bar.txt").touch().unwrap();
///
/// temp.close().unwrap();
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChildPath {
    path: Utf8PathBuf,
}

impl ChildPath {
    /// Wrap a path for use with extension traits.
    ///
    /// See trait implementations or [`PathChild`] for more details.
    pub fn new<P: Into<Utf8PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    /// Access the path.
    pub fn as_path(&self) -> &Utf8Path {
        // Note: name is `as_path` to match `Deref` impl's `as_std_path`.
        &self.path
    }
}

impl AsRef<Utf8Path> for ChildPath {
    fn as_ref(&self) -> &Utf8Path {
        &self.path
    }
}

impl AsRef<Path> for ChildPath {
    fn as_ref(&self) -> &Path {
        self.path.as_ref()
    }
}

impl std::ops::Deref for ChildPath {
    type Target = Utf8Path;
    #[inline]
    fn deref(&self) -> &Utf8Path {
        &self.path
    }
}

macro_rules! impl_partial_eq {
    ($lhs:ty, $rhs: ty) => {
        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Utf8Path as PartialEq>::eq(self, other)
            }
        }
    };
}

impl_partial_eq!(ChildPath, Utf8Path);
impl_partial_eq!(ChildPath, &Utf8Path);
impl_partial_eq!(ChildPath, Utf8PathBuf);
impl_partial_eq!(ChildPath, Cow<'_, Utf8Path>);

macro_rules! impl_partial_eq_std_path {
    ($lhs:ty, $rhs: ty) => {
        impl PartialEq<$rhs> for $lhs {
            #[inline]
            fn eq(&self, other: &$rhs) -> bool {
                <Path as PartialEq>::eq(self.as_ref(), other)
            }
        }

        impl PartialEq<$lhs> for $rhs {
            #[inline]
            fn eq(&self, other: &$lhs) -> bool {
                <Path as PartialEq>::eq(self, other.as_ref())
            }
        }
    };
}

impl_partial_eq_std_path!(ChildPath, Path);
impl_partial_eq_std_path!(ChildPath, &Path);
impl_partial_eq_std_path!(ChildPath, PathBuf);
impl_partial_eq_std_path!(ChildPath, Cow<'_, Path>);
