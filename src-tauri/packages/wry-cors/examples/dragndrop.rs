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

  const HTML: &str = r#"data:text/html,
Drop files onto the window and read the console!<br>
Dropping files onto the following form is also possible:<br><br>
<input type="file"/>
"#;

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_url(HTML)?
    .with_file_drop_handler(|_, data| {
      println!("Window 1: {:?}", data);
      false // Returning true will block the OS default behaviour.
    })
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry application started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  });
}
