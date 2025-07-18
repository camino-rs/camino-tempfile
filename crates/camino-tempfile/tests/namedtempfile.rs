#![deny(rust_2018_idioms)]

use camino::{Utf8Path, Utf8PathBuf};
use camino_tempfile::{Builder, NamedUtf8TempFile, Utf8TempPath, tempdir};
use std::{
    env,
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
    path::Path,
};

fn exists<P: AsRef<Path>>(path: P) -> bool {
    std::fs::metadata(path.as_ref()).is_ok()
}

#[test]
fn test_basic() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    write!(tmpfile, "abcde").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    tmpfile.read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
}

#[test]
fn test_deleted() {
    let tmpfile = NamedUtf8TempFile::new().unwrap();
    let path = tmpfile.path().to_path_buf();
    assert!(exists(&path));
    drop(tmpfile);
    assert!(!exists(&path));
}

#[test]
fn test_persist() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    let old_path = tmpfile.path().to_path_buf();
    let persist_path = env::temp_dir().join("persisted_temporary_file");
    write!(tmpfile, "abcde").unwrap();
    {
        assert!(exists(&old_path));
        let mut f = tmpfile.persist(&persist_path).unwrap();
        assert!(!exists(&old_path));

        // Check original file
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!("abcde", buf);
    }

    {
        // Try opening it at the new path.
        let mut f = File::open(&persist_path).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!("abcde", buf);
    }
    std::fs::remove_file(&persist_path).unwrap();
}

#[test]
fn test_persist_noclobber() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    let old_path = tmpfile.path().to_path_buf();
    let persist_target = NamedUtf8TempFile::new().unwrap();
    let persist_path = persist_target.path().to_path_buf();
    write!(tmpfile, "abcde").unwrap();
    assert!(exists(&old_path));
    {
        tmpfile = tmpfile.persist_noclobber(&persist_path).unwrap_err().into();
        assert!(exists(&old_path));
        std::fs::remove_file(&persist_path).unwrap();
        drop(persist_target);
    }
    tmpfile.persist_noclobber(&persist_path).unwrap();
    // Try opening it at the new path.
    let mut f = File::open(&persist_path).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
    std::fs::remove_file(&persist_path).unwrap();
}

#[test]
fn test_customnamed() {
    let tmpfile = Builder::new()
        .prefix("tmp")
        .suffix(&".rs")
        .rand_bytes(12)
        .tempfile()
        .unwrap();
    let name = tmpfile.path().file_name().unwrap();
    assert!(name.starts_with("tmp"));
    assert!(name.ends_with(".rs"));
    assert_eq!(name.len(), 18);
}

#[test]
fn test_append() {
    let mut tmpfile = Builder::new().append(true).tempfile().unwrap();
    tmpfile.write_all(b"a").unwrap();
    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    tmpfile.write_all(b"b").unwrap();

    tmpfile.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = vec![0u8; 1];
    tmpfile.read_exact(&mut buf).unwrap();
    assert_eq!(buf, b"a");
}

#[test]
fn test_reopen() {
    let source = NamedUtf8TempFile::new().unwrap();
    let mut first = source.reopen().unwrap();
    let mut second = source.reopen().unwrap();
    drop(source);

    write!(first, "abcde").expect("write failed");
    let mut buf = String::new();
    second.read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
}

#[test]
fn test_into_file() {
    let mut file = NamedUtf8TempFile::new().unwrap();
    let path = file.path().to_owned();
    write!(file, "abcde").expect("write failed");

    assert!(path.exists());
    let mut file = file.into_file();
    assert!(!path.exists());

    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
}

#[test]
fn test_immut() {
    let tmpfile = NamedUtf8TempFile::new().unwrap();
    (&tmpfile).write_all(b"abcde").unwrap();
    (&tmpfile).seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    (&tmpfile).read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
}

#[test]
fn test_temppath() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    write!(tmpfile, "abcde").unwrap();

    let path = tmpfile.into_temp_path();
    assert!(path.is_file());
}

#[test]
fn test_temppath_persist() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    write!(tmpfile, "abcde").unwrap();

    let tmppath = tmpfile.into_temp_path();

    let old_path = tmppath.to_path_buf();
    let persist_path = env::temp_dir().join("persisted_temppath_file");

    {
        assert!(exists(&old_path));
        tmppath.persist(&persist_path).unwrap();
        assert!(!exists(&old_path));
    }

    {
        // Try opening it at the new path.
        let mut f = File::open(&persist_path).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!("abcde", buf);
    }

    std::fs::remove_file(&persist_path).unwrap();
}

#[test]
fn test_temppath_persist_noclobber() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    write!(tmpfile, "abcde").unwrap();

    let mut tmppath = tmpfile.into_temp_path();

    let old_path = tmppath.to_path_buf();
    let persist_target = NamedUtf8TempFile::new().unwrap();
    let persist_path = persist_target.path().to_path_buf();

    assert!(exists(&old_path));

    {
        tmppath = tmppath.persist_noclobber(&persist_path).unwrap_err().into();
        assert!(exists(&old_path));
        std::fs::remove_file(&persist_path).unwrap();
        drop(persist_target);
    }

    tmppath.persist_noclobber(&persist_path).unwrap();

    // Try opening it at the new path.
    let mut f = File::open(&persist_path).unwrap();
    f.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    assert_eq!("abcde", buf);
    std::fs::remove_file(&persist_path).unwrap();
}

