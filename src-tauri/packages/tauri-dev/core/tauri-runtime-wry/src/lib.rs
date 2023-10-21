// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

//! The [`wry`] Tauri [`Runtime`].

use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle};
use tauri_runtime::{
  http::{header::CONTENT_TYPE, Request as HttpRequest, RequestParts, Response as HttpResponse},
  menu::{AboutMetadata, CustomMenuItem, Menu, MenuEntry, MenuHash, MenuId, MenuItem, MenuUpdate},
  monitor::Monitor,
  webview::{WebviewIpcHandler, WindowBuilder, WindowBuilderBase},
  window::{
    dpi::{LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, Position, Size},
    CursorIcon, DetachedWindow, FileDropEvent, JsEventListenerKey, PendingWindow, WindowEvent,
  },
  Dispatch, Error, EventLoopProxy, ExitRequestedEventAction, Icon, Result, RunEvent, RunIteration,
  Runtime, RuntimeHandle, UserAttentionType, UserEvent,
};

use tauri_runtime::window::MenuEvent;
#[cfg(all(desktop, feature = "system-tray"))]
use tauri_runtime::{SystemTray, SystemTrayEvent};
#[cfg(windows)]
use webview2_com::FocusChangedEventHandler;
#[cfg(windows)]
use windows::Win32::{Foundation::HWND, System::WinRT::EventRegistrationToken};
#[cfg(target_os = "macos")]
use wry::application::platform::macos::EventLoopWindowTargetExtMacOS;
#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowBuilderExtMacOS;
#[cfg(target_os = "linux")]
use wry::application::platform::unix::{WindowBuilderExtUnix, WindowExtUnix};
#[cfg(windows)]
use wry::application::platform::windows::{WindowBuilderExtWindows, WindowExtWindows};

#[cfg(target_os = "macos")]
use tauri_utils::TitleBarStyle;
use tauri_utils::{config::WindowConfig, debug_eprintln, Theme};
use uuid::Uuid;
use wry::{
  application::{
    dpi::{
      LogicalPosition as WryLogicalPosition, LogicalSize as WryLogicalSize,
      PhysicalPosition as WryPhysicalPosition, PhysicalSize as WryPhysicalSize,
      Position as WryPosition, Size as WrySize,
    },
    event::{Event, StartCause, WindowEvent as WryWindowEvent},
    event_loop::{
      ControlFlow, EventLoop, EventLoopProxy as WryEventLoopProxy, EventLoopWindowTarget,
    },
    menu::{
      AboutMetadata as WryAboutMetadata, CustomMenuItem as WryCustomMenuItem, MenuBar,
      MenuId as WryMenuId, MenuItem as WryMenuItem, MenuItemAttributes as WryMenuItemAttributes,
      MenuType,
    },
    monitor::MonitorHandle,
    window::{
      CursorIcon as WryCursorIcon, Fullscreen, Icon as WryWindowIcon, Theme as WryTheme,
      UserAttentionType as WryUserAttentionType,
    },
  },
  http::{Request as WryRequest, Response as WryResponse},
  webview::{FileDropEvent as WryFileDropEvent, WebContext, WebView, WebViewBuilder},
};

pub use wry;
pub use wry::application::window::{Window, WindowBuilder as WryWindowBuilder, WindowId};

#[cfg(windows)]
use wry::webview::WebviewExtWindows;

#[cfg(target_os = "macos")]
use tauri_runtime::{menu::NativeImage, ActivationPolicy};
#[cfg(target_os = "macos")]
pub use wry::application::platform::macos::{
  ActivationPolicy as WryActivationPolicy, CustomMenuItemExtMacOS, EventLoopExtMacOS,
  NativeImage as WryNativeImage, WindowExtMacOS,
};

use std::{
  cell::RefCell,
  collections::{
    hash_map::Entry::{Occupied, Vacant},
    HashMap, HashSet,
  },
  fmt,
  ops::Deref,
  path::PathBuf,
  sync::{
    mpsc::{channel, Sender},
    Arc, Mutex, Weak,
  },
  thread::{current as current_thread, ThreadId},
};

pub type WebviewId = u64;
type IpcHandler = dyn Fn(&Window, String) + 'static;
type FileDropHandler = dyn Fn(&Window, WryFileDropEvent) -> bool + 'static;
#[cfg(all(desktop, feature = "system-tray"))]
pub use tauri_runtime::TrayId;

#[cfg(desktop)]
mod webview;
#[cfg(desktop)]
pub use webview::Webview;

#[cfg(all(desktop, feature = "system-tray"))]
mod system_tray;
#[cfg(all(desktop, feature = "system-tray"))]
use system_tray::*;

#[cfg(all(desktop, feature = "global-shortcut"))]
mod global_shortcut;
#[cfg(all(desktop, feature = "global-shortcut"))]
use global_shortcut::*;

#[cfg(feature = "clipboard")]
mod clipboard;
#[cfg(feature = "clipboard")]
use clipboard::*;

pub type WebContextStore = Arc<Mutex<HashMap<Option<PathBuf>, WebContext>>>;
// window
pub type WindowEventHandler = Box<dyn Fn(&WindowEvent) + Send>;
pub type WindowEventListeners = Arc<Mutex<HashMap<Uuid, WindowEventHandler>>>;
// menu
pub type MenuEventHandler = Box<dyn Fn(&MenuEvent) + Send>;
pub type WindowMenuEventListeners = Arc<Mutex<HashMap<Uuid, MenuEventHandler>>>;

#[derive(Debug, Clone, Default)]
pub struct WebviewIdStore(Arc<Mutex<HashMap<WindowId, WebviewId>>>);

impl WebviewIdStore {
  pub fn insert(&self, w: WindowId, id: WebviewId) {
    self.0.lock().unwrap().insert(w, id);
  }

  fn get(&self, w: &WindowId) -> Option<WebviewId> {
    self.0.lock().unwrap().get(w).copied()
  }
}

#[macro_export]
macro_rules! getter {
  ($self: ident, $rx: expr, $message: expr) => {{
    $crate::send_user_message(&$self.context, $message)?;
    $rx
      .recv()
      .map_err(|_| $crate::Error::FailedToReceiveMessage)
  }};
}

macro_rules! window_getter {
  ($self: ident, $message: expr) => {{
    let (tx, rx) = channel();
    getter!($self, rx, Message::Window($self.window_id, $message(tx)))
  }};
}

pub(crate) fn send_user_message<T: UserEvent>(
  context: &Context<T>,
  message: Message<T>,
) -> Result<()> {
  if current_thread().id() == context.main_thread_id {
    handle_user_message(
      &context.main_thread.window_target,
      message,
      UserMessageContext {
        webview_id_map: context.webview_id_map.clone(),
        #[cfg(all(desktop, feature = "global-shortcut"))]
        global_shortcut_manager: context.main_thread.global_shortcut_manager.clone(),
        #[cfg(feature = "clipboard")]
        clipboard_manager: context.main_thread.clipboard_manager.clone(),
        windows: context.main_thread.windows.clone(),
        #[cfg(all(desktop, feature = "system-tray"))]
        system_tray_manager: context.main_thread.system_tray_manager.clone(),
      },
      &context.main_thread.web_context,
    );
    Ok(())
  } else {
    context
      .proxy
      .send_event(message)
      .map_err(|_| Error::FailedToSendMessage)
  }
}

#[derive(Clone)]
pub struct Context<T: UserEvent> {
  pub webview_id_map: WebviewIdStore,
  main_thread_id: ThreadId,
  pub proxy: WryEventLoopProxy<Message<T>>,
  main_thread: DispatcherMainThreadContext<T>,
}

impl<T: UserEvent> Context<T> {
  pub fn run_threaded<R, F>(&self, f: F) -> R
  where
    F: FnOnce(Option<&DispatcherMainThreadContext<T>>) -> R,
  {
    f(if current_thread().id() == self.main_thread_id {
      Some(&self.main_thread)
    } else {
      None
    })
  }
}

impl<T: UserEvent> Context<T> {
  fn create_webview(&self, pending: PendingWindow<T, Wry<T>>) -> Result<DetachedWindow<T, Wry<T>>> {
    let label = pending.label.clone();
    let menu_ids = pending.menu_ids.clone();
    let js_event_listeners = pending.js_event_listeners.clone();
    let context = self.clone();
    let window_id = rand::random();

    send_user_message(
      self,
      Message::CreateWebview(
        window_id,
        Box::new(move |event_loop, web_context| {
          create_webview(window_id, event_loop, web_context, context, pending)
        }),
      ),
    )?;

    let dispatcher = WryDispatcher {
      window_id,
      context: self.clone(),
    };
    Ok(DetachedWindow {
      label,
      dispatcher,
      menu_ids,
      js_event_listeners,
    })
  }
}

#[derive(Debug, Clone)]
pub struct DispatcherMainThreadContext<T: UserEvent> {
  pub window_target: EventLoopWindowTarget<Message<T>>,
  pub web_context: WebContextStore,
  #[cfg(all(desktop, feature = "global-shortcut"))]
  pub global_shortcut_manager: Arc<Mutex<WryShortcutManager>>,
  #[cfg(feature = "clipboard")]
  pub clipboard_manager: Arc<Mutex<Clipboard>>,
  pub windows: Arc<RefCell<HashMap<WebviewId, WindowWrapper>>>,
  #[cfg(all(desktop, feature = "system-tray"))]
  system_tray_manager: SystemTrayManager,
}

// SAFETY: we ensure this type is only used on the main thread.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: UserEvent> Send for DispatcherMainThreadContext<T> {}

// SAFETY: we ensure this type is only used on the main thread.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: UserEvent> Sync for DispatcherMainThreadContext<T> {}

impl<T: UserEvent> fmt::Debug for Context<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("Context")
      .field("main_thread_id", &self.main_thread_id)
      .field("proxy", &self.proxy)
      .field("main_thread", &self.main_thread)
      .finish()
  }
}

struct HttpRequestWrapper(HttpRequest);

impl From<&WryRequest<Vec<u8>>> for HttpRequestWrapper {
  fn from(req: &WryRequest<Vec<u8>>) -> Self {
    let parts = RequestParts {
      uri: req.uri().to_string(),
      method: req.method().clone(),
      headers: req.headers().clone(),
    };
    Self(HttpRequest::new_internal(parts, req.body().clone()))
  }
}

// response
struct HttpResponseWrapper(WryResponse<Vec<u8>>);
impl From<HttpResponse> for HttpResponseWrapper {
  fn from(response: HttpResponse) -> Self {
    let (parts, body) = response.into_parts();
    let mut res_builder = WryResponse::builder()
      .status(parts.status)
      .version(parts.version);
    if let Some(mime) = parts.mimetype {
      res_builder = res_builder.header(CONTENT_TYPE, mime);
    }
    for (name, val) in parts.headers.iter() {
      res_builder = res_builder.header(name, val);
    }

    let res = res_builder.body(body).unwrap();
    Self(res)
  }
}

pub struct MenuItemAttributesWrapper<'a>(pub WryMenuItemAttributes<'a>);

impl<'a> From<&'a CustomMenuItem> for MenuItemAttributesWrapper<'a> {
  fn from(item: &'a CustomMenuItem) -> Self {
    let mut attributes = WryMenuItemAttributes::new(&item.title)
      .with_enabled(item.enabled)
      .with_selected(item.selected)
      .with_id(WryMenuId(item.id));
    if let Some(accelerator) = item.keyboard_accelerator.as_ref() {
      attributes = attributes.with_accelerators(&accelerator.parse().expect("invalid accelerator"));
    }
    Self(attributes)
  }
}

pub struct AboutMetadataWrapper(pub WryAboutMetadata);

impl From<AboutMetadata> for AboutMetadataWrapper {
  fn from(metadata: AboutMetadata) -> Self {
    Self(WryAboutMetadata {
      version: metadata.version,
      authors: metadata.authors,
      comments: metadata.comments,
      copyright: metadata.copyright,
      license: metadata.license,
      website: metadata.website,
      website_label: metadata.website_label,
    })
  }
}

pub struct MenuItemWrapper(pub WryMenuItem);

