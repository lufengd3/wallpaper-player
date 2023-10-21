// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Types and functions related to file system directory management.

use serde::Serialize;
use std::{
  fs::{self, metadata, symlink_metadata},
  path::{Path, PathBuf},
};
use tempfile::{self, tempdir};

/// A disk entry which is either a file or a directory.
///
/// This is the result of the [`read_dir`]. The `children` field is always `Some` if the entry is a directory.
#[derive(Debug, Serialize)]
#[non_exhaustive]
pub struct DiskEntry {
  /// The path to the entry.
  pub path: PathBuf,
  /// The name of the entry (file name with extension or directory name).
  pub name: Option<String>,
  /// The children of this entry if it's a directory.
  #[serde(skip_serializing_if = "Option::is_none")]
  pub children: Option<Vec<DiskEntry>>,
}

/// Checks if the given path is a directory.
pub fn is_dir<P: AsRef<Path>>(path: P) -> crate::api::Result<bool> {
  metadata(path).map(|md| md.is_dir()).map_err(Into::into)
}

fn is_symlink<P: AsRef<Path>>(path: P) -> crate::api::Result<bool> {
  // TODO: remove the different implementation once we raise tauri's MSRV to at least 1.58
  #[cfg(windows)]
  let ret = symlink_metadata(path)
    .map(|md| md.is_symlink())
    .map_err(Into::into);

  #[cfg(not(windows))]
  let ret = symlink_metadata(path)
    .map(|md| md.file_type().is_symlink())
    .map_err(Into::into);

  ret
}

/// Reads a directory. Can perform recursive operations.
pub fn read_dir<P: AsRef<Path>>(path: P, recursive: bool) -> crate::api::Result<Vec<DiskEntry>> {
  read_dir_with_options(path, recursive, ReadDirOptions { scope: None })
}

#[derive(Clone, Copy)]
pub(crate) struct ReadDirOptions<'a> {
  pub scope: Option<&'a crate::FsScope>,
}

pub(crate) fn read_dir_with_options<P: AsRef<Path>>(
  path: P,
  recursive: bool,
  options: ReadDirOptions<'_>,
) -> crate::api::Result<Vec<DiskEntry>> {
  let mut files_and_dirs: Vec<DiskEntry> = vec![];
  for entry in fs::read_dir(path)? {
    let path = entry?.path();
    let path_as_string = path.display().to_string();

    if let Ok(flag) = is_dir(&path_as_string) {
      files_and_dirs.push(DiskEntry {
        path: path.clone(),
        children: if flag {
          Some(
            if recursive
              && (!is_symlink(&path_as_string)?
                || options.scope.map(|s| s.is_allowed(&path)).unwrap_or(true))
            {
              read_dir_with_options(&path_as_string, true, options)?
            } else {
              vec![]
            },
          )
        } else {
          None
        },
        name: path
          .file_name()
          .map(|name| name.to_string_lossy())
          .map(|name| name.to_string()),
      });
    }
  }
  Result::Ok(files_and_dirs)
}

/// Runs a closure with a temporary directory argument.
pub fn with_temp_dir<F: FnOnce(&tempfile::TempDir)>(callback: F) -> crate::api::Result<()> {
  let dir = tempdir()?;
  callback(&dir);
  dir.close()?;
  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use quickcheck_macros::quickcheck;
  use std::{ffi::OsStr, path::PathBuf};

  // check is dir function by passing in arbitrary strings
  #[quickcheck]
  fn qc_is_dir(f: String) -> bool {
    // if the string runs through is_dir and comes out as an OK result then it must be a DIR.
    if is_dir(f.clone()).is_ok() {
      PathBuf::from(f).is_dir()
    } else {
      true
    }
  }

  fn name_from_path(path: PathBuf) -> Option<String> {
    path
      .file_name()
      .map(|name| name.to_string_lossy())
      .map(|name| name.to_string())
  }

  #[test]
  // check the read_dir function with recursive = true
  fn check_read_dir_recursively() {
    // define a relative directory string test/api/
    let dir = PathBuf::from("test/api/");
    // add the files to this directory
    let mut file_one = dir.clone();
    file_one.push("test.txt");
    let mut file_two = dir.clone();
    file_two.push("test_binary");

    // call walk_dir on the directory
    let res = read_dir(dir, true);

    // assert that the result is Ok()
    assert!(res.is_ok());

    // destruct the OK into a vector of DiskEntry Structs
    if let Ok(vec) = res {
      // assert that the vector length is only 3
      assert_eq!(vec.len(), 2);

      // get the first DiskEntry
      let first = &vec[0];
      // get the second DiskEntry
      let second = &vec[1];

      if first.path.extension() == Some(OsStr::new("txt")) {
        // check the fields for the first DiskEntry
        assert_eq!(first.path, file_one);
        assert!(first.children.is_none());
        assert_eq!(first.name, name_from_path(file_one));

        // check the fields for the third DiskEntry
        assert_eq!(second.path, file_two);
        assert!(second.children.is_none());
        assert_eq!(second.name, name_from_path(file_two));
      } else {
        // check the fields for the second DiskEntry
        assert_eq!(first.path, file_two);
        assert!(first.children.is_none());
        assert_eq!(first.name, name_from_path(file_two));

        // check the fields for the third DiskEntry
        assert_eq!(second.path, file_one);
        assert!(second.children.is_none());
        assert_eq!(second.name, name_from_path(file_one));
      }
    }
  }

  #[test]
  // check the read_dir function with recursive = false
  fn check_read_dir() {
    // define a relative directory test/api/
    let dir = PathBuf::from("test/api/");

    // call list_dir_contents on the dir
    let res = read_dir(dir, false);

    // assert that the result is Ok()
    assert!(res.is_ok());

    // destruct the vector from the Ok()
    if let Ok(vec) = res {
      // assert the length of the vector is 2
      assert_eq!(vec.len(), 2);

      // get the two DiskEntry structs in this vector
      let first = &vec[0];
      let second = &vec[1];

      if first.path.extension() == Some(OsStr::new("txt")) {
        // check the fields for the first DiskEntry
        assert_eq!(first.path, PathBuf::from("test/api/test.txt"));
        assert!(first.children.is_none());
        assert_eq!(first.name, Some("test.txt".to_string()));

        // check the fields for the second DiskEntry
        assert_eq!(second.path, PathBuf::from("test/api/test_binary"));
        assert!(second.children.is_none());
        assert_eq!(second.name, Some("test_binary".to_string()));
      } else {
        // check the fields for the first DiskEntry
        assert_eq!(second.path, PathBuf::from("test/api/test.txt"));
        assert!(second.children.is_none());
        assert_eq!(second.name, Some("test.txt".to_string()));

        // check the fields for the second DiskEntry
        assert_eq!(first.path, PathBuf::from("test/api/test_binary"));
        assert!(first.children.is_none());
        assert_eq!(first.name, Some("test_binary".to_string()));
      }
    }
  }

  #[test]
  // test the with_temp_dir function
  fn check_test_dir() {
    // create a callback closure that takes in a TempDir type and prints it.
    let callback = |td: &tempfile::TempDir| {
      println!("{:?}", td);
    };

    // execute the with_temp_dir function on the callback
    let res = with_temp_dir(callback);

    // assert that the result is an OK type.
    assert!(res.is_ok());
  }
}
