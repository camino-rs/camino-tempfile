// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Filesystem assertions.
//!
//! See [`PathAssert`].
//!
//! # Examples
//!
//! ```rust
//! use camino_tempfile_ext::prelude::*;
//! use predicates::prelude::*;
//!
//! let temp = Utf8TempDir::new().unwrap();
//! let input_file = temp.child("foo.txt");
//! input_file.touch().unwrap();
//!
//! // ... do something with input_file ...
//!
//! input_file.assert("");
//! temp.child("bar.txt").assert(predicate::path::missing());
//!
//! temp.close().unwrap();
//! ```

use crate::{color::Palette, fixture};
#[cfg(feature = "assert-color")]
use anstream::panic;
use camino::Utf8Path;
use camino_tempfile::{NamedUtf8TempFile, Utf8TempDir};
use predicates::{
    path::PredicateFileContentExt, reflection::PredicateReflection, str::PredicateStrExt,
};
use predicates_core::Predicate;
use predicates_tree::CaseTreeExt;
use std::{fmt, path::Path};

/// Assert the state of files within a [`Utf8TempDir`].
///
/// This uses [`IntoUtf8PathPredicate`] to provide short-hands for common cases,
/// accepting:
///
/// - `Predicate<Utf8Path>` or `Predicate<Path>` for validating a path.
/// - `Predicate<str>` for validating the content of the file.
/// - `&[u8]` or `&str` representing the content of the file.
///
/// Note that both `Predicate<Utf8Path>` and `Predicate<Path>` (such as those in
/// [`predicates::path`]) can be used for validating paths.
///
/// See [`predicates`] for more predicates.
///
/// # Examples
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
/// use predicates::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert("");
/// temp.child("bar.txt").assert(predicate::path::missing());
///
/// temp.close().unwrap();
/// ```
pub trait PathAssert {
    /// Assert the state of files within a [`Utf8TempDir`].
    ///
    /// This uses [`IntoUtf8PathPredicate`] to provide short-hands for common cases,
    /// accepting:
    ///
    /// - `Predicate<Path>` for validating a path.
    /// - `Predicate<str>` for validating the content of the file.
    /// - `&[u8]` or `&str` representing the content of the file.
    ///
    /// Note that accepted predicates are of type `Predicate<Path>`, not
    /// `Predicate<Utf8Path>`, so that predicates from [`predicates::path`] can be
    /// used.
    ///
    /// See [`predicates`] for more predicates.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    /// use predicates::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// let input_file = temp.child("foo.txt");
    /// input_file.touch().unwrap();
    ///
    /// // ... do something with input_file ...
    ///
    /// input_file.assert("");
    /// temp.child("bar.txt").assert(predicate::path::missing());
    ///
    /// temp.close().unwrap();
    /// ```
    #[track_caller]
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoUtf8PathPredicate<P>,
        P: Predicate<Utf8Path>;
}