impl From<MenuItem> for MenuItemWrapper {
  fn from(item: MenuItem) -> Self {
    match item {
      MenuItem::About(name, metadata) => Self(WryMenuItem::About(
        name,
        AboutMetadataWrapper::from(metadata).0,
      )),
      MenuItem::Hide => Self(WryMenuItem::Hide),
      MenuItem::Services => Self(WryMenuItem::Services),
      MenuItem::HideOthers => Self(WryMenuItem::HideOthers),
      MenuItem::ShowAll => Self(WryMenuItem::ShowAll),
      MenuItem::CloseWindow => Self(WryMenuItem::CloseWindow),
      MenuItem::Quit => Self(WryMenuItem::Quit),
      MenuItem::Copy => Self(WryMenuItem::Copy),
      MenuItem::Cut => Self(WryMenuItem::Cut),
      MenuItem::Undo => Self(WryMenuItem::Undo),
      MenuItem::Redo => Self(WryMenuItem::Redo),
      MenuItem::SelectAll => Self(WryMenuItem::SelectAll),
      MenuItem::Paste => Self(WryMenuItem::Paste),
      MenuItem::EnterFullScreen => Self(WryMenuItem::EnterFullScreen),
      MenuItem::Minimize => Self(WryMenuItem::Minimize),
      MenuItem::Zoom => Self(WryMenuItem::Zoom),
      MenuItem::Separator => Self(WryMenuItem::Separator),
      _ => unimplemented!(),
    }
  }
}

#[cfg(target_os = "macos")]
pub struct NativeImageWrapper(pub WryNativeImage);

#[cfg(target_os = "macos")]
impl std::fmt::Debug for NativeImageWrapper {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("NativeImageWrapper").finish()
  }
}

#[cfg(target_os = "macos")]
impl From<NativeImage> for NativeImageWrapper {
  fn from(image: NativeImage) -> NativeImageWrapper {
    let wry_image = match image {
      NativeImage::Add => WryNativeImage::Add,
      NativeImage::Advanced => WryNativeImage::Advanced,
      NativeImage::Bluetooth => WryNativeImage::Bluetooth,
      NativeImage::Bookmarks => WryNativeImage::Bookmarks,
      NativeImage::Caution => WryNativeImage::Caution,
      NativeImage::ColorPanel => WryNativeImage::ColorPanel,
      NativeImage::ColumnView => WryNativeImage::ColumnView,
      NativeImage::Computer => WryNativeImage::Computer,
      NativeImage::EnterFullScreen => WryNativeImage::EnterFullScreen,
      NativeImage::Everyone => WryNativeImage::Everyone,
      NativeImage::ExitFullScreen => WryNativeImage::ExitFullScreen,
      NativeImage::FlowView => WryNativeImage::FlowView,
      NativeImage::Folder => WryNativeImage::Folder,
      NativeImage::FolderBurnable => WryNativeImage::FolderBurnable,
      NativeImage::FolderSmart => WryNativeImage::FolderSmart,
      NativeImage::FollowLinkFreestanding => WryNativeImage::FollowLinkFreestanding,
      NativeImage::FontPanel => WryNativeImage::FontPanel,
      NativeImage::GoLeft => WryNativeImage::GoLeft,
      NativeImage::GoRight => WryNativeImage::GoRight,
      NativeImage::Home => WryNativeImage::Home,
      NativeImage::IChatTheater => WryNativeImage::IChatTheater,
      NativeImage::IconView => WryNativeImage::IconView,
      NativeImage::Info => WryNativeImage::Info,
      NativeImage::InvalidDataFreestanding => WryNativeImage::InvalidDataFreestanding,
      NativeImage::LeftFacingTriangle => WryNativeImage::LeftFacingTriangle,
      NativeImage::ListView => WryNativeImage::ListView,
      NativeImage::LockLocked => WryNativeImage::LockLocked,
      NativeImage::LockUnlocked => WryNativeImage::LockUnlocked,
      NativeImage::MenuMixedState => WryNativeImage::MenuMixedState,
      NativeImage::MenuOnState => WryNativeImage::MenuOnState,
      NativeImage::MobileMe => WryNativeImage::MobileMe,
      NativeImage::MultipleDocuments => WryNativeImage::MultipleDocuments,
      NativeImage::Network => WryNativeImage::Network,
      NativeImage::Path => WryNativeImage::Path,
      NativeImage::PreferencesGeneral => WryNativeImage::PreferencesGeneral,
      NativeImage::QuickLook => WryNativeImage::QuickLook,
      NativeImage::RefreshFreestanding => WryNativeImage::RefreshFreestanding,
      NativeImage::Refresh => WryNativeImage::Refresh,
      NativeImage::Remove => WryNativeImage::Remove,
      NativeImage::RevealFreestanding => WryNativeImage::RevealFreestanding,
      NativeImage::RightFacingTriangle => WryNativeImage::RightFacingTriangle,
      NativeImage::Share => WryNativeImage::Share,
      NativeImage::Slideshow => WryNativeImage::Slideshow,
      NativeImage::SmartBadge => WryNativeImage::SmartBadge,
      NativeImage::StatusAvailable => WryNativeImage::StatusAvailable,
      NativeImage::StatusNone => WryNativeImage::StatusNone,
      NativeImage::StatusPartiallyAvailable => WryNativeImage::StatusPartiallyAvailable,
      NativeImage::StatusUnavailable => WryNativeImage::StatusUnavailable,
      NativeImage::StopProgressFreestanding => WryNativeImage::StopProgressFreestanding,
      NativeImage::StopProgress => WryNativeImage::StopProgress,

      NativeImage::TrashEmpty => WryNativeImage::TrashEmpty,
      NativeImage::TrashFull => WryNativeImage::TrashFull,
      NativeImage::User => WryNativeImage::User,
      NativeImage::UserAccounts => WryNativeImage::UserAccounts,
      NativeImage::UserGroup => WryNativeImage::UserGroup,
      NativeImage::UserGuest => WryNativeImage::UserGuest,
    };
    Self(wry_image)
  }
}

/// Wrapper around a [`wry::application::window::Icon`] that can be created from an [`Icon`].
pub struct WryIcon(pub WryWindowIcon);

fn icon_err<E: std::error::Error + Send + Sync + 'static>(e: E) -> Error {
  Error::InvalidIcon(Box::new(e))
}

impl TryFrom<Icon> for WryIcon {
  type Error = Error;
  fn try_from(icon: Icon) -> std::result::Result<Self, Self::Error> {
    WryWindowIcon::from_rgba(icon.rgba, icon.width, icon.height)
      .map(Self)
      .map_err(icon_err)
  }
}

pub struct WindowEventWrapper(pub Option<WindowEvent>);

impl WindowEventWrapper {
  fn parse(webview: &Option<WindowHandle>, event: &WryWindowEvent<'_>) -> Self {
    match event {
      // resized event from tao doesn't include a reliable size on macOS
      // because wry replaces the NSView
      WryWindowEvent::Resized(_) => {
        if let Some(webview) = webview {
          Self(Some(WindowEvent::Resized(
            PhysicalSizeWrapper(webview.inner_size()).into(),
          )))
        } else {
          Self(None)
        }
      }
      e => e.into(),
    }
  }
}

pub fn map_theme(theme: &WryTheme) -> Theme {
  match theme {
    WryTheme::Light => Theme::Light,
    WryTheme::Dark => Theme::Dark,
    _ => Theme::Light,
  }
}

impl<'a> From<&WryWindowEvent<'a>> for WindowEventWrapper {
  fn from(event: &WryWindowEvent<'a>) -> Self {
    let event = match event {
      WryWindowEvent::Resized(size) => WindowEvent::Resized(PhysicalSizeWrapper(*size).into()),
      WryWindowEvent::Moved(position) => {
        WindowEvent::Moved(PhysicalPositionWrapper(*position).into())
      }
      WryWindowEvent::Destroyed => WindowEvent::Destroyed,
      WryWindowEvent::ScaleFactorChanged {
        scale_factor,
        new_inner_size,
      } => WindowEvent::ScaleFactorChanged {
        scale_factor: *scale_factor,
        new_inner_size: PhysicalSizeWrapper(**new_inner_size).into(),
      },
      #[cfg(any(target_os = "linux", target_os = "macos"))]
      WryWindowEvent::Focused(focused) => WindowEvent::Focused(*focused),
      WryWindowEvent::ThemeChanged(theme) => WindowEvent::ThemeChanged(map_theme(theme)),
      _ => return Self(None),
    };
    Self(Some(event))
  }
}

impl From<&WebviewEvent> for WindowEventWrapper {
  fn from(event: &WebviewEvent) -> Self {
    let event = match event {
      WebviewEvent::Focused(focused) => WindowEvent::Focused(*focused),
    };
    Self(Some(event))
  }
}

pub struct MonitorHandleWrapper(pub MonitorHandle);

impl From<MonitorHandleWrapper> for Monitor {
  fn from(monitor: MonitorHandleWrapper) -> Monitor {
    Self {
      name: monitor.0.name(),
      position: PhysicalPositionWrapper(monitor.0.position()).into(),
      size: PhysicalSizeWrapper(monitor.0.size()).into(),
      scale_factor: monitor.0.scale_factor(),
    }
  }
}

pub struct PhysicalPositionWrapper<T>(pub WryPhysicalPosition<T>);

impl<T> From<PhysicalPositionWrapper<T>> for PhysicalPosition<T> {
  fn from(position: PhysicalPositionWrapper<T>) -> Self {
    Self {
      x: position.0.x,
      y: position.0.y,
    }
  }
}

impl<T> From<PhysicalPosition<T>> for PhysicalPositionWrapper<T> {
  fn from(position: PhysicalPosition<T>) -> Self {
    Self(WryPhysicalPosition {
      x: position.x,
      y: position.y,
    })
  }
}

struct LogicalPositionWrapper<T>(WryLogicalPosition<T>);

impl<T> From<LogicalPosition<T>> for LogicalPositionWrapper<T> {
  fn from(position: LogicalPosition<T>) -> Self {
    Self(WryLogicalPosition {
      x: position.x,
      y: position.y,
    })
  }
}

pub struct PhysicalSizeWrapper<T>(pub WryPhysicalSize<T>);

impl<T> From<PhysicalSizeWrapper<T>> for PhysicalSize<T> {
  fn from(size: PhysicalSizeWrapper<T>) -> Self {
    Self {
      width: size.0.width,
      height: size.0.height,
    }
  }
}

impl<T> From<PhysicalSize<T>> for PhysicalSizeWrapper<T> {
  fn from(size: PhysicalSize<T>) -> Self {
    Self(WryPhysicalSize {
      width: size.width,
      height: size.height,
    })
  }
}

struct LogicalSizeWrapper<T>(WryLogicalSize<T>);

impl<T> From<LogicalSize<T>> for LogicalSizeWrapper<T> {
  fn from(size: LogicalSize<T>) -> Self {
    Self(WryLogicalSize {
      width: size.width,
      height: size.height,
    })
  }
}

pub struct SizeWrapper(pub WrySize);

impl From<Size> for SizeWrapper {
  fn from(size: Size) -> Self {
    match size {
      Size::Logical(s) => Self(WrySize::Logical(LogicalSizeWrapper::from(s).0)),
      Size::Physical(s) => Self(WrySize::Physical(PhysicalSizeWrapper::from(s).0)),
    }
  }
}

pub struct PositionWrapper(pub WryPosition);

impl From<Position> for PositionWrapper {
  fn from(position: Position) -> Self {
    match position {
      Position::Logical(s) => Self(WryPosition::Logical(LogicalPositionWrapper::from(s).0)),
      Position::Physical(s) => Self(WryPosition::Physical(PhysicalPositionWrapper::from(s).0)),
    }
  }
}

#[derive(Debug, Clone)]
pub struct UserAttentionTypeWrapper(pub WryUserAttentionType);

impl From<UserAttentionType> for UserAttentionTypeWrapper {
  fn from(request_type: UserAttentionType) -> Self {
    let o = match request_type {
      UserAttentionType::Critical => WryUserAttentionType::Critical,
      UserAttentionType::Informational => WryUserAttentionType::Informational,
    };
    Self(o)
  }
}

#[derive(Debug)]
pub struct CursorIconWrapper(pub WryCursorIcon);

