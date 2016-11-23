// use std::fs::prelude::*;

use std::path::{Path, PathBuf};
use std::env::{temp_dir};

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

}

#[test]
fn open_file() {

}