impl PathAssert for Utf8TempDir {
    #[track_caller]
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoUtf8PathPredicate<P>,
        P: Predicate<Utf8Path>,
    {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for NamedUtf8TempFile {
    #[track_caller]
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoUtf8PathPredicate<P>,
        P: Predicate<Utf8Path>,
    {
        assert(self.path(), pred);
        self
    }
}

impl PathAssert for fixture::ChildPath {
    #[track_caller]
    fn assert<I, P>(&self, pred: I) -> &Self
    where
        I: IntoUtf8PathPredicate<P>,
        P: Predicate<Utf8Path>,
    {
        assert(self.path(), pred);
        self
    }
}

#[track_caller]
fn assert<I, P>(path: &Utf8Path, pred: I)
where
    I: IntoUtf8PathPredicate<P>,
    P: Predicate<Utf8Path>,
{
    let pred = pred.into_path();
    if let Some(case) = pred.find_case(false, path) {
        let palette = Palette::color();
        panic!(
            "Unexpected file, failed {:#}\n{:#}={:#}",
            case.tree(),
            palette.key("path"),
            palette.value(path)
        );
    }
}

/// Converts a type into the needed [`Predicate<Utf8Path>`].
///
/// # Examples
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
/// use predicates::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
///
/// // ... do something with input_file ...
///
/// temp.child("bar.txt").assert(predicate::path::missing()); // Uses IntoUtf8PathPredicate
///
/// temp.close().unwrap();
/// ```
pub trait IntoUtf8PathPredicate<P>
where
    P: Predicate<Utf8Path>,
{
    /// The type of the predicate being returned.
    type Predicate;

    /// Convert to a predicate for testing a path.
    fn into_path(self) -> P;
}

impl<P> IntoUtf8PathPredicate<P> for P
where
    P: Predicate<Utf8Path>,
{
    type Predicate = P;

    fn into_path(self) -> Self::Predicate {
        self
    }
}

/// Adapter used by [`IntoUtf8PathPredicate`] for static byte slices.
///
/// # Example
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert(b""); // uses BytesContentPathPredicate
///
/// temp.close().unwrap();
/// ```
#[derive(Debug)]
pub struct BytesContentPathPredicate(
    predicates::path::FileContentPredicate<predicates::ord::EqPredicate<&'static [u8]>>,
);

impl BytesContentPathPredicate {
    pub(crate) fn new(value: &'static [u8]) -> Self {
        let pred = predicates::ord::eq(value).from_file_path();
        BytesContentPathPredicate(pred)
    }
}

impl PredicateReflection for BytesContentPathPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl Predicate<Utf8Path> for BytesContentPathPredicate {
    fn eval(&self, item: &Utf8Path) -> bool {
        self.0.eval(item.as_std_path())
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Utf8Path,
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable.as_std_path())
    }
}

impl fmt::Display for BytesContentPathPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoUtf8PathPredicate<BytesContentPathPredicate> for &'static [u8] {
    type Predicate = BytesContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

impl<const N: usize> IntoUtf8PathPredicate<BytesContentPathPredicate> for &'static [u8; N] {
    type Predicate = BytesContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

/// Adapter used by [`IntoUtf8PathPredicate`] for `str` and `String`.
///
/// # Example
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert(""); // Uses StrContentPathPredicate
///
/// temp.close().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct StrContentPathPredicate(
    predicates::path::FileContentPredicate<
        predicates::str::Utf8Predicate<predicates::str::DifferencePredicate>,
    >,
);

impl StrContentPathPredicate {
    pub(crate) fn new(value: String) -> Self {
        let pred = predicates::str::diff(value).from_utf8().from_file_path();
        StrContentPathPredicate(pred)
    }
}

impl predicates_core::reflection::PredicateReflection for StrContentPathPredicate {
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl Predicate<Utf8Path> for StrContentPathPredicate {
    fn eval(&self, item: &Utf8Path) -> bool {
        self.0.eval(item.as_std_path())
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Utf8Path,
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable.as_std_path())
    }
}

impl fmt::Display for StrContentPathPredicate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl IntoUtf8PathPredicate<StrContentPathPredicate> for String {
    type Predicate = StrContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

impl IntoUtf8PathPredicate<StrContentPathPredicate> for &str {
    type Predicate = StrContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self.to_owned())
    }
}

impl IntoUtf8PathPredicate<StrContentPathPredicate> for &String {
    type Predicate = StrContentPathPredicate;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self.to_owned())
    }
}

/// Adapter used by [`IntoUtf8PathPredicate`] for `Predicate<str>` instances,
/// such as those in [`predicates::str`].
///
/// # Example
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
/// use predicates::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert(predicate::str::is_empty()); // Uses StrPathPredicate
///
/// temp.close().unwrap();
/// ```
#[derive(Debug, Clone)]
pub struct StrPathPredicate<P: Predicate<str>>(
    predicates::path::FileContentPredicate<predicates::str::Utf8Predicate<P>>,
);

impl<P> StrPathPredicate<P>
where
    P: Predicate<str>,
{
    pub(crate) fn new(value: P) -> Self {
        let pred = value.from_utf8().from_file_path();
        StrPathPredicate(pred)
    }
}

impl<P> PredicateReflection for StrPathPredicate<P>
where
    P: Predicate<str>,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl<P> Predicate<Utf8Path> for StrPathPredicate<P>
