// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::WindowBuilder,
    },
    webview::WebViewBuilder,
  };

  enum UserEvent {
    Navigation(String),
  }

  let event_loop: EventLoop<UserEvent> = EventLoop::with_user_event();
  let proxy = event_loop.create_proxy();
  let window = WindowBuilder::new()
    .with_title("Hello World")
    .build(&event_loop)?;
  let _webview = WebViewBuilder::new(window)?
    .with_url("http://neverssl.com")?
    .with_navigation_handler(move |uri: String| {
      let submitted = proxy.send_event(UserEvent::Navigation(uri.clone())).is_ok();

      submitted && uri.contains("neverssl")
    })
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::UserEvent(UserEvent::Navigation(uri)) => {
        println!("{}", uri);
      }
      _ => (),
    }
  });
}
