// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{error::Error, fmt};

pub(crate) trait ChainError {
    fn chain<F>(self, cause: F) -> Self
    where
        F: Error + Send + Sync + 'static;
}

pub(crate) trait ResultChainExt<T> {
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainError;
}

impl<T, E> ResultChainExt<T> for Result<T, E>
where
    E: Error + Send + Sync + 'static,
{
    fn chain<C>(self, chainable: C) -> Result<T, C>
    where
        C: ChainError,
    {
        self.map_err(|e| chainable.chain(e))
    }
}

/// Fixture initialization cause.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum FixtureKind {
    /// Failed when walking the source tree.
    Walk,
    /// Failed when copying a file.
    CopyFile,
    /// Failed when writing to a file.
    WriteFile,
    /// Failed when creating a directory.
    CreateDir,
    /// Failed to cleanup a fixture.
    Cleanup,
    /// Failed to create a symlink.
    Symlink,
}

impl fmt::Display for FixtureKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FixtureKind::Walk => write!(f, "error walking source tree"),
            FixtureKind::CopyFile => write!(f, "error copying file"),
            FixtureKind::WriteFile => write!(f, "error writing file"),
            FixtureKind::CreateDir => write!(f, "error creating directory"),
            FixtureKind::Cleanup => write!(f, "error cleaning up fixture"),
            FixtureKind::Symlink => write!(f, "error creating symlink to target"),
        }
    }
}

/// Failure when initializing the fixture.
#[derive(Debug)]
pub struct FixtureError {
    kind: FixtureKind,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl FixtureError {
    /// Create a `FixtureError`.
    pub fn new(kind: FixtureKind) -> Self {
        Self { kind, source: None }
    }

    /// Attach a source to the error.
    pub fn with_source(
        mut self,
        source: impl Into<Box<dyn Error + Send + Sync + 'static>>,
    ) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Return the fixture initialization cause.
    pub fn kind(&self) -> FixtureKind {
        self.kind
    }
}

impl Error for FixtureError {
    fn description(&self) -> &str {
        "failed to initialize fixture"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|source| &**source as &dyn Error)
    }
}

impl fmt::Display for FixtureError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "failed to initialize fixture: {}", self.kind)
    }
}

impl ChainError for FixtureError {
    fn chain<F>(mut self, source: F) -> Self
    where
        F: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::anyhow;
    use std::io;

    #[test]
    fn error_types_work_with_source() {
        // std::io::Error
        let error = FixtureError::new(FixtureKind::WriteFile).with_source(io::Error::other("test"));
        assert_eq!(error.kind(), FixtureKind::WriteFile);
        assert_eq!(error.source().unwrap().to_string(), "test");

        // anyhow::Error
        let error = FixtureError::new(FixtureKind::WriteFile).with_source(anyhow!("test"));
        assert_eq!(error.kind(), FixtureKind::WriteFile);
        assert_eq!(error.source().unwrap().to_string(), "test");

        // another FixtureError
        let error = FixtureError::new(FixtureKind::WriteFile)
            .with_source(FixtureError::new(FixtureKind::CopyFile));
        assert_eq!(error.kind(), FixtureKind::WriteFile);
        assert_eq!(
            error.source().unwrap().to_string(),
            "failed to initialize fixture: error copying file"
        );
    }
}