impl From<CursorIcon> for CursorIconWrapper {
  fn from(icon: CursorIcon) -> Self {
    use CursorIcon::*;
    let i = match icon {
      Default => WryCursorIcon::Default,
      Crosshair => WryCursorIcon::Crosshair,
      Hand => WryCursorIcon::Hand,
      Arrow => WryCursorIcon::Arrow,
      Move => WryCursorIcon::Move,
      Text => WryCursorIcon::Text,
      Wait => WryCursorIcon::Wait,
      Help => WryCursorIcon::Help,
      Progress => WryCursorIcon::Progress,
      NotAllowed => WryCursorIcon::NotAllowed,
      ContextMenu => WryCursorIcon::ContextMenu,
      Cell => WryCursorIcon::Cell,
      VerticalText => WryCursorIcon::VerticalText,
      Alias => WryCursorIcon::Alias,
      Copy => WryCursorIcon::Copy,
      NoDrop => WryCursorIcon::NoDrop,
      Grab => WryCursorIcon::Grab,
      Grabbing => WryCursorIcon::Grabbing,
      AllScroll => WryCursorIcon::AllScroll,
      ZoomIn => WryCursorIcon::ZoomIn,
      ZoomOut => WryCursorIcon::ZoomOut,
      EResize => WryCursorIcon::EResize,
      NResize => WryCursorIcon::NResize,
      NeResize => WryCursorIcon::NeResize,
      NwResize => WryCursorIcon::NwResize,
      SResize => WryCursorIcon::SResize,
      SeResize => WryCursorIcon::SeResize,
      SwResize => WryCursorIcon::SwResize,
      WResize => WryCursorIcon::WResize,
      EwResize => WryCursorIcon::EwResize,
      NsResize => WryCursorIcon::NsResize,
      NeswResize => WryCursorIcon::NeswResize,
      NwseResize => WryCursorIcon::NwseResize,
      ColResize => WryCursorIcon::ColResize,
      RowResize => WryCursorIcon::RowResize,
      _ => WryCursorIcon::Default,
    };
    Self(i)
  }
}

#[derive(Debug, Clone, Default)]
pub struct WindowBuilderWrapper {
  inner: WryWindowBuilder,
  center: bool,
  #[cfg(target_os = "macos")]
  tabbing_identifier: Option<String>,
  menu: Option<Menu>,
}

// SAFETY: this type is `Send` since `menu_items` are read only here
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for WindowBuilderWrapper {}

impl WindowBuilderBase for WindowBuilderWrapper {}
impl WindowBuilder for WindowBuilderWrapper {
  fn new() -> Self {
    Self::default().focused(true)
  }

  fn with_config(config: WindowConfig) -> Self {
    let mut window = WindowBuilderWrapper::new()
      .title(config.title.to_string())
      .inner_size(config.width, config.height)
      .visible(config.visible)
      .resizable(config.resizable)
      .fullscreen(config.fullscreen)
      .decorations(config.decorations)
      .maximized(config.maximized)
      .always_on_top(config.always_on_top)
      .skip_taskbar(config.skip_taskbar)
      .theme(config.theme);

    #[cfg(target_os = "macos")]
    {
      window = window
        .hidden_title(config.hidden_title)
        .title_bar_style(config.title_bar_style);
      if let Some(identifier) = &config.tabbing_identifier {
        window = window.tabbing_identifier(identifier);
      }
    }

    #[cfg(any(not(target_os = "macos"), feature = "macos-private-api"))]
    {
      window = window.transparent(config.transparent);
    }
    #[cfg(all(
      target_os = "macos",
      not(feature = "macos-private-api"),
      debug_assertions
    ))]
    if config.transparent {
      eprintln!(
        "The window is set to be transparent but the `macos-private-api` is not enabled.
        This can be enabled via the `tauri.macOSPrivateApi` configuration property <https://tauri.app/docs/api/config#tauri.macOSPrivateApi>
      ");
    }

    if let (Some(min_width), Some(min_height)) = (config.min_width, config.min_height) {
      window = window.min_inner_size(min_width, min_height);
    }
    if let (Some(max_width), Some(max_height)) = (config.max_width, config.max_height) {
      window = window.max_inner_size(max_width, max_height);
    }
    if let (Some(x), Some(y)) = (config.x, config.y) {
      window = window.position(x, y);
    }

    if config.center {
      window = window.center();
    }

    window
  }

  fn menu(mut self, menu: Menu) -> Self {
    self.menu.replace(menu);
    self
  }

  fn center(mut self) -> Self {
    self.center = true;
    self
  }

  fn position(mut self, x: f64, y: f64) -> Self {
    self.inner = self.inner.with_position(WryLogicalPosition::new(x, y));
    self
  }

  fn inner_size(mut self, width: f64, height: f64) -> Self {
    self.inner = self
      .inner
      .with_inner_size(WryLogicalSize::new(width, height));
    self
  }

  fn min_inner_size(mut self, min_width: f64, min_height: f64) -> Self {
    self.inner = self
      .inner
      .with_min_inner_size(WryLogicalSize::new(min_width, min_height));
    self
  }

  fn max_inner_size(mut self, max_width: f64, max_height: f64) -> Self {
    self.inner = self
      .inner
      .with_max_inner_size(WryLogicalSize::new(max_width, max_height));
    self
  }

  fn resizable(mut self, resizable: bool) -> Self {
    self.inner = self.inner.with_resizable(resizable);
    self
  }

  fn title<S: Into<String>>(mut self, title: S) -> Self {
    self.inner = self.inner.with_title(title.into());
    self
  }

  fn fullscreen(mut self, fullscreen: bool) -> Self {
    self.inner = if fullscreen {
      self
        .inner
        .with_fullscreen(Some(Fullscreen::Borderless(None)))
    } else {
      self.inner.with_fullscreen(None)
    };
    self
  }

  fn focused(mut self, focused: bool) -> Self {
    self.inner = self.inner.with_focused(focused);
    self
  }

  fn maximized(mut self, maximized: bool) -> Self {
    self.inner = self.inner.with_maximized(maximized);
    self
  }

  fn visible(mut self, visible: bool) -> Self {
    self.inner = self.inner.with_visible(visible);
    self
  }

  #[cfg(any(not(target_os = "macos"), feature = "macos-private-api"))]
  fn transparent(mut self, transparent: bool) -> Self {
    self.inner = self.inner.with_transparent(transparent);
    self
  }

  fn decorations(mut self, decorations: bool) -> Self {
    self.inner = self.inner.with_decorations(decorations);
    self
  }

  fn always_on_top(mut self, always_on_top: bool) -> Self {
    self.inner = self.inner.with_always_on_top(always_on_top);
    self
  }

  #[cfg(windows)]
  fn parent_window(mut self, parent: HWND) -> Self {
    self.inner = self.inner.with_parent_window(parent);
    self
  }

  #[cfg(target_os = "macos")]
  fn parent_window(mut self, parent: *mut std::ffi::c_void) -> Self {
    self.inner = self.inner.with_parent_window(parent);
    self
  }

  #[cfg(windows)]
  fn owner_window(mut self, owner: HWND) -> Self {
    self.inner = self.inner.with_owner_window(owner);
    self
  }

  #[cfg(target_os = "macos")]
  fn title_bar_style(mut self, style: TitleBarStyle) -> Self {
    match style {
      TitleBarStyle::Visible => {
        self.inner = self.inner.with_titlebar_transparent(false);
        // Fixes rendering issue when resizing window with devtools open (https://github.com/tauri-apps/tauri/issues/3914)
        self.inner = self.inner.with_fullsize_content_view(true);
      }
      TitleBarStyle::Transparent => {
        self.inner = self.inner.with_titlebar_transparent(true);
        self.inner = self.inner.with_fullsize_content_view(false);
      }
      TitleBarStyle::Overlay => {
        self.inner = self.inner.with_titlebar_transparent(true);
        self.inner = self.inner.with_fullsize_content_view(true);
      }
    }
    self
  }

  #[cfg(target_os = "macos")]
  fn hidden_title(mut self, hidden: bool) -> Self {
    self.inner = self.inner.with_title_hidden(hidden);
    self
  }

  #[cfg(target_os = "macos")]
  fn tabbing_identifier(mut self, identifier: &str) -> Self {
    self.inner = self.inner.with_tabbing_identifier(identifier);
    self.tabbing_identifier.replace(identifier.into());
    self
  }

  fn icon(mut self, icon: Icon) -> Result<Self> {
    self.inner = self
      .inner
      .with_window_icon(Some(WryIcon::try_from(icon)?.0));
    Ok(self)
  }

  #[cfg(any(windows, target_os = "linux"))]
  fn skip_taskbar(mut self, skip: bool) -> Self {
    self.inner = self.inner.with_skip_taskbar(skip);
    self
  }

  #[cfg(any(target_os = "macos", target_os = "ios", target_os = "android"))]
  fn skip_taskbar(self, _skip: bool) -> Self {
    self
  }

  #[allow(unused_variables, unused_mut)]
  fn theme(mut self, theme: Option<Theme>) -> Self {
    self.inner = self.inner.with_theme(if let Some(t) = theme {
      match t {
        Theme::Dark => Some(WryTheme::Dark),
        _ => Some(WryTheme::Light),
      }
    } else {
      None
    });

    self
  }

  fn has_icon(&self) -> bool {
    self.inner.window.window_icon.is_some()
  }

  fn get_menu(&self) -> Option<&Menu> {
    self.menu.as_ref()
  }
}

pub struct FileDropEventWrapper(WryFileDropEvent);

// on Linux, the paths are percent-encoded
#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
fn decode_path(path: PathBuf) -> PathBuf {
  percent_encoding::percent_decode(path.display().to_string().as_bytes())
    .decode_utf8_lossy()
    .into_owned()
    .into()
}

// on Windows and macOS, we do not need to decode the path
#[cfg(not(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
)))]
fn decode_path(path: PathBuf) -> PathBuf {
  path
}

impl From<FileDropEventWrapper> for FileDropEvent {
  fn from(event: FileDropEventWrapper) -> Self {
    match event.0 {
      WryFileDropEvent::Hovered(paths) => {
        FileDropEvent::Hovered(paths.into_iter().map(decode_path).collect())
      }
      WryFileDropEvent::Dropped(paths) => {
        FileDropEvent::Dropped(paths.into_iter().map(decode_path).collect())
      }
      // default to cancelled
      // FIXME(maybe): Add `FileDropEvent::Unknown` event?
      _ => FileDropEvent::Cancelled,
    }
  }
}

#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
pub struct GtkWindow(pub gtk::ApplicationWindow);
#[cfg(any(
  target_os = "linux",
  target_os = "dragonfly",
  target_os = "freebsd",
  target_os = "netbsd",
  target_os = "openbsd"
))]
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl Send for GtkWindow {}

pub struct RawWindowHandle(pub raw_window_handle::RawWindowHandle);
unsafe impl Send for RawWindowHandle {}

#[cfg(target_os = "macos")]
#[derive(Debug, Clone)]
pub enum ApplicationMessage {
  Show,
  Hide,
}

pub enum WindowMessage {
  #[cfg(desktop)]
  WithWebview(Box<dyn FnOnce(Webview) + Send>),
  AddEventListener(Uuid, Box<dyn Fn(&WindowEvent) + Send>),
  AddMenuEventListener(Uuid, Box<dyn Fn(&MenuEvent) + Send>),
  // Devtools
  #[cfg(any(debug_assertions, feature = "devtools"))]
  OpenDevTools,
  #[cfg(any(debug_assertions, feature = "devtools"))]
  CloseDevTools,
  #[cfg(any(debug_assertions, feature = "devtools"))]
  IsDevToolsOpen(Sender<bool>),
  // Getters
  ScaleFactor(Sender<f64>),
  InnerPosition(Sender<Result<PhysicalPosition<i32>>>),
  OuterPosition(Sender<Result<PhysicalPosition<i32>>>),
  InnerSize(Sender<PhysicalSize<u32>>),
  OuterSize(Sender<PhysicalSize<u32>>),
  IsFullscreen(Sender<bool>),
  IsMaximized(Sender<bool>),
  IsDecorated(Sender<bool>),
  IsResizable(Sender<bool>),
  IsVisible(Sender<bool>),
  IsMenuVisible(Sender<bool>),
  CurrentMonitor(Sender<Option<MonitorHandle>>),
  PrimaryMonitor(Sender<Option<MonitorHandle>>),
  AvailableMonitors(Sender<Vec<MonitorHandle>>),
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  GtkWindow(Sender<GtkWindow>),
  RawWindowHandle(Sender<RawWindowHandle>),
  Theme(Sender<Theme>),
  // Setters
  Center,
  RequestUserAttention(Option<UserAttentionTypeWrapper>),
  SetResizable(bool),
  SetTitle(String),
  Maximize,
  Unmaximize,
  Minimize,
  Unminimize,
  ShowMenu,
  HideMenu,
  Show,
  Hide,
  Close,
  SetDecorations(bool),
  SetAlwaysOnTop(bool),
  SetSize(Size),
  SetMinSize(Option<Size>),
  SetMaxSize(Option<Size>),
  SetPosition(Position),
  SetFullscreen(bool),
  SetFocus,
  SetIcon(WryWindowIcon),
  SetSkipTaskbar(bool),
  SetCursorGrab(bool),
  SetCursorVisible(bool),
  SetCursorIcon(CursorIcon),
  SetCursorPosition(Position),
  SetIgnoreCursorEvents(bool),
  DragWindow,
  UpdateMenuItem(u16, MenuUpdate),
  RequestRedraw,
}

