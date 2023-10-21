// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
  use std::{
    fs::{canonicalize, read},
    path::PathBuf,
  };

  use wry::{
    application::{
      accelerator::Accelerator,
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      keyboard::{KeyCode, ModifiersState},
      menu::{MenuBar, MenuItemAttributes},
      window::WindowBuilder,
    },
    http::{header::CONTENT_TYPE, Response},
    webview::WebViewBuilder,
  };

  let mut menu = MenuBar::new();
  let mut file_menu = MenuBar::new();
  file_menu.add_item(
    MenuItemAttributes::new("Quit").with_accelerators(&Accelerator::new(
      Some(ModifiersState::SUPER),
      // Some(ModifiersState::SHIFT),
      // None,
      KeyCode::KeyQ,
    )),
  );
  menu.add_submenu("File", true, file_menu);

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("Custom Protocol")
    .with_menu(menu)
    .build(&event_loop)
    .unwrap();

  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_custom_protocol("wry".into(), move |request| {
      let path = &request.uri().path();
      // Read the file content from file path
      let content = read(canonicalize(PathBuf::from("examples").join(
        if path == &"/" {
          "custom_protocol_page1.html"
        } else {
          // remove leading slash
          &path[1..]
        },
      ))?)?;

      // Return asset contents and mime types based on file extentions
      // If you don't want to do this manually, there are some crates for you.
      // Such as `infer` and `mime_guess`.
      let (data, meta) = if path.ends_with(".html") || path == &"/" {
        (content, "text/html")
      } else if path.ends_with(".js") {
        (content, "text/javascript")
      } else if path.ends_with(".png") {
        (content, "image/png")
      } else {
        unimplemented!();
      };

      Response::builder()
        .header(CONTENT_TYPE, meta)
        .body(data)
        .map_err(Into::into)
    })
    // tell the webview to load the custom protocol
    .with_url("wry://localhost")?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::MenuEvent { menu_id, .. } => {
        println!("Menu clicked! {:?}", menu_id);
        *control_flow = ControlFlow::Exit;
      }
      _ => (),
    }
  });
}
