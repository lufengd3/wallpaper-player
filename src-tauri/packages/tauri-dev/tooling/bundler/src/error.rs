// Copyright 2016-2019 Cargo-Bundle developers <https://github.com/burtonageo/cargo-bundle>
// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::{io, num, path};
use thiserror::Error as DeriveError;

/// Errors returned by the bundler.
#[derive(Debug, DeriveError)]
#[non_exhaustive]
pub enum Error {
  /// Error running tauri_utils API.
  #[error("{0}")]
  Resource(#[from] tauri_utils::Error),
  /// Bundler error.
  #[error("{0}")]
  BundlerError(#[from] anyhow::Error),
  /// I/O error.
  #[error("`{0}`")]
  IoError(#[from] io::Error),
  /// Image error.
  #[error("`{0}`")]
  ImageError(#[from] image::ImageError),
  /// TOML error.
  #[error("`{0}`")]
  TomlError(#[from] toml::de::Error),
  /// Error walking directory.
  #[error("`{0}`")]
  WalkdirError(#[from] walkdir::Error),
  /// Strip prefix error.
  #[error("`{0}`")]
  StripError(#[from] path::StripPrefixError),
  /// Number parse error.
  #[error("`{0}`")]
  ConvertError(#[from] num::TryFromIntError),
  /// Zip error.
  #[cfg(target_os = "windows")]
  #[error("`{0}`")]
  ZipError(#[from] zip::result::ZipError),
  /// Hex error.
  #[cfg(target_os = "windows")]
  #[error("`{0}`")]
  HexError(#[from] hex::FromHexError),
  /// Handlebars template error.
  #[error("`{0}`")]
  HandleBarsError(#[from] handlebars::RenderError),
  /// JSON error.
  #[error("`{0}`")]
  JsonError(#[from] serde_json::error::Error),
  /// Regex error.
  #[cfg(any(target_os = "macos", windows))]
  #[error("`{0}`")]
  RegexError(#[from] regex::Error),
  /// Failed to perform HTTP request.
  #[cfg(windows)]
  #[error("`{0}`")]
  HttpError(#[from] attohttpc::Error),
  /// Invalid glob pattern.
  #[cfg(windows)]
  #[error("{0}")]
  GlobPattern(#[from] glob::PatternError),
  /// Failed to use glob pattern.
  #[cfg(windows)]
  #[error("`{0}`")]
  Glob(#[from] glob::GlobError),
  /// Failed to validate downloaded file hash.
  #[error("hash mismatch of downloaded file")]
  HashError,
  /// Unsupported architecture.
  #[error("Architecture Error: `{0}`")]
  ArchError(String),
  /// Couldn't find icons.
  #[error("Could not find Icon paths.  Please make sure they exist in the tauri config JSON file")]
  IconPathError,
  /// Error on path util operation.
  #[error("Path Error:`{0}`")]
  PathUtilError(String),
  /// Error on shell script.
  #[error("Shell Scripting Error:`{0}`")]
  ShellScriptError(String),
  /// Generic error.
  #[error("`{0}`")]
  GenericError(String),
  /// No bundled project found for the updater.
  #[error("Unable to find a bundled project for the updater")]
  UnableToFindProject,
  /// String is not UTF-8.
  #[error("string is not UTF-8")]
  Utf8(#[from] std::str::Utf8Error),
  /// Windows SignTool not found.
  #[error("SignTool not found")]
  SignToolNotFound,
  /// Failed to open Windows registry.
  #[error("failed to open registry {0}")]
  OpenRegistry(String),
  /// Failed to get registry value.
  #[error("failed to get {0} value on registry")]
  GetRegistryValue(String),
  /// Unsupported OS bitness.
  #[error("unsupported OS bitness")]
  UnsupportedBitness,
  /// Failed to sign application.
  #[error("failed to sign app: {0}")]
  Sign(String),
  /// time error.
  #[cfg(target_os = "macos")]
  #[error("`{0}`")]
  TimeError(#[from] time::error::Error),
  /// Plist error.
  #[cfg(target_os = "macos")]
  #[error(transparent)]
  Plist(#[from] plist::Error),
}

/// Convenient type alias of Result type.
pub type Result<T> = std::result::Result<T, Error>;
