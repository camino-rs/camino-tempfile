# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.2] - 2025-06-29

### Fixed

- Optional features are now documented on docs.rs ([#7]).

[#7]: https://github.com/camino-rs/camino-tempfile/issues/7

## [0.3.1] - 2025-05-27

### Added

- `FixtureError::with_source` attaches a source to an error.

### Changed

- The `fmt::Display` implementation for `FixtureError` no longer shows the source, in keeping with modern Rust community standards.

## [0.3.0] - 2025-05-03

### Added

- Implement `PartialEq` with `Utf8Path` and `std::path::Path` types for `ChildPath`.

### Changed

- `ChildPath::path` is renamed to `ChildPath::as_path` to match `as_std_path` from the `Deref` implementation.

## [0.2.0] - 2025-05-03

### Added

- `impl AsRef<std::path::Path> for ChildPath`.

### Changed

- Assertions are now a non-default feature, `assert`.
- The `color` feature is now disabled by default and renamed to `assert-color`.

## [0.1.0] - 2025-05-03

Initial release.

[0.3.2]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-ext-0.3.2
[0.3.1]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-ext-0.3.1
[0.3.0]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-ext-0.3.0
[0.2.0]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-ext-0.2.0
[0.1.0]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-ext-0.1.0
