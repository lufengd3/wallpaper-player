// Copyright 2016-2019 Cargo-Bundle developers <https://github.com/burtonageo/cargo-bundle>
// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use super::common;

#[cfg(target_os = "macos")]
use super::macos::app;

#[cfg(target_os = "linux")]
use super::linux::appimage;

#[cfg(target_os = "windows")]
use super::windows::msi;
use log::error;
#[cfg(target_os = "windows")]
use std::{fs::File, io::prelude::*};
#[cfg(target_os = "windows")]
use zip::write::FileOptions;

use crate::{bundle::Bundle, Settings};
use anyhow::Context;
use log::info;
use std::path::{Path, PathBuf};
use std::{fs, io::Write};

// Build update
pub fn bundle_project(settings: &Settings, bundles: &[Bundle]) -> crate::Result<Vec<PathBuf>> {
  if cfg!(unix) || cfg!(windows) || cfg!(macos) {
    // Create our archive bundle
    let bundle_result = bundle_update(settings, bundles)?;
    Ok(bundle_result)
  } else {
    error!("Current platform do not support updates");
    Ok(vec![])
  }
}

// Create simple update-macos.tar.gz
// This is the Mac OS App packaged
#[cfg(target_os = "macos")]
fn bundle_update(settings: &Settings, bundles: &[Bundle]) -> crate::Result<Vec<PathBuf>> {
  use std::ffi::OsStr;

  // find our .app or rebuild our bundle
  let bundle_path = match bundles
    .iter()
    .filter(|bundle| bundle.package_type == crate::PackageType::MacOsBundle)
    .find_map(|bundle| {
      bundle
        .bundle_paths
        .iter()
        .find(|path| path.extension() == Some(OsStr::new("app")))
    }) {
    Some(path) => vec![path.clone()],
    None => app::bundle_project(settings)?,
  };

  // we expect our .app to be on bundle_path[0]
  if bundle_path.is_empty() {
    return Err(crate::Error::UnableToFindProject);
  }

  let source_path = &bundle_path[0];

  // add .tar.gz to our path
  let osx_archived = format!("{}.tar.gz", source_path.display());
  let osx_archived_path = PathBuf::from(&osx_archived);

  // Create our gzip file (need to send parent)
  // as we walk the source directory (source isnt added)
  create_tar(source_path, &osx_archived_path)
    .with_context(|| "Failed to tar.gz update directory")?;

  info!(action = "Bundling"; "{} ({})", osx_archived, osx_archived_path.display());

  Ok(vec![osx_archived_path])
}

// Create simple update-linux_<arch>.tar.gz
// Including the AppImage
// Right now in linux we hot replace the bin and request a restart
// No assets are replaced
#[cfg(target_os = "linux")]
fn bundle_update(settings: &Settings, bundles: &[Bundle]) -> crate::Result<Vec<PathBuf>> {
  use std::ffi::OsStr;

  // build our app actually we support only appimage on linux
  let bundle_path = match bundles
    .iter()
    .filter(|bundle| bundle.package_type == crate::PackageType::AppImage)
    .find_map(|bundle| {
      bundle
        .bundle_paths
        .iter()
        .find(|path| path.extension() == Some(OsStr::new("AppImage")))
    }) {
    Some(path) => vec![path.clone()],
    None => appimage::bundle_project(settings)?,
  };

  // we expect our .app to be on bundle[0]
  if bundle_path.is_empty() {
    return Err(crate::Error::UnableToFindProject);
  }

  let source_path = &bundle_path[0];

  // add .tar.gz to our path
  let appimage_archived = format!("{}.tar.gz", source_path.display());
  let appimage_archived_path = PathBuf::from(&appimage_archived);

  // Create our gzip file
  create_tar(source_path, &appimage_archived_path)
    .with_context(|| "Failed to tar.gz update directory")?;

  info!(action = "Bundling"; "{} ({})", appimage_archived, appimage_archived_path.display());

  Ok(vec![appimage_archived_path])
}

