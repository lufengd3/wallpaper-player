use std::fs;
use std::io::prelude::*;

// #[tauri::command(async)]
#[tauri::command]
// pub async fn save2db(data: String) {
pub fn save2db(data: String) {
  println!("save2db...");
  // println!("save2db {}", data);
  let mut file = fs::OpenOptions::new()
    .write(true)
    .append(true)
    .open("/Users/luf/coding/rust/react-gui/imgdb")
    .unwrap();
  
  if let Err(e) = writeln!(file, "{}", data) {
    eprintln!("Couldn't write to file: {}", e);
  }
}