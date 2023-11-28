# Changelog

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.1.1] - 2023-11-27

### Fixed

- Documentation fixes.

## [1.1.0] - 2023-11-27

### Added

- Mirroring the new API added in tempfile 3.8.0, added `with_prefix` and `with_prefix_in` to `Utf8TempDir` and `NamedUtf8TempFile` to make it easier to create temporary files/directories with nice prefixes.

### Changed

- Updated tempfile dependency to 3.8.1.
- Updated MSRV to 1.63 to match the MSRV of tempfile.

## [1.0.3] - 2023-11-27

This version was yanked because the MSRV needed to be bumped.

## [1.0.2] - 2023-04-23

Fix another publishing issue.

## [1.0.1] - 2023-04-23

Fix a publishing issue.

## [1.0.0] - 2023-04-23

Initial release.

[1.1.1]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.1.1
[1.1.0]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.1.0
[1.0.3]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.0.3
[1.0.2]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.0.2
[1.0.1]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.0.1
[1.0.0]: https://github.com/camino-rs/camino-tempfile/releases/tag/camino-tempfile-1.0.0
