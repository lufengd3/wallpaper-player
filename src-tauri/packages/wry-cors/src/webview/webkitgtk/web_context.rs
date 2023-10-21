// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! Unix platform extensions for [`WebContext`](super::WebContext).

use crate::{webview::web_context::WebContextData, Error};
use glib::FileError;
use http::{header::CONTENT_TYPE, Request, Response};
use std::{
  cell::RefCell,
  collections::{HashSet, VecDeque},
  path::PathBuf,
  rc::Rc,
  str::FromStr,
  sync::{
    atomic::{AtomicBool, Ordering::SeqCst},
    Mutex,
  },
};
use url::Url;
use webkit2gtk::{
  traits::*, ApplicationInfo, CookiePersistentStorage, LoadEvent, UserContentManager, WebContext,
  WebContextBuilder, WebView, WebsiteDataManagerBuilder,
};

#[derive(Debug)]
pub struct WebContextImpl {
  context: WebContext,
  manager: UserContentManager,
  webview_uri_loader: Rc<WebviewUriLoader>,
  registered_protocols: HashSet<String>,
  automation: bool,
  app_info: Option<ApplicationInfo>,
}

impl WebContextImpl {
  pub fn new(data: &WebContextData) -> Self {
    use webkit2gtk::traits::*;

    let mut context_builder = WebContextBuilder::new();
    if let Some(data_directory) = data.data_directory() {
      let data_manager = WebsiteDataManagerBuilder::new()
        .local_storage_directory(&data_directory.join("localstorage").to_string_lossy())
        .indexeddb_directory(
          &data_directory
            .join("databases")
            .join("indexeddb")
            .to_string_lossy(),
        )
        .build();
      if let Some(cookie_manager) = data_manager.cookie_manager() {
        cookie_manager.set_persistent_storage(
          &data_directory.join("cookies").to_string_lossy(),
          CookiePersistentStorage::Text,
        );
      }
      context_builder = context_builder.website_data_manager(&data_manager);
    }

    let context = context_builder.build();

    let automation = false;
    context.set_automation_allowed(automation);

    // e.g. wry 0.9.4
    let app_info = ApplicationInfo::new();
    app_info.set_name(env!("CARGO_PKG_NAME"));
    app_info.set_version(
      env!("CARGO_PKG_VERSION_MAJOR")
        .parse()
        .expect("invalid wry version major"),
      env!("CARGO_PKG_VERSION_MINOR")
        .parse()
        .expect("invalid wry version minor"),
      env!("CARGO_PKG_VERSION_PATCH")
        .parse()
        .expect("invalid wry version patch"),
    );

    Self {
      context,
      automation,
      manager: UserContentManager::new(),
      registered_protocols: Default::default(),
      webview_uri_loader: Rc::default(),
      app_info: Some(app_info),
    }
  }

  pub fn set_allows_automation(&mut self, flag: bool) {
    use webkit2gtk::traits::*;
    self.automation = flag;
    self.context.set_automation_allowed(flag);
  }
}

/// [`WebContext`](super::WebContext) items that only matter on unix.
pub trait WebContextExt {
  /// The GTK [`WebContext`] of all webviews in the context.
  fn context(&self) -> &WebContext;

  /// The GTK [`UserContentManager`] of all webviews in the context.
  fn manager(&self) -> &UserContentManager;

  /// Register a custom protocol to the web context.
  ///
  /// When duplicate schemes are registered, the duplicate handler will still be submitted and the
  /// `Err(Error::DuplicateCustomProtocol)` will be returned. It is safe to ignore if you are
  /// relying on the platform's implementation to properly handle duplicated scheme handlers.
  fn register_uri_scheme<F>(&mut self, name: &str, handler: F) -> crate::Result<()>
  where
    F: Fn(&Request<Vec<u8>>) -> crate::Result<Response<Vec<u8>>> + 'static;

  /// Register a custom protocol to the web context, only if it is not a duplicate scheme.
  ///
  /// If a duplicate scheme has been passed, its handler will **NOT** be registered and the
  /// function will return `Err(Error::DuplicateCustomProtocol)`.
  fn try_register_uri_scheme<F>(&mut self, name: &str, handler: F) -> crate::Result<()>
  where
    F: Fn(&Request<Vec<u8>>) -> crate::Result<Response<Vec<u8>>> + 'static;

  /// Add a [`WebView`] to the queue waiting to be opened.
  ///
  /// See the `WebviewUriLoader` for more information.
  fn queue_load_uri(&self, webview: Rc<WebView>, url: Url);

  /// Flush all queued [`WebView`]s waiting to load a uri.
  ///
  /// See the `WebviewUriLoader` for more information.
  fn flush_queue_loader(&self);

  /// If the context allows automation.
  ///
  /// **Note:** `libwebkit2gtk` only allows 1 automation context at a time.
  fn allows_automation(&self) -> bool;

  fn register_automation(&mut self, webview: WebView);

