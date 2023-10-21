// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{
  cmp::Ordering,
  env::current_dir,
  ffi::OsStr,
  path::{Path, PathBuf},
};

use ignore::WalkBuilder;
use once_cell::sync::Lazy;

use tauri_utils::config::parse::{
  folder_has_configuration_file, is_configuration_file, ConfigFormat,
};

const TAURI_GITIGNORE: &[u8] = include_bytes!("../../tauri.gitignore");

fn lookup<F: Fn(&PathBuf) -> bool>(dir: &Path, checker: F) -> Option<PathBuf> {
  let mut default_gitignore = std::env::temp_dir();
  default_gitignore.push(".gitignore");
  if !default_gitignore.exists() {
    if let Ok(mut file) = std::fs::File::create(default_gitignore.clone()) {
      use std::io::Write;
      let _ = file.write_all(TAURI_GITIGNORE);
    }
  }

  let mut builder = WalkBuilder::new(dir);
  builder.add_custom_ignore_filename(".taurignore");
  let _ = builder.add_ignore(default_gitignore);
  builder
    .require_git(false)
    .ignore(false)
    .max_depth(Some(
      std::env::var("TAURI_PATH_DEPTH")
        .map(|d| {
          d.parse()
            .expect("`TAURI_PATH_DEPTH` environment variable must be a positive integer")
        })
        .unwrap_or(3),
    ))
    .sort_by_file_path(|a, _| {
      if a.extension().is_some() {
        Ordering::Less
      } else {
        Ordering::Greater
      }
    });

  for entry in builder.build().flatten() {
    let path = dir.join(entry.path());
    if checker(&path) {
      return Some(path);
    }
  }
  None
}

fn get_tauri_dir() -> PathBuf {
  let cwd = current_dir().expect("failed to read cwd");

  if cwd.join("src-tauri/tauri.conf.json").exists()
    || cwd.join("src-tauri/tauri.conf.json5").exists()
  {
    return cwd.join("src-tauri/");
  }

  lookup(&cwd, |path| folder_has_configuration_file(path) || is_configuration_file(path))
  .map(|p| if p.is_dir() { p } else {  p.parent().unwrap().to_path_buf() })
  .unwrap_or_else(||
    panic!("Couldn't recognize the current folder as a Tauri project. It must contain a `{}`, `{}` or `{}` file in any subfolder.",
      ConfigFormat::Json.into_file_name(),
      ConfigFormat::Json5.into_file_name(),
      ConfigFormat::Toml.into_file_name()
    )
  )
}

fn get_app_dir() -> Option<PathBuf> {
  lookup(&current_dir().expect("failed to read cwd"), |path| {
    if let Some(file_name) = path.file_name() {
      file_name == OsStr::new("package.json")
    } else {
      false
    }
  })
  .map(|p| p.parent().unwrap().to_path_buf())
}

pub fn app_dir() -> &'static PathBuf {
  static APP_DIR: Lazy<PathBuf> =
    Lazy::new(|| get_app_dir().unwrap_or_else(|| get_tauri_dir().parent().unwrap().to_path_buf()));
  &APP_DIR
}

pub fn tauri_dir() -> PathBuf {
  get_tauri_dir()
}