#[test]
fn temp_path_from_existing() {
    let tmp_dir = tempdir().unwrap();
    let tmp_file_path_1 = tmp_dir.path().join("testfile1");
    let tmp_file_path_2 = tmp_dir.path().join("testfile2");

    File::create(&tmp_file_path_1).unwrap();
    assert!(tmp_file_path_1.exists(), "Test file 1 hasn't been created");

    File::create(&tmp_file_path_2).unwrap();
    assert!(tmp_file_path_2.exists(), "Test file 2 hasn't been created");

    let tmp_path = Utf8TempPath::from_path(&tmp_file_path_1);
    assert!(
        tmp_file_path_1.exists(),
        "Test file has been deleted before dropping TempPath"
    );

    drop(tmp_path);
    assert!(
        !tmp_file_path_1.exists(),
        "Test file exists after dropping TempPath"
    );
    assert!(
        tmp_file_path_2.exists(),
        "Test file 2 has been deleted before dropping TempDir"
    );
}

#[test]
#[allow(unreachable_code)]
fn temp_path_from_argument_types() {
    // This just has to compile
    return;

    Utf8TempPath::from_path("");
    Utf8TempPath::from_path(String::new());
    Utf8TempPath::from_path(Utf8Path::new(""));
    Utf8TempPath::from_path(Utf8PathBuf::new());
    Utf8TempPath::from_path(Utf8PathBuf::new().into_boxed_path());
}

#[test]
fn test_write_after_close() {
    let path = NamedUtf8TempFile::new().unwrap().into_temp_path();
    File::create(path).unwrap().write_all(b"test").unwrap();
}

#[test]
fn test_change_dir() {
    env::set_current_dir(env::temp_dir()).unwrap();
    let tmpfile = NamedUtf8TempFile::new_in(".").unwrap();
    let path = env::current_dir().unwrap().join(tmpfile.path());
    env::set_current_dir("/").unwrap();
    drop(tmpfile);
    assert!(!exists(path))
}

#[test]
fn test_into_parts() {
    let mut file = NamedUtf8TempFile::new().unwrap();
    write!(file, "abcd").expect("write failed");

    let (mut file, temp_path) = file.into_parts();

    let path = temp_path.to_path_buf();

    assert!(path.exists());
    drop(temp_path);
    assert!(!path.exists());

    write!(file, "efgh").expect("write failed");

    file.seek(SeekFrom::Start(0)).unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    assert_eq!("abcdefgh", buf);
}

#[test]
fn test_from_parts() {
    let mut file = NamedUtf8TempFile::new().unwrap();
    write!(file, "abcd").expect("write failed");

    let (file, temp_path) = file.into_parts();

    let file = NamedUtf8TempFile::from_parts(file, temp_path);

    assert!(file.path().exists());
}

#[test]
fn test_keep() {
    let mut tmpfile = NamedUtf8TempFile::new().unwrap();
    write!(tmpfile, "abcde").unwrap();
    let (mut f, temp_path) = tmpfile.into_parts();
    let path;
    {
        assert!(exists(&temp_path));
        path = temp_path.keep().unwrap();
        assert!(exists(&path));

        // Check original file
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!("abcde", buf);
    }

    {
        // Try opening it again.
        let mut f = File::open(&path).unwrap();
        f.seek(SeekFrom::Start(0)).unwrap();
        let mut buf = String::new();
        f.read_to_string(&mut buf).unwrap();
        assert_eq!("abcde", buf);
    }
    std::fs::remove_file(&path).unwrap();
}

#[test]
fn test_make() {
    let tmpfile = Builder::new().make(|path| File::create(path)).unwrap();

    assert!(tmpfile.path().is_file());
}

#[test]
fn test_make_in() {
    let tmp_dir = tempdir().unwrap();

    let tmpfile = Builder::new()
        .make_in(tmp_dir.path(), |path| File::create(path))
        .unwrap();

    assert!(tmpfile.path().is_file());
    assert_eq!(tmpfile.path().parent(), Some(tmp_dir.path()));
}

#[test]
fn test_make_fnmut() {
    let mut count = 0;

    // Show that an FnMut can be used.
    let tmpfile = Builder::new()
        .make(|path| {
            count += 1;
            File::create(path)
        })
        .unwrap();

    assert!(tmpfile.path().is_file());
}

#[cfg(unix)]
#[test]
fn test_make_uds() {
    use std::os::unix::net::UnixListener;

    let temp_sock = Builder::new()
        .prefix("tmp")
        .suffix(".sock")
        .rand_bytes(12)
        .make(|path| UnixListener::bind(path))
        .unwrap();

    assert!(temp_sock.path().exists());
}

// Issue #224 on tempfile-rs.
#[test]
fn test_overly_generic_bounds() {
    pub struct Foo<T>(T);

    impl<T> Foo<T>
    where
        T: Sync + Send + 'static,
        for<'a> &'a T: Write + Read,
    {
        pub fn new(foo: T) -> Self {
            Self(foo)
        }
    }

    // Don't really need to run this. Only care if it compiles.
    if let Ok(file) = File::open("i_do_not_exist") {
        let mut f;
        let _x = {
            f = Foo::new(file);
            &mut f
        };
    }
}