where
    P: Predicate<str>,
{
    fn eval(&self, item: &Utf8Path) -> bool {
        self.0.eval(item.as_std_path())
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Utf8Path,
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable.as_std_path())
    }
}

impl<P> fmt::Display for StrPathPredicate<P>
where
    P: Predicate<str>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<P> IntoUtf8PathPredicate<StrPathPredicate<P>> for P
where
    P: Predicate<str>,
{
    type Predicate = StrPathPredicate<P>;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

/// Adapter used by [`IntoUtf8PathPredicate`] for `Predicate<Path>` instances,
/// such as those in [`predicates::path`].
///
/// # Example
///
/// ```rust
/// use camino_tempfile_ext::prelude::*;
/// use predicates::prelude::*;
///
/// let temp = Utf8TempDir::new().unwrap();
/// let input_file = temp.child("foo.txt");
/// input_file.touch().unwrap();
///
/// // ... do something with input_file ...
///
/// input_file.assert(predicate::path::exists()); // Uses PathPredicate
///
/// temp.close().unwrap();
/// ```
pub struct PathPredicate<P: Predicate<Path>>(P);

impl<P> PathPredicate<P>
where
    P: Predicate<Path>,
{
    pub(crate) fn new(predicate: P) -> Self {
        Self(predicate)
    }
}

impl<P> PredicateReflection for PathPredicate<P>
where
    P: Predicate<Path>,
{
    fn parameters<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Parameter<'a>> + 'a> {
        self.0.parameters()
    }

    /// Nested `Predicate`s of the current `Predicate`.
    fn children<'a>(
        &'a self,
    ) -> Box<dyn Iterator<Item = predicates_core::reflection::Child<'a>> + 'a> {
        self.0.children()
    }
}

impl<P> Predicate<Utf8Path> for PathPredicate<P>
where
    P: Predicate<Path>,
{
    fn eval(&self, item: &Utf8Path) -> bool {
        self.0.eval(item.as_std_path())
    }

    fn find_case<'a>(
        &'a self,
        expected: bool,
        variable: &Utf8Path,
    ) -> Option<predicates_core::reflection::Case<'a>> {
        self.0.find_case(expected, variable.as_std_path())
    }
}

impl<P> fmt::Display for PathPredicate<P>
where
    P: Predicate<Path>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<P> IntoUtf8PathPredicate<PathPredicate<P>> for P
where
    P: Predicate<Path>,
{
    type Predicate = PathPredicate<P>;

    fn into_path(self) -> Self::Predicate {
        Self::Predicate::new(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use predicates::prelude::*;

    // Since IntoUtf8PathPredicate exists solely for conversion, test it under that scenario to ensure
    // it works as expected.
    fn convert_path<I, P>(pred: I) -> P
    where
        I: IntoUtf8PathPredicate<P>,
        P: Predicate<Utf8Path>,
    {
        pred.into_path()
    }

    #[test]
    fn into_utf8_path_from_pred() {
        let pred = convert_path(predicate::eq(Utf8Path::new("hello.md")));
        let case = pred.find_case(false, Utf8Path::new("hello.md"));
        println!("Failing case: {case:?}");
        assert!(case.is_none());
    }

    #[test]
    fn into_utf8_path_from_bytes() {
        let pred = convert_path(b"hello\n" as &[u8]);
        let case = pred.find_case(false, Utf8Path::new("tests/fixture/hello.txt"));
        println!("Failing case: {case:?}");
        assert!(case.is_none());
    }

    #[test]
    fn into_utf8_path_from_str() {
        let pred = convert_path("hello\n");
        let case = pred.find_case(false, Utf8Path::new("tests/fixture/hello.txt"));
        println!("Failing case: {case:?}");
        assert!(case.is_none());
    }

    #[test]
    fn into_utf8_path_from_path() {
        let pred = convert_path(predicate::path::missing());
        let case = pred.find_case(false, Utf8Path::new("tests/fixture/missing.txt"));
        println!("Failing case: {case:?}");
        assert!(case.is_none());
    }
}
