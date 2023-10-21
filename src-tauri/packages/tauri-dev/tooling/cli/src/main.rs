// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::env::args_os;
use std::ffi::OsStr;
use std::path::Path;
use std::process::exit;

fn main() {
  let mut args = args_os().peekable();
  let bin_name = match args
    .next()
    .as_deref()
    .map(Path::new)
    .and_then(Path::file_stem)
    .and_then(OsStr::to_str)
  {
    Some("cargo-tauri") => {
      if args.peek().and_then(|s| s.to_str()) == Some("tauri") {
        // remove the extra cargo subcommand
        args.next();
        Some("cargo tauri".into())
      } else {
        Some("cargo-tauri".into())
      }
    }
    Some(stem) => Some(stem.to_string()),
    None => {
      eprintln!("cargo-tauri wrapper unable to read first argument");
      exit(1);
    }
  };

  tauri_cli::run(args, bin_name)
}
