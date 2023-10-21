// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
use crate::webview::webkitgtk::WebContextImpl;
#[cfg(any(target_os = "macos", target_os = "ios"))]
use crate::webview::wkwebview::WebContextImpl;

use std::path::{Path, PathBuf};

/// A context that is shared between multiple [`WebView`]s.
///
/// A browser would have a context for all the normal tabs and a different context for all the
/// private/incognito tabs.
///
/// # Warning
/// If [`Webview`] is created by a WebContext. Dropping `WebContext` will cause [`WebView`] lose
/// some actions like custom protocol on Mac. Please keep both instances when you still wish to
/// interact with them.
///
/// [`WebView`]: crate::webview::WebView
#[derive(Debug)]
pub struct WebContext {
  data: WebContextData,
  #[allow(dead_code)] // It's not needed on Windows and macOS.
  pub(crate) os: WebContextImpl,
}

impl WebContext {
  /// Create a new [`WebContext`].
  ///
  /// `data_directory`:
  /// * Whether the WebView window should have a custom user data path. This is useful in Windows
  ///   when a bundled application can't have the webview data inside `Program Files`.
  pub fn new(data_directory: Option<PathBuf>) -> Self {
    let data = WebContextData { data_directory };
    let os = WebContextImpl::new(&data);
    Self { data, os }
  }

  /// A reference to the data directory the context was created with.
  pub fn data_directory(&self) -> Option<&Path> {
    self.data.data_directory()
  }

  /// Set if this context allows automation.
  ///
  /// **Note:** This is currently only enforced on Linux, and has the stipulation that
  /// only 1 context allows automation at a time.
  pub fn set_allows_automation(&mut self, flag: bool) {
    self.os.set_allows_automation(flag);
  }
}

impl Default for WebContext {
  fn default() -> Self {
    let data = WebContextData::default();
    let os = WebContextImpl::new(&data);
    Self { data, os }
  }
}

/// Data that all [`WebContext`] share regardless of platform.
#[derive(Default, Debug)]
pub struct WebContextData {
  data_directory: Option<PathBuf>,
}

impl WebContextData {
  /// A reference to the data directory the context was created with.
  pub fn data_directory(&self) -> Option<&Path> {
    self.data_directory.as_deref()
  }
}

#[cfg(any(target_os = "windows", target_os = "android"))]
#[derive(Debug)]
pub(crate) struct WebContextImpl;

#[cfg(any(target_os = "windows", target_os = "android"))]
impl WebContextImpl {
  fn new(_data: &WebContextData) -> Self {
    Self
  }

  fn set_allows_automation(&mut self, _flag: bool) {}
}
