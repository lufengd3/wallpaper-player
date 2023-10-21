// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// Application code for a splashscreen system that waits on a Rust initialization script
mod rust {
  use std::{thread::sleep, time::Duration};
  use tauri::Manager;

  // this command is here just so the example doesn't throw an error
  #[tauri::command]
  fn close_splashscreen() {}

  pub fn main() {
    tauri::Builder::default()
      .setup(|app| {
        let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_window("main").unwrap();
        // we perform the initialization code on a new task so the app doesn't crash
        tauri::async_runtime::spawn(async move {
          println!("Initializing...");
          sleep(Duration::from_secs(2));
          println!("Done initializing.");

          // After it's done, close the splashscreen and display the main window
          splashscreen_window.close().unwrap();
          main_window.show().unwrap();
        });
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![close_splashscreen])
      .run(super::context())
      .expect("failed to run app");
  }
}

// Application code for a splashscreen system that waits for the UI
mod ui {
  use std::sync::{Arc, Mutex};
  use tauri::{Manager, State, Window};

  // wrappers around each Window
  // we use a dedicated type because Tauri can only manage a single instance of a given type
  struct SplashscreenWindow(Arc<Mutex<Window>>);
  struct MainWindow(Arc<Mutex<Window>>);

  #[tauri::command]
  fn close_splashscreen(
    _: Window, // force inference of P
    splashscreen: State<SplashscreenWindow>,
    main: State<MainWindow>,
  ) {
    // Close splashscreen
    splashscreen.0.lock().unwrap().close().unwrap();
    // Show main window
    main.0.lock().unwrap().show().unwrap();
  }

  pub fn main() {
    let context = super::context();
    tauri::Builder::default()
      .menu(if cfg!(target_os = "macos") {
        tauri::Menu::os_default(&context.package_info().name)
      } else {
        tauri::Menu::default()
      })
      .setup(|app| {
        // set the splashscreen and main windows to be globally available with the tauri state API
        app.manage(SplashscreenWindow(Arc::new(Mutex::new(
          app.get_window("splashscreen").unwrap(),
        ))));
        app.manage(MainWindow(Arc::new(Mutex::new(
          app.get_window("main").unwrap(),
        ))));
        Ok(())
      })
      .invoke_handler(tauri::generate_handler![close_splashscreen])
      .run(context)
      .expect("error while running tauri application");
  }
}

fn context() -> tauri::Context<tauri::utils::assets::EmbeddedAssets> {
  tauri::generate_context!("../../examples/splashscreen/tauri.conf.json")
}

fn main() {
  // toggle this flag to experiment with different splashscreen usages
  let ui = false;
  if ui {
    ui::main();
  } else {
    rust::main();
  }
}
