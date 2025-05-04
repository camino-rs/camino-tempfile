// Copyright (c) The camino-tempfile Contributors
// Adapted from assert_fs: Copyright (c) The assert_fs Contributors
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#![cfg(feature = "assert")]

use camino_tempfile_ext::{fixture::ChildPath, prelude::*};
use predicates::prelude::*;

#[test]
fn code_example() {
    let temp = Utf8TempDir::new().unwrap();
    let input_file = temp.child("foo.txt");
    input_file.touch().unwrap();

    // ... do something with input_file ...

    input_file.assert("");
    temp.child("bar.txt").assert(predicate::path::missing());

    temp.close().unwrap();
}

#[test]
#[should_panic]
fn verify_failure_output() {
    let f = ChildPath::new("Cargo.toml");
    f.assert("Not real content");
}
