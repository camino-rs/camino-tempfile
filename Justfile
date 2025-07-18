set positional-arguments

# Note: help messages should be 1 line long as required by just.

# Print a help message.
help:
    just --list

# Run `cargo hack --feature-powerset` on crates
powerset *args:
    cargo hack --feature-powerset --workspace "$@"

# Build docs for crates and direct dependencies
rustdoc *args:
    #!/usr/bin/env sh
    cargo tree --depth 1 -e normal --prefix none --workspace --all-features \
        | gawk '{ gsub(" v", "@", $0); printf("%s\n", $1); }' \
        | xargs printf -- '-p %s\n' \
        | RUSTC_BOOTSTRAP=1 RUSTDOCFLAGS="$RUSTDOCFLAGS --cfg=doc_cfg" xargs cargo doc --no-deps --all-features {{args}}

# Generate README.md files using `cargo-sync-rdme`.
generate-readmes:
    cargo sync-rdme --toolchain nightly-2025-06-21 --workspace --all-features

# Run cargo release in CI.
ci-cargo-release package:
    # cargo-release requires a release off a branch.
    git checkout -B to-release
    cargo release publish --publish --execute --no-confirm --package {{package}}
    git checkout -
    git branch -D to-release