#[derive(Debug, Clone)]
pub enum WebviewMessage {
  EvaluateScript(String),
  #[allow(dead_code)]
  WebviewEvent(WebviewEvent),
  Print,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum WebviewEvent {
  Focused(bool),
}

#[cfg(all(desktop, feature = "system-tray"))]
#[derive(Debug, Clone)]
pub enum TrayMessage {
  UpdateItem(u16, MenuUpdate),
  UpdateMenu(SystemTrayMenu),
  UpdateIcon(Icon),
  #[cfg(target_os = "macos")]
  UpdateIconAsTemplate(bool),
  #[cfg(target_os = "macos")]
  UpdateTitle(String),
  Create(SystemTray, Sender<Result<()>>),
  Destroy(Sender<Result<()>>),
}

pub type CreateWebviewClosure<T> = Box<
  dyn FnOnce(&EventLoopWindowTarget<Message<T>>, &WebContextStore) -> Result<WindowWrapper> + Send,
>;

pub enum Message<T: 'static> {
  Task(Box<dyn FnOnce() + Send>),
  #[cfg(target_os = "macos")]
  Application(ApplicationMessage),
  Window(WebviewId, WindowMessage),
  Webview(WebviewId, WebviewMessage),
  #[cfg(all(desktop, feature = "system-tray"))]
  Tray(TrayId, TrayMessage),
  CreateWebview(WebviewId, CreateWebviewClosure<T>),
  CreateWindow(
    WebviewId,
    Box<dyn FnOnce() -> (String, WryWindowBuilder) + Send>,
    Sender<Result<Weak<Window>>>,
  ),
  #[cfg(all(desktop, feature = "global-shortcut"))]
  GlobalShortcut(GlobalShortcutMessage),
  #[cfg(feature = "clipboard")]
  Clipboard(ClipboardMessage),
  UserEvent(T),
}

impl<T: UserEvent> Clone for Message<T> {
  fn clone(&self) -> Self {
    match self {
      Self::Webview(i, m) => Self::Webview(*i, m.clone()),
      #[cfg(all(desktop, feature = "system-tray"))]
      Self::Tray(i, m) => Self::Tray(*i, m.clone()),
      #[cfg(all(desktop, feature = "global-shortcut"))]
      Self::GlobalShortcut(m) => Self::GlobalShortcut(m.clone()),
      #[cfg(feature = "clipboard")]
      Self::Clipboard(m) => Self::Clipboard(m.clone()),
      Self::UserEvent(t) => Self::UserEvent(t.clone()),
      _ => unimplemented!(),
    }
  }
}

/// The Tauri [`Dispatch`] for [`Wry`].
#[derive(Debug, Clone)]
pub struct WryDispatcher<T: UserEvent> {
  window_id: WebviewId,
  context: Context<T>,
}

// SAFETY: this is safe since the `Context` usage is guarded on `send_user_message`.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: UserEvent> Sync for WryDispatcher<T> {}

impl<T: UserEvent> WryDispatcher<T> {
  #[cfg(desktop)]
  pub fn with_webview<F: FnOnce(Webview) + Send + 'static>(&self, f: F) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::WithWebview(Box::new(f))),
    )
  }
}

impl<T: UserEvent> Dispatch<T> for WryDispatcher<T> {
  type Runtime = Wry<T>;
  type WindowBuilder = WindowBuilderWrapper;

  fn run_on_main_thread<F: FnOnce() + Send + 'static>(&self, f: F) -> Result<()> {
    send_user_message(&self.context, Message::Task(Box::new(f)))
  }

  fn on_window_event<F: Fn(&WindowEvent) + Send + 'static>(&self, f: F) -> Uuid {
    let id = Uuid::new_v4();
    let _ = self.context.proxy.send_event(Message::Window(
      self.window_id,
      WindowMessage::AddEventListener(id, Box::new(f)),
    ));
    id
  }

  fn on_menu_event<F: Fn(&MenuEvent) + Send + 'static>(&self, f: F) -> Uuid {
    let id = Uuid::new_v4();
    let _ = self.context.proxy.send_event(Message::Window(
      self.window_id,
      WindowMessage::AddMenuEventListener(id, Box::new(f)),
    ));
    id
  }

  #[cfg(any(debug_assertions, feature = "devtools"))]
  fn open_devtools(&self) {
    let _ = send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::OpenDevTools),
    );
  }

  #[cfg(any(debug_assertions, feature = "devtools"))]
  fn close_devtools(&self) {
    let _ = send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::CloseDevTools),
    );
  }

  /// Gets the devtools window's current open state.
  #[cfg(any(debug_assertions, feature = "devtools"))]
  fn is_devtools_open(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsDevToolsOpen)
  }

  // Getters

  fn scale_factor(&self) -> Result<f64> {
    window_getter!(self, WindowMessage::ScaleFactor)
  }

  fn inner_position(&self) -> Result<PhysicalPosition<i32>> {
    window_getter!(self, WindowMessage::InnerPosition)?
  }

  fn outer_position(&self) -> Result<PhysicalPosition<i32>> {
    window_getter!(self, WindowMessage::OuterPosition)?
  }

  fn inner_size(&self) -> Result<PhysicalSize<u32>> {
    window_getter!(self, WindowMessage::InnerSize)
  }

  fn outer_size(&self) -> Result<PhysicalSize<u32>> {
    window_getter!(self, WindowMessage::OuterSize)
  }

  fn is_fullscreen(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsFullscreen)
  }

  fn is_maximized(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsMaximized)
  }

  /// Gets the window’s current decoration state.
  fn is_decorated(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsDecorated)
  }

  /// Gets the window’s current resizable state.
  fn is_resizable(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsResizable)
  }

  fn is_visible(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsVisible)
  }

  fn is_menu_visible(&self) -> Result<bool> {
    window_getter!(self, WindowMessage::IsMenuVisible)
  }

  fn current_monitor(&self) -> Result<Option<Monitor>> {
    Ok(window_getter!(self, WindowMessage::CurrentMonitor)?.map(|m| MonitorHandleWrapper(m).into()))
  }

  fn primary_monitor(&self) -> Result<Option<Monitor>> {
    Ok(window_getter!(self, WindowMessage::PrimaryMonitor)?.map(|m| MonitorHandleWrapper(m).into()))
  }

  fn available_monitors(&self) -> Result<Vec<Monitor>> {
    Ok(
      window_getter!(self, WindowMessage::AvailableMonitors)?
        .into_iter()
        .map(|m| MonitorHandleWrapper(m).into())
        .collect(),
    )
  }

  fn theme(&self) -> Result<Theme> {
    window_getter!(self, WindowMessage::Theme)
  }

  /// Returns the `ApplicationWindow` from gtk crate that is used by this window.
  #[cfg(any(
    target_os = "linux",
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "netbsd",
    target_os = "openbsd"
  ))]
  fn gtk_window(&self) -> Result<gtk::ApplicationWindow> {
    window_getter!(self, WindowMessage::GtkWindow).map(|w| w.0)
  }

  fn raw_window_handle(&self) -> Result<raw_window_handle::RawWindowHandle> {
    window_getter!(self, WindowMessage::RawWindowHandle).map(|w| w.0)
  }

  // Setters

  fn center(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Center),
    )
  }

  fn print(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Webview(self.window_id, WebviewMessage::Print),
    )
  }

  fn request_user_attention(&self, request_type: Option<UserAttentionType>) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(
        self.window_id,
        WindowMessage::RequestUserAttention(request_type.map(Into::into)),
      ),
    )
  }

  // Creates a window by dispatching a message to the event loop.
  // Note that this must be called from a separate thread, otherwise the channel will introduce a deadlock.
  fn create_window(
    &mut self,
    pending: PendingWindow<T, Self::Runtime>,
  ) -> Result<DetachedWindow<T, Self::Runtime>> {
    self.context.create_webview(pending)
  }

  fn set_resizable(&self, resizable: bool) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetResizable(resizable)),
    )
  }

  fn set_title<S: Into<String>>(&self, title: S) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetTitle(title.into())),
    )
  }

  fn maximize(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Maximize),
    )
  }

  fn unmaximize(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Unmaximize),
    )
  }

  fn minimize(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Minimize),
    )
  }

  fn unminimize(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Unminimize),
    )
  }

  fn show_menu(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::ShowMenu),
    )
  }

  fn hide_menu(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::HideMenu),
    )
  }

  fn show(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Show),
    )
  }

  fn hide(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::Hide),
    )
  }

  fn close(&self) -> Result<()> {
    // NOTE: close cannot use the `send_user_message` function because it accesses the event loop callback
    self
      .context
      .proxy
      .send_event(Message::Window(self.window_id, WindowMessage::Close))
      .map_err(|_| Error::FailedToSendMessage)
  }

  fn set_decorations(&self, decorations: bool) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetDecorations(decorations)),
    )
  }

  fn set_always_on_top(&self, always_on_top: bool) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetAlwaysOnTop(always_on_top)),
    )
  }

  fn set_size(&self, size: Size) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetSize(size)),
    )
  }

  fn set_min_size(&self, size: Option<Size>) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetMinSize(size)),
    )
  }

  fn set_max_size(&self, size: Option<Size>) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetMaxSize(size)),
    )
  }

  fn set_position(&self, position: Position) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetPosition(position)),
    )
  }

  fn set_fullscreen(&self, fullscreen: bool) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetFullscreen(fullscreen)),
    )
  }

  fn set_focus(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetFocus),
    )
  }

  fn set_icon(&self, icon: Icon) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(
        self.window_id,
        WindowMessage::SetIcon(WryIcon::try_from(icon)?.0),
      ),
    )
  }

  fn set_skip_taskbar(&self, skip: bool) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetSkipTaskbar(skip)),
    )
  }

  fn set_cursor_grab(&self, grab: bool) -> crate::Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetCursorGrab(grab)),
    )
  }

  fn set_cursor_visible(&self, visible: bool) -> crate::Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetCursorVisible(visible)),
    )
  }

  fn set_cursor_icon(&self, icon: CursorIcon) -> crate::Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetCursorIcon(icon)),
    )
  }

  fn set_cursor_position<Pos: Into<Position>>(&self, position: Pos) -> crate::Result<()> {
    send_user_message(
      &self.context,
      Message::Window(
        self.window_id,
        WindowMessage::SetCursorPosition(position.into()),
      ),
    )
  }

  fn set_ignore_cursor_events(&self, ignore: bool) -> crate::Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::SetIgnoreCursorEvents(ignore)),
    )
  }

  fn start_dragging(&self) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::DragWindow),
    )
  }

  fn eval_script<S: Into<String>>(&self, script: S) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Webview(
        self.window_id,
        WebviewMessage::EvaluateScript(script.into()),
      ),
    )
  }

  fn update_menu_item(&self, id: u16, update: MenuUpdate) -> Result<()> {
    send_user_message(
      &self.context,
      Message::Window(self.window_id, WindowMessage::UpdateMenuItem(id, update)),
    )
  }
}

#[derive(Clone)]
enum WindowHandle {
  Webview {
    inner: Arc<WebView>,
    context_store: WebContextStore,
    // the key of the WebContext if it's not shared
    context_key: Option<PathBuf>,
  },
  Window(Arc<Window>),
}

impl Drop for WindowHandle {
  fn drop(&mut self) {
    if let Self::Webview {
      inner,
      context_store,
      context_key,
    } = self
    {
      if Arc::get_mut(inner).is_some() {
        context_store.lock().unwrap().remove(context_key);
      }
    }
  }
}

impl fmt::Debug for WindowHandle {
  fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Ok(())
  }
}

impl Deref for WindowHandle {
  type Target = Window;

  #[inline(always)]
  fn deref(&self) -> &Window {
    match self {
      Self::Webview { inner, .. } => inner.window(),
      Self::Window(w) => w,
    }
  }
}

impl WindowHandle {
  fn inner_size(&self) -> WryPhysicalSize<u32> {
    match self {
      WindowHandle::Window(w) => w.inner_size(),
      WindowHandle::Webview { inner, .. } => inner.inner_size(),
    }
  }
}

pub struct WindowWrapper {
  label: String,
  inner: Option<WindowHandle>,
  menu_items: Option<HashMap<u16, WryCustomMenuItem>>,
  window_event_listeners: WindowEventListeners,
  menu_event_listeners: WindowMenuEventListeners,
}

impl fmt::Debug for WindowWrapper {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("WindowWrapper")
      .field("label", &self.label)
      .field("inner", &self.inner)
      .field("menu_items", &self.menu_items)
      .finish()
  }
}

