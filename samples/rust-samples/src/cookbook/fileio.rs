// use std::fs::prelude::*;

use std::path::{Path, PathBuf};
use std::env::{temp_dir};

fn make_file_path(filename: &str) -> String {
    let mut path: PathBuf = temp_dir();
    path.push(filename);
    path.as_path().to_str().unwrap().to_string()
}

#[test]
fn mkdir_recursive() {
    //
    use std::fs::{self, DirBuilder};
    use std::env::{temp_dir};
    let mut path: PathBuf = temp_dir();
    path.push(Path::new("test_dir"));
    let result = DirBuilder::new().recursive(true).create(path.as_path());
    assert!(fs::metadata(path).unwrap().is_dir());
}

#[test]
fn create_file() {
    use std::io::prelude::*;
    use std::fs::File;

    let mut create_result = File::create(make_file_path("created.txt"));
    assert!(create_result.is_ok());
    let mut f = create_result.unwrap();
    for i in 0..100 {
        write!(f, "Line {}\n", i);
    }
}

#[test]
fn open_file() {
    use std::io::prelude::*;
    use std::fs::File;

    let mut open_result = File::open(make_file_path("open.txt"));
    assert!(open_result.is_ok());
    let mut f = open_result.unwrap();

}