// Create simple update-win_<arch>.zip
// Including the binary as root
// Right now in windows we hot replace the bin and request a restart
// No assets are replaced
#[cfg(target_os = "windows")]
fn bundle_update(settings: &Settings, bundles: &[Bundle]) -> crate::Result<Vec<PathBuf>> {
  use crate::bundle::settings::WebviewInstallMode;

  // find our .msi or rebuild
  let bundle_paths = if matches!(
    settings.windows().webview_install_mode,
    WebviewInstallMode::OfflineInstaller { .. } | WebviewInstallMode::EmbedBootstrapper { .. }
  ) {
    msi::bundle_project(settings, true)?
  } else {
    let paths = bundles
      .iter()
      .find(|bundle| bundle.package_type == crate::PackageType::WindowsMsi)
      .map(|bundle| bundle.bundle_paths.clone())
      .unwrap_or_default();

    // we expect our .msi files to be on `bundle_paths`
    if paths.is_empty() {
      msi::bundle_project(settings, false)?
    } else {
      paths
    }
  };

  let mut msi_archived_paths = Vec::new();

  for source_path in bundle_paths {
    // add .zip to our path
    let msi_archived_path = source_path
      .components()
      .fold(PathBuf::new(), |mut p, c| {
        if let std::path::Component::Normal(name) = c {
          if name == msi::MSI_UPDATER_FOLDER_NAME {
            p.push(msi::MSI_FOLDER_NAME);
            return p;
          }
        }
        p.push(c);
        p
      })
      .with_extension("msi.zip");

    info!(action = "Bundling"; "{}", msi_archived_path.display());

    // Create our gzip file
    create_zip(&source_path, &msi_archived_path).with_context(|| "Failed to zip update MSI")?;

    msi_archived_paths.push(msi_archived_path);
  }

  Ok(msi_archived_paths)
}

#[cfg(target_os = "windows")]
pub fn create_zip(src_file: &Path, dst_file: &Path) -> crate::Result<PathBuf> {
  let parent_dir = dst_file.parent().expect("No data in parent");
  fs::create_dir_all(parent_dir)?;
  let writer = common::create_file(dst_file)?;

  let file_name = src_file
    .file_name()
    .expect("Can't extract file name from path");

  let mut zip = zip::ZipWriter::new(writer);
  let options = FileOptions::default()
    .compression_method(zip::CompressionMethod::Stored)
    .unix_permissions(0o755);

  zip.start_file(file_name.to_string_lossy(), options)?;
  let mut f = File::open(src_file)?;
  let mut buffer = Vec::new();
  f.read_to_end(&mut buffer)?;
  zip.write_all(&*buffer)?;
  buffer.clear();

  Ok(dst_file.to_owned())
}

#[cfg(not(target_os = "windows"))]
fn create_tar(src_dir: &Path, dest_path: &Path) -> crate::Result<PathBuf> {
  let dest_file = common::create_file(dest_path)?;
  let gzip_encoder = libflate::gzip::Encoder::new(dest_file)?;

  let gzip_encoder = create_tar_from_src(src_dir, gzip_encoder)?;
  let mut dest_file = gzip_encoder.finish().into_result()?;
  dest_file.flush()?;
  Ok(dest_path.to_owned())
}

#[cfg(not(target_os = "windows"))]
fn create_tar_from_src<P: AsRef<Path>, W: Write>(src_dir: P, dest_file: W) -> crate::Result<W> {
  let src_dir = src_dir.as_ref();
  let mut tar_builder = tar::Builder::new(dest_file);

  // validate source type
  let file_type = fs::metadata(src_dir).expect("Can't read source directory");
  // if it's a file don't need to walkdir
  if file_type.is_file() {
    let mut src_file = fs::File::open(src_dir)?;
    let file_name = src_dir
      .file_name()
      .expect("Can't extract file name from path");

    tar_builder.append_file(file_name, &mut src_file)?;
  } else {
    for entry in walkdir::WalkDir::new(src_dir) {
      let entry = entry?;
      let src_path = entry.path();
      if src_path == src_dir {
        continue;
      }

      // We add the .parent() because example if we send a path
      // /dev/src-tauri/target/debug/bundle/osx/app.app
      // We need a tar with app.app/<...> (source root folder should be included)
      // safe to unwrap: the path has a parent
      let dest_path = src_path.strip_prefix(src_dir.parent().unwrap())?;
      if entry.file_type().is_dir() {
        tar_builder.append_dir(dest_path, src_path)?;
      } else {
        let mut src_file = fs::File::open(src_path)?;
        tar_builder.append_file(dest_path, &mut src_file)?;
      }
    }
  }
  let dest_file = tar_builder.into_inner()?;
  Ok(dest_file)
}