#[derive(Debug, Clone)]
pub struct EventProxy<T: UserEvent>(WryEventLoopProxy<Message<T>>);

#[cfg(target_os = "ios")]
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: UserEvent> Sync for EventProxy<T> {}

impl<T: UserEvent> EventLoopProxy<T> for EventProxy<T> {
  fn send_event(&self, event: T) -> Result<()> {
    self
      .0
      .send_event(Message::UserEvent(event))
      .map_err(|_| Error::EventLoopClosed)
  }
}

pub trait PluginBuilder<T: UserEvent> {
  type Plugin: Plugin<T>;
  fn build(self, context: Context<T>) -> Self::Plugin;
}

pub trait Plugin<T: UserEvent> {
  fn on_event(
    &mut self,
    event: &Event<Message<T>>,
    event_loop: &EventLoopWindowTarget<Message<T>>,
    proxy: &WryEventLoopProxy<Message<T>>,
    control_flow: &mut ControlFlow,
    context: EventLoopIterationContext<'_, T>,
    web_context: &WebContextStore,
  ) -> bool;
}

/// A Tauri [`Runtime`] wrapper around wry.
pub struct Wry<T: UserEvent> {
  context: Context<T>,

  plugins: Vec<Box<dyn Plugin<T>>>,

  #[cfg(all(desktop, feature = "global-shortcut"))]
  global_shortcut_manager_handle: GlobalShortcutManagerHandle<T>,

  #[cfg(feature = "clipboard")]
  clipboard_manager_handle: ClipboardManagerWrapper<T>,

  event_loop: EventLoop<Message<T>>,
}

impl<T: UserEvent> fmt::Debug for Wry<T> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let mut d = f.debug_struct("Wry");
    d.field("main_thread_id", &self.context.main_thread_id)
      .field("event_loop", &self.event_loop)
      .field("windows", &self.context.main_thread.windows)
      .field("web_context", &self.context.main_thread.web_context);

    #[cfg(all(desktop, feature = "system-tray"))]
    d.field(
      "system_tray_manager",
      &self.context.main_thread.system_tray_manager,
    );

    #[cfg(all(desktop, feature = "global-shortcut"))]
    #[cfg(feature = "global-shortcut")]
    d.field(
      "global_shortcut_manager",
      &self.context.main_thread.global_shortcut_manager,
    )
    .field(
      "global_shortcut_manager_handle",
      &self.global_shortcut_manager_handle,
    );

    #[cfg(feature = "clipboard")]
    d.field(
      "clipboard_manager",
      &self.context.main_thread.clipboard_manager,
    )
    .field("clipboard_manager_handle", &self.clipboard_manager_handle);

    d.finish()
  }
}

/// A handle to the Wry runtime.
#[derive(Debug, Clone)]
pub struct WryHandle<T: UserEvent> {
  context: Context<T>,
}

// SAFETY: this is safe since the `Context` usage is guarded on `send_user_message`.
#[allow(clippy::non_send_fields_in_send_ty)]
unsafe impl<T: UserEvent> Sync for WryHandle<T> {}

impl<T: UserEvent> WryHandle<T> {
  /// Creates a new tao window using a callback, and returns its window id.
  pub fn create_tao_window<F: FnOnce() -> (String, WryWindowBuilder) + Send + 'static>(
    &self,
    f: F,
  ) -> Result<Weak<Window>> {
    let (tx, rx) = channel();
    send_user_message(
      &self.context,
      Message::CreateWindow(rand::random(), Box::new(f), tx),
    )?;
    rx.recv().unwrap()
  }

  /// Gets the [`WebviewId'] associated with the given [`WindowId`].
  pub fn window_id(&self, window_id: WindowId) -> WebviewId {
    *self
      .context
      .webview_id_map
      .0
      .lock()
      .unwrap()
      .get(&window_id)
      .unwrap()
  }

  /// Send a message to the event loop.
  pub fn send_event(&self, message: Message<T>) -> Result<()> {
    self
      .context
      .proxy
      .send_event(message)
      .map_err(|_| Error::FailedToSendMessage)?;
    Ok(())
  }
}

impl<T: UserEvent> RuntimeHandle<T> for WryHandle<T> {
  type Runtime = Wry<T>;

  fn create_proxy(&self) -> EventProxy<T> {
    EventProxy(self.context.proxy.clone())
  }

  // Creates a window by dispatching a message to the event loop.
  // Note that this must be called from a separate thread, otherwise the channel will introduce a deadlock.
  fn create_window(
    &self,
    pending: PendingWindow<T, Self::Runtime>,
  ) -> Result<DetachedWindow<T, Self::Runtime>> {
    self.context.create_webview(pending)
  }

  fn run_on_main_thread<F: FnOnce() + Send + 'static>(&self, f: F) -> Result<()> {
    send_user_message(&self.context, Message::Task(Box::new(f)))
  }

  #[cfg(all(desktop, feature = "system-tray"))]
  fn system_tray(
    &self,
    system_tray: SystemTray,
  ) -> Result<<Self::Runtime as Runtime<T>>::TrayHandler> {
    let id = system_tray.id;
    let (tx, rx) = channel();
    send_user_message(
      &self.context,
      Message::Tray(id, TrayMessage::Create(system_tray, tx)),
    )?;
    rx.recv().unwrap()?;
    Ok(SystemTrayHandle {
      context: self.context.clone(),
      id,
      proxy: self.context.proxy.clone(),
    })
  }

  fn raw_display_handle(&self) -> RawDisplayHandle {
    self.context.main_thread.window_target.raw_display_handle()
  }

  #[cfg(target_os = "macos")]
  fn show(&self) -> tauri_runtime::Result<()> {
    send_user_message(
      &self.context,
      Message::Application(ApplicationMessage::Show),
    )
  }

  #[cfg(target_os = "macos")]
  fn hide(&self) -> tauri_runtime::Result<()> {
    send_user_message(
      &self.context,
      Message::Application(ApplicationMessage::Hide),
    )
  }
}

impl<T: UserEvent> Wry<T> {
  fn init(event_loop: EventLoop<Message<T>>) -> Result<Self> {
    let main_thread_id = current_thread().id();
    let web_context = WebContextStore::default();

    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager = Arc::new(Mutex::new(WryShortcutManager::new(&event_loop)));

    #[cfg(feature = "clipboard")]
    let clipboard_manager = Arc::new(Mutex::new(Clipboard::new()));

    let windows = Arc::new(RefCell::new(HashMap::default()));
    let webview_id_map = WebviewIdStore::default();

    #[cfg(all(desktop, feature = "system-tray"))]
    let system_tray_manager = Default::default();

    let context = Context {
      webview_id_map,
      main_thread_id,
      proxy: event_loop.create_proxy(),
      main_thread: DispatcherMainThreadContext {
        window_target: event_loop.deref().clone(),
        web_context,
        #[cfg(all(desktop, feature = "global-shortcut"))]
        global_shortcut_manager,
        #[cfg(feature = "clipboard")]
        clipboard_manager,
        windows,
        #[cfg(all(desktop, feature = "system-tray"))]
        system_tray_manager,
      },
    };

    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager_handle = GlobalShortcutManagerHandle {
      context: context.clone(),
      shortcuts: Default::default(),
      listeners: Default::default(),
    };

    #[cfg(feature = "clipboard")]
    #[allow(clippy::redundant_clone)]
    let clipboard_manager_handle = ClipboardManagerWrapper {
      context: context.clone(),
    };

    Ok(Self {
      context,

      plugins: Default::default(),

      #[cfg(all(desktop, feature = "global-shortcut"))]
      global_shortcut_manager_handle,

      #[cfg(feature = "clipboard")]
      clipboard_manager_handle,

      event_loop,
    })
  }

  pub fn plugin<P: PluginBuilder<T> + 'static>(&mut self, plugin: P) {
    self
      .plugins
      .push(Box::new(plugin.build(self.context.clone())));
  }
}

impl<T: UserEvent> Runtime<T> for Wry<T> {
  type Dispatcher = WryDispatcher<T>;
  type Handle = WryHandle<T>;

  #[cfg(all(desktop, feature = "global-shortcut"))]
  type GlobalShortcutManager = GlobalShortcutManagerHandle<T>;

  #[cfg(feature = "clipboard")]
  type ClipboardManager = ClipboardManagerWrapper<T>;

  #[cfg(all(desktop, feature = "system-tray"))]
  type TrayHandler = SystemTrayHandle<T>;

  type EventLoopProxy = EventProxy<T>;

  fn new() -> Result<Self> {
    let event_loop = EventLoop::<Message<T>>::with_user_event();
    Self::init(event_loop)
  }

  #[cfg(any(windows, target_os = "linux"))]
  fn new_any_thread() -> Result<Self> {
    #[cfg(target_os = "linux")]
    use wry::application::platform::unix::EventLoopExtUnix;
    #[cfg(windows)]
    use wry::application::platform::windows::EventLoopExtWindows;
    let event_loop = EventLoop::<Message<T>>::new_any_thread();
    Self::init(event_loop)
  }

  fn create_proxy(&self) -> EventProxy<T> {
    EventProxy(self.event_loop.create_proxy())
  }

  fn handle(&self) -> Self::Handle {
    WryHandle {
      context: self.context.clone(),
    }
  }

  #[cfg(all(desktop, feature = "global-shortcut"))]
  fn global_shortcut_manager(&self) -> Self::GlobalShortcutManager {
    self.global_shortcut_manager_handle.clone()
  }

  #[cfg(feature = "clipboard")]
  fn clipboard_manager(&self) -> Self::ClipboardManager {
    self.clipboard_manager_handle.clone()
  }

  fn create_window(&self, pending: PendingWindow<T, Self>) -> Result<DetachedWindow<T, Self>> {
    let label = pending.label.clone();
    let menu_ids = pending.menu_ids.clone();
    let js_event_listeners = pending.js_event_listeners.clone();
    let window_id = rand::random();

    let webview = create_webview(
      window_id,
      &self.event_loop,
      &self.context.main_thread.web_context,
      self.context.clone(),
      pending,
    )?;

    let dispatcher = WryDispatcher {
      window_id,
      context: self.context.clone(),
    };

    self
      .context
      .main_thread
      .windows
      .borrow_mut()
      .insert(window_id, webview);

    Ok(DetachedWindow {
      label,
      dispatcher,
      menu_ids,
      js_event_listeners,
    })
  }

  #[cfg(all(desktop, feature = "system-tray"))]
  fn system_tray(&self, mut system_tray: SystemTray) -> Result<Self::TrayHandler> {
    let id = system_tray.id;
    let mut listeners = Vec::new();
    if let Some(l) = system_tray.on_event.take() {
      listeners.push(Arc::new(l));
    }
    let (tray, items) = create_tray(WryTrayId(id), system_tray, &self.event_loop)?;
    self
      .context
      .main_thread
      .system_tray_manager
      .trays
      .lock()
      .unwrap()
      .insert(
        id,
        TrayContext {
          tray: Arc::new(Mutex::new(Some(tray))),
          listeners: Arc::new(Mutex::new(listeners)),
          items: Arc::new(Mutex::new(items)),
        },
      );

    Ok(SystemTrayHandle {
      context: self.context.clone(),
      id,
      proxy: self.event_loop.create_proxy(),
    })
  }

