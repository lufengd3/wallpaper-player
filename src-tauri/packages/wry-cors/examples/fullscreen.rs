// Copyright 2020-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

fn main() -> wry::Result<()> {
  use wry::{
    application::{
      event::{Event, StartCause, WindowEvent},
      event_loop::{ControlFlow, EventLoop},
      window::{Fullscreen, WindowBuilder},
    },
    webview::WebViewBuilder,
  };

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new()
    .with_title("3D Render Test")
    .with_fullscreen(Some(Fullscreen::Borderless(None)))
    .build(&event_loop)
    .unwrap();
  let _webview = WebViewBuilder::new(window)
    .unwrap()
    .with_url("https://browserbench.org/MotionMark1.2/")?
    .build()?;

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      _ => {}
    }
  });
}

// Test Result:
// CPU: i7 9750H || GPU: Intel(R) UHD Graphics 630
// Linux kernel 5.8.18-18-ibryza-standard-xin
// Mesa Mesa 20.2.6
// ================================================
// Canvas score - Test 1: 542 - Test 2: 368
// WebGL score - Test 1: 1390 - Test 2: 1342
// Total score: 3642
