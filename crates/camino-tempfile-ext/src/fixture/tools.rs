// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::{ChildPath, FixtureError, FixtureKind, ResultChainExt};
use camino::Utf8Path;
use camino_tempfile::{NamedUtf8TempFile, Utf8TempDir};
use globwalk::GlobWalkerBuilder;
use std::{fs, io::Write, path::Path};

/// Create empty directories at [`ChildPath`].
pub trait PathCreateDir {
    /// Create an empty directory at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp.child("subdir").create_dir_all().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    fn create_dir_all(&self) -> Result<(), FixtureError>;
}

impl PathCreateDir for ChildPath {
    fn create_dir_all(&self) -> Result<(), FixtureError> {
        create_dir_all(self.path())
    }
}

/// Create empty files at [`ChildPath`].
///
pub trait FileTouch {
    /// Create an empty file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp.child("foo.txt").touch().unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    fn touch(&self) -> Result<(), FixtureError>;
}

impl FileTouch for ChildPath {
    fn touch(&self) -> Result<(), FixtureError> {
        touch(self.path())
    }
}

impl FileTouch for NamedUtf8TempFile {
    fn touch(&self) -> Result<(), FixtureError> {
        touch(self.path())
    }
}

/// Write a binary file at [`ChildPath`].
///
pub trait FileWriteBin {
    /// Write a binary file at [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp
    ///     .child("foo.txt")
    ///     .write_binary(b"To be or not to be...")
    ///     .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError>;
}

impl FileWriteBin for ChildPath {
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError> {
        write_binary(self.path(), data)
    }
}

impl FileWriteBin for NamedUtf8TempFile {
    fn write_binary(&self, data: &[u8]) -> Result<(), FixtureError> {
        write_binary(self.path(), data)
    }
}

/// Write a text file at a [`ChildPath`] or [`NamedUtf8TempFile`].
pub trait FileWriteStr {
    /// Write a text file at [`ChildPath`] or [`NamedUtf8TempFile`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_str("To be or not to be...")
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    fn write_str(&self, data: &str) -> Result<(), FixtureError>;
}

impl FileWriteStr for ChildPath {
    fn write_str(&self, data: &str) -> Result<(), FixtureError> {
        write_str(self.path(), data)
    }
}

impl FileWriteStr for NamedUtf8TempFile {
    fn write_str(&self, data: &str) -> Result<(), FixtureError> {
        write_str(self.path(), data)
    }
}

/// Write (copy) a file to [`ChildPath`].
pub trait FileWriteFile {
    /// Write (copy) a file to [`ChildPath`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino::Utf8Path;
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp
    ///    .child("foo.txt")
    ///    .write_file(Utf8Path::new("Cargo.toml"))
    ///    .unwrap();
    /// temp.close().unwrap();
    /// ```
    ///
    fn write_file(&self, data: &Utf8Path) -> Result<(), FixtureError>;
}

impl FileWriteFile for ChildPath {
    fn write_file(&self, data: &Utf8Path) -> Result<(), FixtureError> {
        write_file(self.path(), data)
    }
}

impl FileWriteFile for NamedUtf8TempFile {
    fn write_file(&self, data: &Utf8Path) -> Result<(), FixtureError> {
        write_file(self.path(), data)
    }
}

/// Copy files into [`Utf8TempDir`].
pub trait PathCopy {
    /// Copy files and directories into the current path from the `source` according to the glob
    /// `patterns`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// temp.copy_from(".", &["*.rs"]).unwrap();
    /// temp.close().unwrap();
    /// ```
    fn copy_from<P: AsRef<Utf8Path>, S: AsRef<str>>(
        &self,
        source: P,
        patterns: &[S],
    ) -> Result<(), FixtureError>;
}

impl PathCopy for Utf8TempDir {
    fn copy_from<P: AsRef<Utf8Path>, S: AsRef<str>>(
        &self,
        source: P,
        patterns: &[S],
    ) -> Result<(), FixtureError> {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

impl PathCopy for ChildPath {
    fn copy_from<P: AsRef<Utf8Path>, S: AsRef<str>>(
        &self,
        source: P,
        patterns: &[S],
    ) -> Result<(), FixtureError> {
        copy_files(self.path(), source.as_ref(), patterns)
    }
}

/// Create a symlink to a target file.
///
pub trait SymlinkToFile {
    /// Create a symlink to the provided target file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// let real_file = temp.child("real_file");
    /// real_file.touch().unwrap();
    ///
    /// temp.child("link_file").symlink_to_file(real_file.path()).unwrap();
    ///
    /// temp.close().unwrap();
    /// ```
    fn symlink_to_file<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError>;
}

impl SymlinkToFile for ChildPath {
    fn symlink_to_file<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError> {
        symlink_to_file(self.path(), target.as_ref())
    }
}

impl SymlinkToFile for NamedUtf8TempFile {
    fn symlink_to_file<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError> {
        symlink_to_file(self.path(), target.as_ref())
    }
}

/// Create a symlink to a target directory.
pub trait SymlinkToDir {
    /// Create a symlink to the provided target directory.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use camino_tempfile_ext::prelude::*;
    ///
    /// let temp = Utf8TempDir::new().unwrap();
    /// let real_dir = temp.child("real_dir");
    /// real_dir.create_dir_all().unwrap();
    ///
    /// temp.child("link_dir").symlink_to_dir(real_dir.path()).unwrap();
    ///
    /// temp.close().unwrap();
    /// ```
    fn symlink_to_dir<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError>;
}

impl SymlinkToDir for ChildPath {
    fn symlink_to_dir<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError> {
        symlink_to_dir(self.path(), target.as_ref())
    }
}

impl SymlinkToDir for Utf8TempDir {
    fn symlink_to_dir<P: AsRef<Path>>(&self, target: P) -> Result<(), FixtureError> {
        symlink_to_dir(self.path(), target.as_ref())
    }
}

fn ensure_parent_dir(path: &Utf8Path) -> Result<(), FixtureError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).chain(FixtureError::new(FixtureKind::CreateDir))?;
    }
    Ok(())
}