  #[cfg(all(desktop, feature = "system-tray"))]
  fn on_system_tray_event<F: Fn(TrayId, &SystemTrayEvent) + Send + 'static>(&mut self, f: F) {
    self
      .context
      .main_thread
      .system_tray_manager
      .global_listeners
      .lock()
      .unwrap()
      .push(Arc::new(Box::new(f)));
  }

  #[cfg(target_os = "macos")]
  fn set_activation_policy(&mut self, activation_policy: ActivationPolicy) {
    self
      .event_loop
      .set_activation_policy(match activation_policy {
        ActivationPolicy::Regular => WryActivationPolicy::Regular,
        ActivationPolicy::Accessory => WryActivationPolicy::Accessory,
        ActivationPolicy::Prohibited => WryActivationPolicy::Prohibited,
        _ => unimplemented!(),
      });
  }

  #[cfg(target_os = "macos")]
  fn show(&self) {
    self.event_loop.show_application();
  }

  #[cfg(target_os = "macos")]
  fn hide(&self) {
    self.event_loop.hide_application();
  }

  #[cfg(desktop)]
  fn run_iteration<F: FnMut(RunEvent<T>) + 'static>(&mut self, mut callback: F) -> RunIteration {
    use wry::application::platform::run_return::EventLoopExtRunReturn;
    let windows = self.context.main_thread.windows.clone();
    let webview_id_map = self.context.webview_id_map.clone();
    let web_context = &self.context.main_thread.web_context;
    let plugins = &mut self.plugins;
    #[cfg(all(desktop, feature = "system-tray"))]
    let system_tray_manager = self.context.main_thread.system_tray_manager.clone();

    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager = self.context.main_thread.global_shortcut_manager.clone();
    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager_handle = self.global_shortcut_manager_handle.clone();

    #[cfg(feature = "clipboard")]
    let clipboard_manager = self.context.main_thread.clipboard_manager.clone();
    let mut iteration = RunIteration::default();

    let proxy = self.event_loop.create_proxy();

    self
      .event_loop
      .run_return(|event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::MainEventsCleared = &event {
          *control_flow = ControlFlow::Exit;
        }

        for p in plugins.iter_mut() {
          let prevent_default = p.on_event(
            &event,
            event_loop,
            &proxy,
            control_flow,
            EventLoopIterationContext {
              callback: &mut callback,
              webview_id_map: webview_id_map.clone(),
              windows: windows.clone(),
              #[cfg(all(desktop, feature = "global-shortcut"))]
              global_shortcut_manager: global_shortcut_manager.clone(),
              #[cfg(all(desktop, feature = "global-shortcut"))]
              global_shortcut_manager_handle: &global_shortcut_manager_handle,
              #[cfg(feature = "clipboard")]
              clipboard_manager: clipboard_manager.clone(),
              #[cfg(all(desktop, feature = "system-tray"))]
              system_tray_manager: system_tray_manager.clone(),
            },
            web_context,
          );
          if prevent_default {
            return;
          }
        }

        iteration = handle_event_loop(
          event,
          event_loop,
          control_flow,
          EventLoopIterationContext {
            callback: &mut callback,
            windows: windows.clone(),
            webview_id_map: webview_id_map.clone(),
            #[cfg(all(desktop, feature = "global-shortcut"))]
            global_shortcut_manager: global_shortcut_manager.clone(),
            #[cfg(all(desktop, feature = "global-shortcut"))]
            global_shortcut_manager_handle: &global_shortcut_manager_handle,
            #[cfg(feature = "clipboard")]
            clipboard_manager: clipboard_manager.clone(),
            #[cfg(all(desktop, feature = "system-tray"))]
            system_tray_manager: system_tray_manager.clone(),
          },
          web_context,
        );
      });

    iteration
  }

  fn run<F: FnMut(RunEvent<T>) + 'static>(self, mut callback: F) {
    let windows = self.context.main_thread.windows.clone();
    let webview_id_map = self.context.webview_id_map.clone();
    let web_context = self.context.main_thread.web_context;
    let mut plugins = self.plugins;

    #[cfg(all(desktop, feature = "system-tray"))]
    let system_tray_manager = self.context.main_thread.system_tray_manager;

    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager = self.context.main_thread.global_shortcut_manager.clone();
    #[cfg(all(desktop, feature = "global-shortcut"))]
    let global_shortcut_manager_handle = self.global_shortcut_manager_handle.clone();

    #[cfg(feature = "clipboard")]
    let clipboard_manager = self.context.main_thread.clipboard_manager.clone();

    let proxy = self.event_loop.create_proxy();

    self.event_loop.run(move |event, event_loop, control_flow| {
      for p in &mut plugins {
        let prevent_default = p.on_event(
          &event,
          event_loop,
          &proxy,
          control_flow,
          EventLoopIterationContext {
            callback: &mut callback,
            webview_id_map: webview_id_map.clone(),
            windows: windows.clone(),
            #[cfg(all(desktop, feature = "global-shortcut"))]
            global_shortcut_manager: global_shortcut_manager.clone(),
            #[cfg(all(desktop, feature = "global-shortcut"))]
            global_shortcut_manager_handle: &global_shortcut_manager_handle,
            #[cfg(feature = "clipboard")]
            clipboard_manager: clipboard_manager.clone(),
            #[cfg(all(desktop, feature = "system-tray"))]
            system_tray_manager: system_tray_manager.clone(),
          },
          &web_context,
        );
        if prevent_default {
          return;
        }
      }
      handle_event_loop(
        event,
        event_loop,
        control_flow,
        EventLoopIterationContext {
          callback: &mut callback,
          webview_id_map: webview_id_map.clone(),
          windows: windows.clone(),
          #[cfg(all(desktop, feature = "global-shortcut"))]
          global_shortcut_manager: global_shortcut_manager.clone(),
          #[cfg(all(desktop, feature = "global-shortcut"))]
          global_shortcut_manager_handle: &global_shortcut_manager_handle,
          #[cfg(feature = "clipboard")]
          clipboard_manager: clipboard_manager.clone(),
          #[cfg(all(desktop, feature = "system-tray"))]
          system_tray_manager: system_tray_manager.clone(),
        },
        &web_context,
      );
    })
  }
}

pub struct EventLoopIterationContext<'a, T: UserEvent> {
  pub callback: &'a mut (dyn FnMut(RunEvent<T>) + 'static),
  pub webview_id_map: WebviewIdStore,
  pub windows: Arc<RefCell<HashMap<WebviewId, WindowWrapper>>>,
  #[cfg(all(desktop, feature = "global-shortcut"))]
  pub global_shortcut_manager: Arc<Mutex<WryShortcutManager>>,
  #[cfg(all(desktop, feature = "global-shortcut"))]
  pub global_shortcut_manager_handle: &'a GlobalShortcutManagerHandle<T>,
  #[cfg(feature = "clipboard")]
  pub clipboard_manager: Arc<Mutex<Clipboard>>,
  #[cfg(all(desktop, feature = "system-tray"))]
  pub system_tray_manager: SystemTrayManager,
}

struct UserMessageContext {
  windows: Arc<RefCell<HashMap<WebviewId, WindowWrapper>>>,
  webview_id_map: WebviewIdStore,
  #[cfg(all(desktop, feature = "global-shortcut"))]
  global_shortcut_manager: Arc<Mutex<WryShortcutManager>>,
  #[cfg(feature = "clipboard")]
  clipboard_manager: Arc<Mutex<Clipboard>>,
  #[cfg(all(desktop, feature = "system-tray"))]
  system_tray_manager: SystemTrayManager,
}