  fn register_download_handler(
    &mut self,
    download_started_callback: Option<Box<dyn FnMut(String, &mut PathBuf) -> bool>>,
    download_completed_callback: Option<Rc<dyn Fn(String, Option<PathBuf>, bool) + 'static>>,
  );
}

impl WebContextExt for super::WebContext {
  fn context(&self) -> &WebContext {
    &self.os.context
  }

  fn manager(&self) -> &UserContentManager {
    &self.os.manager
  }

  fn register_uri_scheme<F>(&mut self, name: &str, handler: F) -> crate::Result<()>
  where
    F: Fn(&Request<Vec<u8>>) -> crate::Result<Response<Vec<u8>>> + 'static,
  {
    actually_register_uri_scheme(self, name, handler)?;
    if self.os.registered_protocols.insert(name.to_string()) {
      Ok(())
    } else {
      Err(Error::DuplicateCustomProtocol(name.to_string()))
    }
  }

  fn try_register_uri_scheme<F>(&mut self, name: &str, handler: F) -> crate::Result<()>
  where
    F: Fn(&Request<Vec<u8>>) -> crate::Result<Response<Vec<u8>>> + 'static,
  {
    if self.os.registered_protocols.insert(name.to_string()) {
      actually_register_uri_scheme(self, name, handler)
    } else {
      Err(Error::DuplicateCustomProtocol(name.to_string()))
    }
  }

  fn queue_load_uri(&self, webview: Rc<WebView>, url: Url) {
    self.os.webview_uri_loader.push(webview, url)
  }

  fn flush_queue_loader(&self) {
    Rc::clone(&self.os.webview_uri_loader).flush()
  }

  fn allows_automation(&self) -> bool {
    self.os.automation
  }

  fn register_automation(&mut self, webview: WebView) {
    use webkit2gtk::traits::*;

    if let (true, Some(app_info)) = (self.os.automation, self.os.app_info.take()) {
      self.os.context.connect_automation_started(move |_, auto| {
        let webview = webview.clone();
        auto.set_application_info(&app_info);

        // We do **NOT** support arbitrarily creating new webviews.
        // To support this in the future, we would need a way to specify the
        // default WindowBuilder to use to create the window it will use, and
        // possibly "default" webview attributes. Difficulty comes in for controlling
        // the owned Window that would need to be used.
        //
        // Instead, we just pass the first created webview.
        auto.connect_create_web_view(None, move |_| webview.clone());
      });
    }
  }

  fn register_download_handler(
    &mut self,
    download_started_handler: Option<Box<dyn FnMut(String, &mut PathBuf) -> bool>>,
    download_completed_handler: Option<Rc<dyn Fn(String, Option<PathBuf>, bool) + 'static>>,
  ) {
    use webkit2gtk::traits::*;
    let context = &self.os.context;

    let download_started_handler = RefCell::new(download_started_handler);
    let failed = Rc::new(RefCell::new(false));

    context.connect_download_started(move |_context, download| {
      if let Some(uri) = download.request().and_then(|req| req.uri()) {
        let uri = uri.to_string();
        let mut download_location = download
          .destination()
          .and_then(|p| PathBuf::from_str(&p).ok())
          .unwrap_or_default();

        if let Some(download_started_handler) = download_started_handler.borrow_mut().as_mut() {
          if download_started_handler(uri, &mut download_location) {
            download.connect_response_notify(move |download| {
              download.set_destination(&download_location.to_string_lossy());
            });
          } else {
            download.cancel();
          }
        }
      }
      download.connect_failed({
        let failed = failed.clone();
        move |_, _error| {
          *failed.borrow_mut() = true;
        }
      });
      if let Some(download_completed_handler) = download_completed_handler.clone() {
        download.connect_finished({
          let failed = failed.clone();
          move |download| {
            if let Some(uri) = download.request().and_then(|req| req.uri()) {
              let failed = failed.borrow();
              let uri = uri.to_string();
              download_completed_handler(
                uri,
                (!*failed)
                  .then(|| {
                    download
                      .destination()
                      .map_or_else(|| None, |p| Some(PathBuf::from(p.as_str())))
                  })
                  .flatten(),
                !*failed,
              )
            }
          }
        });
      }
    });
  }
}