fn create_dir_all(path: &Utf8Path) -> Result<(), FixtureError> {
    fs::create_dir_all(path).chain(FixtureError::new(FixtureKind::CreateDir))?;
    Ok(())
}

fn touch(path: &Utf8Path) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    fs::File::create(path).chain(FixtureError::new(FixtureKind::WriteFile))?;
    Ok(())
}

fn write_binary(path: &Utf8Path, data: &[u8]) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    let mut file = fs::File::create(path).chain(FixtureError::new(FixtureKind::WriteFile))?;
    file.write_all(data)
        .chain(FixtureError::new(FixtureKind::WriteFile))?;
    Ok(())
}

fn write_str(path: &Utf8Path, data: &str) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    write_binary(path, data.as_bytes()).chain(FixtureError::new(FixtureKind::WriteFile))
}

fn write_file(path: &Utf8Path, data: &Utf8Path) -> Result<(), FixtureError> {
    ensure_parent_dir(path)?;
    fs::copy(data, path).chain(FixtureError::new(FixtureKind::CopyFile))?;
    Ok(())
}

fn copy_files<S>(target: &Utf8Path, source: &Utf8Path, patterns: &[S]) -> Result<(), FixtureError>
where
    S: AsRef<str>,
{
    // `walkdir`, on Windows, seems to convert "." into "" which then fails.
    let source = source
        .canonicalize()
        .chain(FixtureError::new(FixtureKind::Walk))?;

    // Use a regular `Path` rather than `Utf8Path` for this -- no particular
    // reason to restrict to UTF-8 paths within subdirectories like this.
    let target = target.as_std_path();

    for entry in GlobWalkerBuilder::from_patterns(&source, patterns)
        .follow_links(true)
        .build()
        .chain(FixtureError::new(FixtureKind::Walk))?
    {
        let entry = entry.chain(FixtureError::new(FixtureKind::Walk))?;
        let rel = entry
            .path()
            .strip_prefix(&source)
            .expect("entries to be under `source`");
        let target_path = target.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(target_path).chain(FixtureError::new(FixtureKind::CreateDir))?;
        } else if entry.file_type().is_file() {
            fs::create_dir_all(target_path.parent().expect("at least `target` exists"))
                .chain(FixtureError::new(FixtureKind::CreateDir))?;
            fs::copy(entry.path(), target_path).chain(FixtureError::new(FixtureKind::CopyFile))?;
        }
    }
    Ok(())
}

#[cfg(windows)]
fn symlink_to_file(link: &Utf8Path, target: &Path) -> Result<(), FixtureError> {
    std::os::windows::fs::symlink_file(target, link)
        .chain(FixtureError::new(FixtureKind::Symlink))?;
    Ok(())
}

#[cfg(windows)]
fn symlink_to_dir(link: &Utf8Path, target: &Path) -> Result<(), FixtureError> {
    std::os::windows::fs::symlink_dir(target, link)
        .chain(FixtureError::new(FixtureKind::Symlink))?;
    Ok(())
}

#[cfg(not(windows))]
fn symlink_to_file(link: &Utf8Path, target: &Path) -> Result<(), FixtureError> {
    std::os::unix::fs::symlink(target, link).chain(FixtureError::new(FixtureKind::Symlink))?;
    Ok(())
}

#[cfg(not(windows))]
fn symlink_to_dir(link: &Utf8Path, target: &Path) -> Result<(), FixtureError> {
    std::os::unix::fs::symlink(target, link).chain(FixtureError::new(FixtureKind::Symlink))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture::PathChild;

    #[test]
    fn test_symlink_to_file() {
        let temp_dir = Utf8TempDir::new().unwrap();
        let file = temp_dir.child("file");
        file.touch().unwrap();
        let link = temp_dir.child("link");
        link.symlink_to_file(&file).unwrap();

        assert!(link.exists());
        assert!(link.is_symlink());
        assert_eq!(link.read_link_utf8().unwrap().as_path(), file.as_std_path());
    }
}