fn handle_user_message<T: UserEvent>(
  event_loop: &EventLoopWindowTarget<Message<T>>,
  message: Message<T>,
  context: UserMessageContext,
  web_context: &WebContextStore,
) -> RunIteration {
  let UserMessageContext {
    webview_id_map,
    #[cfg(all(desktop, feature = "global-shortcut"))]
    global_shortcut_manager,
    #[cfg(feature = "clipboard")]
    clipboard_manager,
    windows,
    #[cfg(all(desktop, feature = "system-tray"))]
    system_tray_manager,
  } = context;
  match message {
    Message::Task(task) => task(),
    #[cfg(target_os = "macos")]
    Message::Application(application_message) => match application_message {
      ApplicationMessage::Show => {
        event_loop.show_application();
      }
      ApplicationMessage::Hide => {
        event_loop.hide_application();
      }
    },
    Message::Window(id, window_message) => {
      if let WindowMessage::UpdateMenuItem(item_id, update) = window_message {
        if let Some(menu_items) = windows.borrow_mut().get_mut(&id).map(|w| &mut w.menu_items) {
          if let Some(menu_items) = menu_items.as_mut() {
            let item = menu_items.get_mut(&item_id).expect("menu item not found");
            match update {
              MenuUpdate::SetEnabled(enabled) => item.set_enabled(enabled),
              MenuUpdate::SetTitle(title) => item.set_title(&title),
              MenuUpdate::SetSelected(selected) => item.set_selected(selected),
              #[cfg(target_os = "macos")]
              MenuUpdate::SetNativeImage(image) => {
                item.set_native_image(NativeImageWrapper::from(image).0)
              }
            }
          }
        }
      } else {
        let w = windows.borrow().get(&id).map(|w| {
          (
            w.inner.clone(),
            w.window_event_listeners.clone(),
            w.menu_event_listeners.clone(),
          )
        });
        if let Some((Some(window), window_event_listeners, menu_event_listeners)) = w {
          match window_message {
            #[cfg(desktop)]
            WindowMessage::WithWebview(f) => {
              if let WindowHandle::Webview { inner: w, .. } = &window {
                #[cfg(any(
                  target_os = "linux",
                  target_os = "dragonfly",
                  target_os = "freebsd",
                  target_os = "netbsd",
                  target_os = "openbsd"
                ))]
                {
                  use wry::webview::WebviewExtUnix;
                  f(w.webview());
                }
                #[cfg(target_os = "macos")]
                {
                  use wry::webview::WebviewExtMacOS;
                  f(Webview {
                    webview: w.webview(),
                    manager: w.manager(),
                    ns_window: w.ns_window(),
                  });
                }

                #[cfg(windows)]
                {
                  f(Webview {
                    controller: w.controller(),
                  });
                }
              }
            }

            WindowMessage::AddEventListener(id, listener) => {
              window_event_listeners.lock().unwrap().insert(id, listener);
            }

            WindowMessage::AddMenuEventListener(id, listener) => {
              menu_event_listeners.lock().unwrap().insert(id, listener);
            }

            #[cfg(any(debug_assertions, feature = "devtools"))]
            WindowMessage::OpenDevTools => {
              if let WindowHandle::Webview { inner: w, .. } = &window {
                w.open_devtools();
              }
            }
            #[cfg(any(debug_assertions, feature = "devtools"))]
            WindowMessage::CloseDevTools => {
              if let WindowHandle::Webview { inner: w, .. } = &window {
                w.close_devtools();
              }
            }
            #[cfg(any(debug_assertions, feature = "devtools"))]
            WindowMessage::IsDevToolsOpen(tx) => {
              if let WindowHandle::Webview { inner: w, .. } = &window {
                tx.send(w.is_devtools_open()).unwrap();
              } else {
                tx.send(false).unwrap();
              }
            }
            // Getters
            WindowMessage::ScaleFactor(tx) => tx.send(window.scale_factor()).unwrap(),
            WindowMessage::InnerPosition(tx) => tx
              .send(
                window
                  .inner_position()
                  .map(|p| PhysicalPositionWrapper(p).into())
                  .map_err(|_| Error::FailedToSendMessage),
              )
              .unwrap(),
            WindowMessage::OuterPosition(tx) => tx
              .send(
                window
                  .outer_position()
                  .map(|p| PhysicalPositionWrapper(p).into())
                  .map_err(|_| Error::FailedToSendMessage),
              )
              .unwrap(),
            WindowMessage::InnerSize(tx) => tx
              .send(PhysicalSizeWrapper(window.inner_size()).into())
              .unwrap(),
            WindowMessage::OuterSize(tx) => tx
              .send(PhysicalSizeWrapper(window.outer_size()).into())
              .unwrap(),
            WindowMessage::IsFullscreen(tx) => tx.send(window.fullscreen().is_some()).unwrap(),
            WindowMessage::IsMaximized(tx) => tx.send(window.is_maximized()).unwrap(),
            WindowMessage::IsDecorated(tx) => tx.send(window.is_decorated()).unwrap(),
            WindowMessage::IsResizable(tx) => tx.send(window.is_resizable()).unwrap(),
            WindowMessage::IsVisible(tx) => tx.send(window.is_visible()).unwrap(),
            WindowMessage::IsMenuVisible(tx) => tx.send(window.is_menu_visible()).unwrap(),
            WindowMessage::CurrentMonitor(tx) => tx.send(window.current_monitor()).unwrap(),
            WindowMessage::PrimaryMonitor(tx) => tx.send(window.primary_monitor()).unwrap(),
            WindowMessage::AvailableMonitors(tx) => {
              tx.send(window.available_monitors().collect()).unwrap()
            }
            #[cfg(any(
              target_os = "linux",
              target_os = "dragonfly",
              target_os = "freebsd",
              target_os = "netbsd",
              target_os = "openbsd"
            ))]
            WindowMessage::GtkWindow(tx) => {
              tx.send(GtkWindow(window.gtk_window().clone())).unwrap()
            }
            WindowMessage::RawWindowHandle(tx) => tx
              .send(RawWindowHandle(window.raw_window_handle()))
              .unwrap(),
            WindowMessage::Theme(tx) => {
              tx.send(map_theme(&window.theme())).unwrap();
            }
            // Setters
            WindowMessage::Center => {
              let _ = center_window(&window, window.inner_size());
            }
            WindowMessage::RequestUserAttention(request_type) => {
              window.request_user_attention(request_type.map(|r| r.0));
            }
            WindowMessage::SetResizable(resizable) => window.set_resizable(resizable),
            WindowMessage::SetTitle(title) => window.set_title(&title),
            WindowMessage::Maximize => window.set_maximized(true),
            WindowMessage::Unmaximize => window.set_maximized(false),
            WindowMessage::Minimize => window.set_minimized(true),
            WindowMessage::Unminimize => window.set_minimized(false),
            WindowMessage::ShowMenu => window.show_menu(),
            WindowMessage::HideMenu => window.hide_menu(),
            WindowMessage::Show => window.set_visible(true),
            WindowMessage::Hide => window.set_visible(false),
            WindowMessage::Close => {
              panic!("cannot handle `WindowMessage::Close` on the main thread")
            }
            WindowMessage::SetDecorations(decorations) => window.set_decorations(decorations),
            WindowMessage::SetAlwaysOnTop(always_on_top) => window.set_always_on_top(always_on_top),
            WindowMessage::SetSize(size) => {
              window.set_inner_size(SizeWrapper::from(size).0);
            }
            WindowMessage::SetMinSize(size) => {
              window.set_min_inner_size(size.map(|s| SizeWrapper::from(s).0));
            }
            WindowMessage::SetMaxSize(size) => {
              window.set_max_inner_size(size.map(|s| SizeWrapper::from(s).0));
            }
            WindowMessage::SetPosition(position) => {
              window.set_outer_position(PositionWrapper::from(position).0)
            }
            WindowMessage::SetFullscreen(fullscreen) => {
              if fullscreen {
                window.set_fullscreen(Some(Fullscreen::Borderless(None)))
              } else {
                window.set_fullscreen(None)
              }
            }
            WindowMessage::SetFocus => {
              window.set_focus();
            }
            WindowMessage::SetIcon(icon) => {
              window.set_window_icon(Some(icon));
            }
            #[allow(unused_variables)]
            WindowMessage::SetSkipTaskbar(skip) => {
              #[cfg(any(windows, target_os = "linux"))]
              window.set_skip_taskbar(skip);
            }
            WindowMessage::SetCursorGrab(grab) => {
              let _ = window.set_cursor_grab(grab);
            }
            WindowMessage::SetCursorVisible(visible) => {
              window.set_cursor_visible(visible);
            }
            WindowMessage::SetCursorIcon(icon) => {
              window.set_cursor_icon(CursorIconWrapper::from(icon).0);
            }
            WindowMessage::SetCursorPosition(position) => {
              let _ = window.set_cursor_position(PositionWrapper::from(position).0);
            }
            WindowMessage::SetIgnoreCursorEvents(ignore) => {
              let _ = window.set_ignore_cursor_events(ignore);
            }
            WindowMessage::DragWindow => {
              let _ = window.drag_window();
            }
            WindowMessage::UpdateMenuItem(_id, _update) => {
              // already handled
            }
            WindowMessage::RequestRedraw => {
              window.request_redraw();
            }
          }
        }
      }
    }
    Message::Webview(id, webview_message) => match webview_message {
      WebviewMessage::EvaluateScript(script) => {
        if let Some(WindowHandle::Webview { inner: webview, .. }) =
          windows.borrow().get(&id).and_then(|w| w.inner.as_ref())
        {
          if let Err(e) = webview.evaluate_script(&script) {
            debug_eprintln!("{}", e);
          }
        }
      }
      WebviewMessage::Print => {
        if let Some(WindowHandle::Webview { inner: webview, .. }) =
          windows.borrow().get(&id).and_then(|w| w.inner.as_ref())
        {
          let _ = webview.print();
        }
      }
      WebviewMessage::WebviewEvent(event) => {
        let window_event_listeners = windows
          .borrow()
          .get(&id)
          .map(|w| w.window_event_listeners.clone());
        if let Some(window_event_listeners) = window_event_listeners {
          if let Some(event) = WindowEventWrapper::from(&event).0 {
            let listeners = window_event_listeners.lock().unwrap();
            let handlers = listeners.values();
            for handler in handlers {
              handler(&event);
            }
          }
        }
      }
    },
    Message::CreateWebview(window_id, handler) => match handler(event_loop, web_context) {
      Ok(webview) => {
        windows.borrow_mut().insert(window_id, webview);
      }
      Err(e) => {
        debug_eprintln!("{}", e);
      }
    },
    Message::CreateWindow(window_id, handler, sender) => {
      let (label, builder) = handler();
      if let Ok(window) = builder.build(event_loop) {
        webview_id_map.insert(window.id(), window_id);

        let w = Arc::new(window);

        windows.borrow_mut().insert(
          window_id,
          WindowWrapper {
            label,
            inner: Some(WindowHandle::Window(w.clone())),
            menu_items: Default::default(),
            window_event_listeners: Default::default(),
            menu_event_listeners: Default::default(),
          },
        );
        sender.send(Ok(Arc::downgrade(&w))).unwrap();
      } else {
        sender.send(Err(Error::CreateWindow)).unwrap();
      }
    }

    #[cfg(all(desktop, feature = "system-tray"))]
    Message::Tray(tray_id, tray_message) => {
      let mut trays = system_tray_manager.trays.lock().unwrap();

      if let TrayMessage::Create(tray, tx) = tray_message {
        match create_tray(WryTrayId(tray_id), tray, event_loop) {
          Ok((tray, items)) => {
            trays.insert(
              tray_id,
              TrayContext {
                tray: Arc::new(Mutex::new(Some(tray))),
                listeners: Default::default(),
                items: Arc::new(Mutex::new(items)),
              },
            );

            tx.send(Ok(())).unwrap();
          }

          Err(e) => {
            tx.send(Err(e)).unwrap();
          }
        }
      } else if let Some(tray_context) = trays.get(&tray_id) {
        match tray_message {
          TrayMessage::UpdateItem(menu_id, update) => {
            let mut tray = tray_context.items.as_ref().lock().unwrap();
            let item = tray.get_mut(&menu_id).expect("menu item not found");
            match update {
              MenuUpdate::SetEnabled(enabled) => item.set_enabled(enabled),
              MenuUpdate::SetTitle(title) => item.set_title(&title),
              MenuUpdate::SetSelected(selected) => item.set_selected(selected),
              #[cfg(target_os = "macos")]
              MenuUpdate::SetNativeImage(image) => {
                item.set_native_image(NativeImageWrapper::from(image).0)
              }
            }
          }
          TrayMessage::UpdateMenu(menu) => {
            if let Some(tray) = &mut *tray_context.tray.lock().unwrap() {
              let mut items = HashMap::new();
              tray.set_menu(&to_wry_context_menu(&mut items, menu));
              *tray_context.items.lock().unwrap() = items;
            }
          }
          TrayMessage::UpdateIcon(icon) => {
            if let Some(tray) = &mut *tray_context.tray.lock().unwrap() {
              if let Ok(icon) = TrayIcon::try_from(icon) {
                tray.set_icon(icon.0);
              }
            }
          }
          #[cfg(target_os = "macos")]
          TrayMessage::UpdateIconAsTemplate(is_template) => {
            if let Some(tray) = &mut *tray_context.tray.lock().unwrap() {
              tray.set_icon_as_template(is_template);
            }
          }
          #[cfg(target_os = "macos")]
          TrayMessage::UpdateTitle(title) => {
            if let Some(tray) = &mut *tray_context.tray.lock().unwrap() {
              tray.set_title(&title);
            }
          }
          TrayMessage::Create(_tray, _tx) => {
            // already handled
          }
          TrayMessage::Destroy(tx) => {
            *tray_context.tray.lock().unwrap() = None;
            tray_context.listeners.lock().unwrap().clear();
            tray_context.items.lock().unwrap().clear();
            tx.send(Ok(())).unwrap();
          }
        }
      }
    }
    #[cfg(all(desktop, feature = "global-shortcut"))]
    Message::GlobalShortcut(message) => {
      handle_global_shortcut_message(message, &global_shortcut_manager)
    }
    #[cfg(feature = "clipboard")]
    Message::Clipboard(message) => handle_clipboard_message(message, &clipboard_manager),
    Message::UserEvent(_) => (),
  }

  let it = RunIteration {
    window_count: windows.borrow().len(),
  };
  it
}

fn handle_event_loop<T: UserEvent>(
  event: Event<'_, Message<T>>,
  event_loop: &EventLoopWindowTarget<Message<T>>,
  control_flow: &mut ControlFlow,
  context: EventLoopIterationContext<'_, T>,
  web_context: &WebContextStore,
) -> RunIteration {
  let EventLoopIterationContext {
    callback,
    webview_id_map,
    windows,
    #[cfg(all(desktop, feature = "global-shortcut"))]
    global_shortcut_manager,
    #[cfg(all(desktop, feature = "global-shortcut"))]
    global_shortcut_manager_handle,
    #[cfg(feature = "clipboard")]
    clipboard_manager,
    #[cfg(all(desktop, feature = "system-tray"))]
    system_tray_manager,
  } = context;
  if *control_flow != ControlFlow::Exit {
    *control_flow = ControlFlow::Wait;
  }

  match event {
    Event::NewEvents(StartCause::Init) => {
      callback(RunEvent::Ready);
    }

    Event::NewEvents(StartCause::Poll) => {
      callback(RunEvent::Resumed);
    }

    Event::MainEventsCleared => {
      callback(RunEvent::MainEventsCleared);
    }

    Event::LoopDestroyed => {
      callback(RunEvent::Exit);
    }

    #[cfg(all(desktop, feature = "global-shortcut"))]
    Event::GlobalShortcutEvent(accelerator_id) => {
      for (id, handler) in &*global_shortcut_manager_handle.listeners.lock().unwrap() {
        if accelerator_id == *id {
          handler();
        }
      }
    }
    Event::MenuEvent {
      window_id,
      menu_id,
      origin: MenuType::MenuBar,
      ..
    } => {
      #[allow(unused_mut)]
      let mut window_id = window_id.unwrap(); // always Some on MenuBar event

      #[cfg(target_os = "macos")]
      {
        // safety: we're only checking to see if the window_id is 0
        // which is the value sent by macOS when the window is minimized (NSApplication::sharedApplication::mainWindow is null)
        if window_id == unsafe { WindowId::dummy() } {
          window_id = *webview_id_map.0.lock().unwrap().keys().next().unwrap();
        }
      }

      let event = MenuEvent {
        menu_item_id: menu_id.0,
      };
      let window_menu_event_listeners = {
        // on macOS the window id might be the inspector window if it is detached
        let window_id = if let Some(window_id) = webview_id_map.get(&window_id) {
          window_id
        } else {
          *webview_id_map.0.lock().unwrap().values().next().unwrap()
        };
        windows
          .borrow()
          .get(&window_id)
          .unwrap()
          .menu_event_listeners
          .clone()
      };
      let listeners = window_menu_event_listeners.lock().unwrap();
      let handlers = listeners.values();
      for handler in handlers {
        handler(&event);
      }
    }
    #[cfg(all(desktop, feature = "system-tray"))]
    Event::MenuEvent {
      window_id: _,
      menu_id,
      origin: MenuType::ContextMenu,
      ..
    } => {
      let event = SystemTrayEvent::MenuItemClick(menu_id.0);

      let trays = system_tray_manager.trays.lock().unwrap();
      let trays_iter = trays.iter();

      let (mut listeners, mut tray_id) = (None, 0);
      for (id, tray_context) in trays_iter {
        let has_menu = {
          let items = tray_context.items.lock().unwrap();
          items.contains_key(&menu_id.0)
        };
        if has_menu {
          listeners.replace(tray_context.listeners.lock().unwrap().clone());
          tray_id = *id;
          break;
        }
      }
      drop(trays);
      if let Some(listeners) = listeners {
        let handlers = listeners.iter();
        for handler in handlers {
          handler(&event);
        }

        let global_listeners = system_tray_manager.global_listeners.lock().unwrap();
        let global_listeners_iter = global_listeners.iter();
        for global_listener in global_listeners_iter {
          global_listener(tray_id, &event);
        }
      }
    }
    #[cfg(all(desktop, feature = "system-tray"))]
    Event::TrayEvent {
      id,
      bounds,
      event,
      position: _cursor_position,
      ..
    } => {
      let (position, size) = (
        PhysicalPositionWrapper(bounds.position).into(),
        PhysicalSizeWrapper(bounds.size).into(),
      );
      let event = match event {
        TrayEvent::RightClick => SystemTrayEvent::RightClick { position, size },
        TrayEvent::DoubleClick => SystemTrayEvent::DoubleClick { position, size },
        // default to left click
        _ => SystemTrayEvent::LeftClick { position, size },
      };
      let trays = system_tray_manager.trays.lock().unwrap();
      if let Some(tray_context) = trays.get(&id.0) {
        let listeners = tray_context.listeners.lock().unwrap();
        let iter = listeners.iter();
        for handler in iter {
          handler(&event);
        }
      }

      let global_listeners = system_tray_manager.global_listeners.lock().unwrap();
      let global_listeners_iter = global_listeners.iter();
      for global_listener in global_listeners_iter {
        global_listener(id.0, &event);
      }
    }
    Event::WindowEvent {
      event, window_id, ..
    } => {
      if let Some(window_id) = webview_id_map.get(&window_id) {
        {
          let windows_ref = windows.borrow();
          if let Some(window) = windows_ref.get(&window_id) {
            if let Some(event) = WindowEventWrapper::parse(&window.inner, &event).0 {
              let label = window.label.clone();
              let window_event_listeners = window.window_event_listeners.clone();

              drop(windows_ref);

              callback(RunEvent::WindowEvent {
                label,
                event: event.clone(),
              });
              let listeners = window_event_listeners.lock().unwrap();
              let handlers = listeners.values();
              for handler in handlers {
                handler(&event);
              }
            }
          }
        }

        match event {
          WryWindowEvent::CloseRequested => {
            on_close_requested(callback, window_id, windows.clone());
          }
          WryWindowEvent::Destroyed => {
            let removed = windows.borrow_mut().remove(&window_id).is_some();
            if removed {
              let is_empty = windows.borrow().is_empty();
              if is_empty {
                let (tx, rx) = channel();
                callback(RunEvent::ExitRequested { tx });

                let recv = rx.try_recv();
                let should_prevent = matches!(recv, Ok(ExitRequestedEventAction::Prevent));

                if !should_prevent {
                  *control_flow = ControlFlow::Exit;
                }
              }
            }
          }
          _ => {}
        }
      }
    }
    Event::UserEvent(message) => match message {
      Message::Window(id, WindowMessage::Close) => {
        on_window_close(id, windows.clone());
      }
      Message::UserEvent(t) => callback(RunEvent::UserEvent(t)),
      message => {
        return handle_user_message(
          event_loop,
          message,
          UserMessageContext {
            webview_id_map,
            #[cfg(all(desktop, feature = "global-shortcut"))]
            global_shortcut_manager,
            #[cfg(feature = "clipboard")]
            clipboard_manager,
            windows,
            #[cfg(all(desktop, feature = "system-tray"))]
            system_tray_manager,
          },
          web_context,
        );
      }
    },
    _ => (),
  }

  let it = RunIteration {
    window_count: windows.borrow().len(),
  };
  it
}