fn actually_register_uri_scheme<F>(
  context: &mut super::WebContext,
  name: &str,
  handler: F,
) -> crate::Result<()>
where
  F: Fn(&Request<Vec<u8>>) -> crate::Result<Response<Vec<u8>>> + 'static,
{
  use webkit2gtk::traits::*;
  let context = &context.os.context;
  // Enable secure context
  context
    .security_manager()
    .ok_or(Error::MissingManager)?
    .register_uri_scheme_as_secure(name);

  context.register_uri_scheme(name, move |request| {
    if let Some(uri) = request.uri() {
      let uri = uri.as_str();

      // FIXME: Read the body (forms post)
      #[allow(unused_mut)]
      let mut http_request = Request::builder().uri(uri).method("GET");
      #[cfg(feature = "linux-headers")]
      {
        use http::{header::HeaderName, HeaderValue};

        if let Some(mut headers) = request.http_headers() {
          if let Some(map) = http_request.headers_mut() {
            headers.foreach(move |k, v| {
              if let Ok(name) = HeaderName::from_bytes(k.as_bytes()) {
                if let Ok(value) = HeaderValue::from_bytes(v.as_bytes()) {
                  map.insert(name, value);
                }
              }
            });
          }
        }

        if let Some(method) = request.http_method() {
          http_request = http_request.method(method.as_str());
        }
      }
      let http_request = match http_request.body(Vec::new()) {
        Ok(req) => req,
        Err(_) => {
          request.finish_error(&mut glib::Error::new(
            // TODO: use UriError when we can use 2_66 webkit2gtk feature flag
            FileError::Exist,
            "Could not get uri.",
          ));
          return;
        }
      };

      match handler(&http_request) {
        Ok(http_response) => {
          let buffer = http_response.body();
          let input = gio::MemoryInputStream::from_bytes(&glib::Bytes::from(buffer));
          let content_type = http_response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|h| h.to_str().ok());
          #[cfg(feature = "linux-headers")]
          {
            use soup::{MessageHeaders, MessageHeadersType};
            use webkit2gtk::URISchemeResponse;

            let response = URISchemeResponse::new(&input, buffer.len() as i64);
            response.set_status(http_response.status().as_u16() as u32, None);
            if let Some(content_type) = content_type {
              response.set_content_type(content_type);
            }

            let mut headers = MessageHeaders::new(MessageHeadersType::Response);
            for (name, value) in http_response.headers().into_iter() {
              headers.append(name.as_str(), value.to_str().unwrap_or(""));
            }
            response.set_http_headers(&mut headers);

            request.finish_with_response(&response);
          }
          #[cfg(not(feature = "linux-headers"))]
          request.finish(&input, buffer.len() as i64, content_type)
        }
        Err(_) => request.finish_error(&mut glib::Error::new(
          FileError::Exist,
          "Could not get requested file.",
        )),
      }
    } else {
      request.finish_error(&mut glib::Error::new(
        FileError::Exist,
        "Could not get uri.",
      ));
    }
  });

  Ok(())
}

/// Prevents an unknown concurrency bug with loading multiple URIs at the same time on webkit2gtk.
///
/// Using the queue prevents data race issues with loading uris for multiple [`WebView`]s in the
/// same context at the same time. Occasionally, the one of the [`WebView`]s will be clobbered
/// and it's content will be injected into a different [`WebView`].
///
/// Example of `webview-c` clobbering `webview-b` while `webview-a` is okay:
/// ```text
/// webview-a triggers load-change::started
/// URISchemeRequestCallback triggered with webview-a
/// webview-a triggers load-change::committed
/// webview-a triggers load-change::finished
/// webview-b triggers load-change::started
/// webview-c triggers load-change::started
/// URISchemeRequestCallback triggered with webview-c
/// URISchemeRequestCallback triggered with webview-c
/// webview-c triggers load-change::committed
/// webview-c triggers load-change::finished
/// ```
///
/// In that example, `webview-a` will load fine. `webview-b` will remain empty as the uri was
/// never loaded. `webview-c` will contain the content of both `webview-b` and `webview-c`
/// because it was triggered twice even through only started once. The content injected will not
/// be sequential, and often is interjected in the middle of one of the other contents.
///
/// FIXME: We think this may be an underlying concurrency bug in webkit2gtk as the usual ways of
/// fixing threading issues are not working. Ideally, the locks are not needed if we can understand
/// the true cause of the bug.
#[derive(Debug, Default)]
struct WebviewUriLoader {
  lock: AtomicBool,
  queue: Mutex<VecDeque<(Rc<WebView>, Url)>>,
}

impl WebviewUriLoader {
  /// Check if the lock is in use.
  fn is_locked(&self) -> bool {
    self.lock.swap(true, SeqCst)
  }

  /// Unlock the lock.
  fn unlock(&self) {
    self.lock.store(false, SeqCst)
  }

  /// Add a [`WebView`] to the queue.
  fn push(&self, webview: Rc<WebView>, url: Url) {
    let mut queue = self.queue.lock().expect("poisoned load queue");
    queue.push_back((webview, url))
  }

  /// Remove a [`WebView`] from the queue and return it.
  fn pop(&self) -> Option<(Rc<WebView>, Url)> {
    let mut queue = self.queue.lock().expect("poisoned load queue");
    queue.pop_front()
  }

  /// Load the next uri to load if the lock is not engaged.
  fn flush(self: Rc<Self>) {
    if !self.is_locked() {
      if let Some((webview, url)) = self.pop() {
        // we do not need to listen to failed events because those will finish the change event anyways
        webview.connect_load_changed(move |_, event| {
          if let LoadEvent::Finished = event {
            self.unlock();
            Rc::clone(&self).flush();
          };
        });

        webview.load_uri(url.as_str());
      } else {
        self.unlock();
      }
    }
  }
}
