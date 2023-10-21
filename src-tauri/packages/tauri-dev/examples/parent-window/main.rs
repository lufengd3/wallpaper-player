// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{command, window::WindowBuilder, Window, WindowUrl};

#[command]
async fn create_child_window(id: String, window: Window) {
  let child = WindowBuilder::new(&window, id, WindowUrl::default())
    .title("Child")
    .inner_size(400.0, 300.0);

  #[cfg(target_os = "macos")]
  let child = child.parent_window(window.ns_window().unwrap());
  #[cfg(windows)]
  let child = child.parent_window(window.hwnd().unwrap());

  child.build().unwrap();
}

fn main() {
  tauri::Builder::default()
    .on_page_load(|window, _payload| {
      let label = window.label().to_string();
      window.listen("clicked".to_string(), move |_payload| {
        println!("got 'clicked' event on window '{}'", label);
      });
    })
    .invoke_handler(tauri::generate_handler![create_child_window])
    .setup(|app| {
      WindowBuilder::new(app, "main".to_string(), WindowUrl::default())
        .title("Main")
        .inner_size(600.0, 400.0)
        .build()?;
      Ok(())
    })
    .run(tauri::generate_context!(
      "../../examples/parent-window/tauri.conf.json"
    ))
    .expect("failed to run tauri application");
}