fn on_close_requested<'a, T: UserEvent>(
  callback: &'a mut (dyn FnMut(RunEvent<T>) + 'static),
  window_id: WebviewId,
  windows: Arc<RefCell<HashMap<WebviewId, WindowWrapper>>>,
) {
  let (tx, rx) = channel();
  let windows_ref = windows.borrow();
  if let Some(w) = windows_ref.get(&window_id) {
    let label = w.label.clone();
    let window_event_listeners = w.window_event_listeners.clone();

    drop(windows_ref);

    let listeners = window_event_listeners.lock().unwrap();
    let handlers = listeners.values();
    for handler in handlers {
      handler(&WindowEvent::CloseRequested {
        signal_tx: tx.clone(),
      });
    }
    callback(RunEvent::WindowEvent {
      label,
      event: WindowEvent::CloseRequested { signal_tx: tx },
    });
    if let Ok(true) = rx.try_recv() {
    } else {
      on_window_close(window_id, windows);
    }
  }
}

fn on_window_close(window_id: WebviewId, windows: Arc<RefCell<HashMap<WebviewId, WindowWrapper>>>) {
  if let Some(mut window_wrapper) = windows.borrow_mut().get_mut(&window_id) {
    window_wrapper.inner = None;
  }
}

pub fn center_window(window: &Window, window_size: WryPhysicalSize<u32>) -> Result<()> {
  if let Some(monitor) = window.current_monitor() {
    let screen_size = monitor.size();
    let monitor_pos = monitor.position();
    let x = (screen_size.width as i32 - window_size.width as i32) / 2;
    let y = (screen_size.height as i32 - window_size.height as i32) / 2;
    window.set_outer_position(WryPhysicalPosition::new(
      monitor_pos.x + x,
      monitor_pos.y + y,
    ));
    Ok(())
  } else {
    Err(Error::FailedToGetMonitor)
  }
}

fn to_wry_menu(
  custom_menu_items: &mut HashMap<MenuHash, WryCustomMenuItem>,
  menu: Menu,
) -> MenuBar {
  let mut wry_menu = MenuBar::new();
  for item in menu.items {
    match item {
      MenuEntry::CustomItem(c) => {
        let mut attributes = MenuItemAttributesWrapper::from(&c).0;
        attributes = attributes.with_id(WryMenuId(c.id));
        #[allow(unused_mut)]
        let mut item = wry_menu.add_item(attributes);
        #[cfg(target_os = "macos")]
        if let Some(native_image) = c.native_image {
          item.set_native_image(NativeImageWrapper::from(native_image).0);
        }
        custom_menu_items.insert(c.id, item);
      }
      MenuEntry::NativeItem(i) => {
        wry_menu.add_native_item(MenuItemWrapper::from(i).0);
      }
      MenuEntry::Submenu(submenu) => {
        wry_menu.add_submenu(
          &submenu.title,
          submenu.enabled,
          to_wry_menu(custom_menu_items, submenu.inner),
        );
      }
    }
  }
  wry_menu
}

fn create_webview<T: UserEvent>(
  window_id: WebviewId,
  event_loop: &EventLoopWindowTarget<Message<T>>,
  web_context_store: &WebContextStore,
  context: Context<T>,
  pending: PendingWindow<T, Wry<T>>,
) -> Result<WindowWrapper> {
  #[allow(unused_mut)]
  let PendingWindow {
    webview_attributes,
    uri_scheme_protocols,
    mut window_builder,
    ipc_handler,
    label,
    url,
    menu_ids,
    js_event_listeners,
    ..
  } = pending;
  let webview_id_map = context.webview_id_map.clone();
  #[cfg(windows)]
  let proxy = context.proxy.clone();

  let window_event_listeners = WindowEventListeners::default();

  #[cfg(windows)]
  {
    window_builder.inner = window_builder
      .inner
      .with_drag_and_drop(webview_attributes.file_drop_handler_enabled);
  }

  #[cfg(target_os = "macos")]
  {
    if window_builder.tabbing_identifier.is_none()
      || window_builder.inner.window.transparent
      || !window_builder.inner.window.decorations
    {
      window_builder.inner = window_builder.inner.with_automatic_window_tabbing(false);
    }
  }

  let is_window_transparent = window_builder.inner.window.transparent;
  let menu_items = if let Some(menu) = window_builder.menu {
    let mut menu_items = HashMap::new();
    let menu = to_wry_menu(&mut menu_items, menu);
    window_builder.inner = window_builder.inner.with_menu(menu);
    Some(menu_items)
  } else {
    None
  };
  let window = window_builder.inner.build(event_loop).unwrap();

  webview_id_map.insert(window.id(), window_id);

  if window_builder.center {
    let _ = center_window(&window, window.inner_size());
  }
  let mut webview_builder = WebViewBuilder::new(window)
    .map_err(|e| Error::CreateWebview(Box::new(e)))?
    .with_url(&url)
    .unwrap() // safe to unwrap because we validate the URL beforehand
    .with_transparent(is_window_transparent)
    .with_accept_first_mouse(webview_attributes.accept_first_mouse);
  if webview_attributes.file_drop_handler_enabled {
    webview_builder = webview_builder
      .with_file_drop_handler(create_file_drop_handler(window_event_listeners.clone()));
  }
  if let Some(user_agent) = webview_attributes.user_agent {
    webview_builder = webview_builder.with_user_agent(&user_agent);
  }
  if let Some(handler) = ipc_handler {
    webview_builder = webview_builder.with_ipc_handler(create_ipc_handler(
      context,
      label.clone(),
      menu_ids,
      js_event_listeners,
      handler,
    ));
  }
  for (scheme, protocol) in uri_scheme_protocols {
    webview_builder = webview_builder.with_custom_protocol(scheme, move |wry_request| {
      protocol(&HttpRequestWrapper::from(wry_request).0)
        .map(|tauri_response| HttpResponseWrapper::from(tauri_response).0)
        .map_err(|_| wry::Error::InitScriptError)
    });
  }

  for script in webview_attributes.initialization_scripts {
    webview_builder = webview_builder.with_initialization_script(&script);
  }

  let mut web_context = web_context_store.lock().expect("poisoned WebContext store");
  let is_first_context = web_context.is_empty();
  let automation_enabled = std::env::var("TAURI_AUTOMATION").as_deref() == Ok("true");
  let web_context_key = // force a unique WebContext when automation is false;
    // the context must be stored on the HashMap because it must outlive the WebView on macOS
    if automation_enabled {
      webview_attributes.data_directory.clone()
    } else {
      // random unique key
      Some(Uuid::new_v4().as_hyphenated().to_string().into())
    };
  let entry = web_context.entry(web_context_key.clone());
  let web_context = match entry {
    Occupied(occupied) => occupied.into_mut(),
    Vacant(vacant) => {
      let mut web_context = WebContext::new(webview_attributes.data_directory);
      web_context.set_allows_automation(if automation_enabled {
        is_first_context
      } else {
        false
      });
      vacant.insert(web_context)
    }
  };

  if webview_attributes.clipboard {
    webview_builder.webview.clipboard = true;
  }

  #[cfg(any(debug_assertions, feature = "devtools"))]
  {
    webview_builder = webview_builder.with_devtools(true);
  }

  let webview = webview_builder
    .with_web_context(web_context)
    .build()
    .map_err(|e| Error::CreateWebview(Box::new(e)))?;

  #[cfg(windows)]
  {
    let controller = webview.controller();
    let proxy_ = proxy.clone();
    let mut token = EventRegistrationToken::default();
    unsafe {
      controller.add_GotFocus(
        &FocusChangedEventHandler::create(Box::new(move |_, _| {
          let _ = proxy_.send_event(Message::Webview(
            window_id,
            WebviewMessage::WebviewEvent(WebviewEvent::Focused(true)),
          ));
          Ok(())
        })),
        &mut token,
      )
    }
    .unwrap();
    unsafe {
      controller.add_LostFocus(
        &FocusChangedEventHandler::create(Box::new(move |_, _| {
          let _ = proxy.send_event(Message::Webview(
            window_id,
            WebviewMessage::WebviewEvent(WebviewEvent::Focused(false)),
          ));
          Ok(())
        })),
        &mut token,
      )
    }
    .unwrap();
  }

  Ok(WindowWrapper {
    label,
    inner: Some(WindowHandle::Webview {
      inner: Arc::new(webview),
      context_store: web_context_store.clone(),
      context_key: if automation_enabled {
        None
      } else {
        web_context_key
      },
    }),
    menu_items,
    window_event_listeners,
    menu_event_listeners: Default::default(),
  })
}

/// Create a wry ipc handler from a tauri ipc handler.
fn create_ipc_handler<T: UserEvent>(
  context: Context<T>,
  label: String,
  menu_ids: Arc<Mutex<HashMap<MenuHash, MenuId>>>,
  js_event_listeners: Arc<Mutex<HashMap<JsEventListenerKey, HashSet<u64>>>>,
  handler: WebviewIpcHandler<T, Wry<T>>,
) -> Box<IpcHandler> {
  Box::new(move |window, request| {
    let window_id = context.webview_id_map.get(&window.id()).unwrap();
    handler(
      DetachedWindow {
        dispatcher: WryDispatcher {
          window_id,
          context: context.clone(),
        },
        label: label.clone(),
        menu_ids: menu_ids.clone(),
        js_event_listeners: js_event_listeners.clone(),
      },
      request,
    );
  })
}

/// Create a wry file drop handler.
fn create_file_drop_handler(window_event_listeners: WindowEventListeners) -> Box<FileDropHandler> {
  Box::new(move |_window, event| {
    let event: FileDropEvent = FileDropEventWrapper(event).into();
    let window_event = WindowEvent::FileDrop(event);
    let listeners_map = window_event_listeners.lock().unwrap();
    let has_listener = !listeners_map.is_empty();
    let handlers = listeners_map.values();
    for listener in handlers {
      listener(&window_event);
    }
    // block the default OS action on file drop if we had a listener
    has_listener
  })
}
