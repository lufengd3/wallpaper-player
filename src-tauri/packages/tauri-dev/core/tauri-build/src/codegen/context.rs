// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use anyhow::{Context, Result};
use std::{
  env::var,
  fs::{create_dir_all, File},
  io::{BufWriter, Write},
  path::PathBuf,
};
use tauri_codegen::{context_codegen, ContextData};
use tauri_utils::config::{AppUrl, WindowUrl};

// TODO docs
/// A builder for generating a Tauri application context during compile time.
#[cfg_attr(doc_cfg, doc(cfg(feature = "codegen")))]
#[derive(Debug)]
pub struct CodegenContext {
  dev: bool,
  config_path: PathBuf,
  out_file: PathBuf,
}

impl Default for CodegenContext {
  fn default() -> Self {
    Self {
      dev: false,
      config_path: PathBuf::from("tauri.conf.json"),
      out_file: PathBuf::from("tauri-build-context.rs"),
    }
  }
}

impl CodegenContext {
  /// Create a new [`CodegenContext`] builder that is already filled with the default options.
  pub fn new() -> Self {
    Self::default()
  }

  /// Set the path to the `tauri.conf.json` (relative to the package's directory).
  ///
  /// This defaults to a file called `tauri.conf.json` inside of the current working directory of
  /// the package compiling; does not need to be set manually if that config file is in the same
  /// directory as your `Cargo.toml`.
  #[must_use]
  pub fn config_path(mut self, config_path: impl Into<PathBuf>) -> Self {
    self.config_path = config_path.into();
    self
  }

  /// Sets the output file's path.
  ///
  /// **Note:** This path should be relative to the `OUT_DIR`.
  ///
  /// Don't set this if you are using [`tauri::tauri_build_context!`] as that helper macro
  /// expects the default value. This option can be useful if you are not using the helper and
  /// instead using [`std::include!`] on the generated code yourself.
  ///
  /// Defaults to `tauri-build-context.rs`.
  ///
  /// [`tauri::tauri_build_context!`]: https://docs.rs/tauri/latest/tauri/macro.tauri_build_context.html
  #[must_use]
  pub fn out_file(mut self, filename: PathBuf) -> Self {
    self.out_file = filename;
    self
  }

  /// Run the codegen in a `dev` context, meaning that Tauri is using a dev server or local file for development purposes,
  /// usually with the `tauri dev` CLI command.
  #[must_use]
  pub fn dev(mut self) -> Self {
    self.dev = true;
    self
  }

  /// Generate the code and write it to the output file - returning the path it was saved to.
  ///
  /// Unless you are doing something special with this builder, you don't need to do anything with
  /// the returned output path.
  ///
  /// # Panics
  ///
  /// If any parts of the codegen fail, this will panic with the related error message. This is
  /// typically desirable when running inside a build script; see [`Self::try_build`] for no panics.
  pub fn build(self) -> PathBuf {
    match self.try_build() {
      Ok(out) => out,
      Err(error) => panic!("Error found during Codegen::build: {}", error),
    }
  }

  /// Non-panicking [`Self::build`]
  pub fn try_build(self) -> Result<PathBuf> {
    let (config, config_parent) = tauri_codegen::get_config(&self.config_path)?;

    // rerun if changed
    let app_url = if self.dev {
      &config.build.dev_path
    } else {
      &config.build.dist_dir
    };
    match app_url {
      AppUrl::Url(WindowUrl::App(p)) => {
        println!("cargo:rerun-if-changed={}", config_parent.join(p).display());
      }
      AppUrl::Files(files) => {
        for path in files {
          println!(
            "cargo:rerun-if-changed={}",
            config_parent.join(path).display()
          );
        }
      }
      _ => (),
    }
    for icon in &config.tauri.bundle.icon {
      println!(
        "cargo:rerun-if-changed={}",
        config_parent.join(icon).display()
      );
    }
    if let Some(tray_icon) = config.tauri.system_tray.as_ref().map(|t| &t.icon_path) {
      println!(
        "cargo:rerun-if-changed={}",
        config_parent.join(tray_icon).display()
      );
    }

    #[cfg(target_os = "macos")]
    println!(
      "cargo:rerun-if-changed={}",
      config_parent.join("Info.plist").display()
    );

    let code = context_codegen(ContextData {
      dev: self.dev,
      config,
      config_parent,
      // it's very hard to have a build script for unit tests, so assume this is always called from
      // outside the tauri crate, making the ::tauri root valid.
      root: quote::quote!(::tauri),
    })?;

    // get the full output file path
    let out = var("OUT_DIR")
      .map(PathBuf::from)
      .map(|path| path.join(&self.out_file))
      .with_context(|| "unable to find OUT_DIR during tauri-build")?;

    // make sure any nested directories in OUT_DIR are created
    let parent = out.parent().with_context(|| {
      "`Codegen` could not find the parent to `out_file` while creating the file"
    })?;
    create_dir_all(parent)?;

    let mut file = File::create(&out).map(BufWriter::new).with_context(|| {
      format!(
        "Unable to create output file during tauri-build {}",
        out.display()
      )
    })?;

    writeln!(file, "{}", code).with_context(|| {
      format!(
        "Unable to write tokenstream to out file during tauri-build {}",
        out.display()
      )
    })?;

    Ok(out)
  }
}
