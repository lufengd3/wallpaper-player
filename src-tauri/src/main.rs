#![windows_subsystem = "windows"]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::{error, string, sync::Mutex};

use tauri::Manager;
use tauri::SystemTray;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent};
use window_shadows::set_shadow;
use tauri_plugin_autostart::MacosLauncher;

mod commands;
mod wp;
mod utils;

pub struct AppState {
  pub my_app_data_dir: PathBuf,
  pub config_file: PathBuf,
  pub win: Option<tauri::Window>,
}

fn main() {
  let ctx = tauri::generate_context!();
  let app_data_dir: PathBuf= match tauri::api::path::app_data_dir(&ctx.config()) {
    Some(res) => res,
    None => env::temp_dir()
  };
  let mut state = AppState {
    my_app_data_dir: app_data_dir.clone(),
    config_file:  app_data_dir.join("appconf"),
    win: None
  };
  fs::create_dir_all(app_data_dir).unwrap();

  let system_tray = config_system_tray();
  tauri::Builder::default()
    .setup(move |app| {
      let window = app.get_window("main").unwrap();
      // autostart hide window
      let startup_args: Vec<String> = env::args().collect();
      if startup_args.contains(&"--autostart".to_owned()) {
        window.hide().unwrap();
      }
      // window border shadow
      set_shadow(&window, true).expect("Unsupported platform!");
      // copy to state for commands
      state.win = Some(window.clone());
      app.manage(state);
      Ok(())
    })
    .system_tray(system_tray)
    .on_system_tray_event(|app, event| match event {
      // SystemTrayEvent::LeftClick {
      //   position: _,
      //   size: _,
      //   ..
      // } => {
      //   println!("system tray received a left click");
      // }
      // SystemTrayEvent::RightClick {
      //   position: _,
      //   size: _,
      //   ..
      // } => {
      //   println!("system tray received a right click");
      // }
      SystemTrayEvent::DoubleClick {
        position: _,
        size: _,
        ..
      } => {
        println!("system tray received a double click");
        let window = app.get_window("main").unwrap();
        window.show().unwrap();
      }
      SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
          "next" => {
            let window = app.get_window("main").unwrap();
            window.emit("backend:nextwp", {});
          }
          "show" => {
            let window = app.get_window("main").unwrap();
            window.show().unwrap();
          }
          "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
          }
          "quit" => {
            std::process::exit(0);
          }
          _ => {}
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![wp::update_wallpaper, wp::get_wallpaper, wp::download_wallpaper, commands::db::save2db])
    .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--autostart"])))
    .run(ctx)
    .expect("error while running tauri application");
}
  
fn config_system_tray() -> SystemTray {
  let next = CustomMenuItem::new("next".to_string(), "Next Wallpaper");
  let show = CustomMenuItem::new("show".to_string(), "Show");
  let hide = CustomMenuItem::new("hide".to_string(), "Hide");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu = SystemTrayMenu::new()
  .add_item(next)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(show)
  .add_item(hide)
  .add_native_item(SystemTrayMenuItem::Separator)
  .add_item(quit);
  let tray = SystemTray::new().with_menu(tray_menu);

  return tray;
}