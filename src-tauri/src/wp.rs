use std::fs::{File, read_to_string, create_dir};
use std::io::{copy};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::path::PathBuf;
use std::error::Error;
use std::io::Cursor as StdCursor;
use std::sync::atomic::{AtomicUsize, Ordering};
use image;
use image::io::Reader as ImageReader;
use serde::{Serialize, Deserialize};
use wallpaper;
use reqwest;
use serde_json;
use base64;

use crate::AppState;
use crate::RuntimeSwitches;
// use crate::utils::config::get_config_by_key;

static AUTO_SHARE_CONFIG_KEY: &str = "auto_share";

#[derive(Deserialize, Debug)]
struct AppConfObj {
  img_folder: PathBuf,
}

#[derive(Clone, Serialize)]
struct WpChangedPayload {
  filepath: String,
}

#[tauri::command(async)]
pub async fn update_wallpaper(app_state: tauri::State<'_, AppState>, runtime_switches: tauri::State<'_, RuntimeSwitches>, url: String, base64_img:Option<String>, share_type: Option<String>) -> Result<bool, bool> {
  let wallpaper_file_path: String;
  let remote_wallpaper = url.contains("http://") || url.contains("https://");

  if remote_wallpaper {
    let conf_str: String = read_to_string(app_state.config_file.clone()).unwrap();
    let app_conf: AppConfObj = match serde_json::from_str(&conf_str) {
      Ok(res) => res,
      Err(_) => AppConfObj { img_folder: env::temp_dir() }
    };
    // TODO: check exists
    // let img_folder_exists = Path::new(app_conf.img_folder).exists();
    // let img_folder = img_folder_exists { app_conf.img_folder } else { env::temp_dir() };
    let download_result = download_img(&url, app_conf.img_folder, base64_img).await;
    wallpaper_file_path = match download_result {
      Ok(filepath) => filepath,
      Err(_) => String::from("")
    };
  } else {
    wallpaper_file_path = url.clone();
  }

  if wallpaper_file_path.len() > 0 {
    let wp_res = wallpaper::set_from_path(&wallpaper_file_path);
    match wp_res {
      Ok(data) => {
        println!("change wp success {:?}", data);
        app_state.win.as_ref().unwrap()
          .emit("backend:wpchanged", WpChangedPayload {
            filepath: wallpaper_file_path
          });
        
        if remote_wallpaper {
          match share_type {
            Some(value) if value == "donotshare" => {
            }
            _ => {
              if let Some(val) = runtime_switches.0.lock().unwrap().get(AUTO_SHARE_CONFIG_KEY).cloned() {
                println!("auto share switches {:?}", val);
                if val == true {
                  save2cloud(&url);
                }
              }
            }
          }
        }

        Ok(true)
      },
      Err(e) => {
        println!("error {:?}", e);
        Ok(false)
      }
    }
  } else {
    Ok(false)
  }
}

#[tauri::command(async)]
pub async fn download_wallpaper(app_state: tauri::State<'_, AppState>, url: String, base64_img: Option<String>) -> Result<bool, bool> {
  let conf_str: String = read_to_string(app_state.config_file.clone()).unwrap();
  let app_conf: AppConfObj = match serde_json::from_str(&conf_str) {
    Ok(res) => res,
    Err(_) => AppConfObj { img_folder: env::temp_dir() }
  };
  let download_result = download_img(&url, app_conf.img_folder, base64_img).await;
  match download_result {
    Ok(filepath) => Ok(true),
    Err(_) => Ok(false)
  }
}

#[tauri::command]
pub fn get_wallpaper(app_state: tauri::State<AppState>) -> String {
  let wp_result = wallpaper::get();
  let mut wallpaper_path = String::from("");

  match wp_result {
    Ok(data) => {
      println!("{:?}", data);
      wallpaper_path = data;
    },
    Err(e) => {
      println!("error {:?}", e);
    }
  }

  return wallpaper_path;
}

#[tauri::command]
pub fn update_autoshare_state(runtime_switches: tauri::State<'_, RuntimeSwitches>, next_state: bool) -> Option<bool> {
  runtime_switches.0.lock().unwrap().insert(AUTO_SHARE_CONFIG_KEY.to_string(), next_state);

  let auto_share_state = runtime_switches.0.lock().unwrap().get(AUTO_SHARE_CONFIG_KEY).cloned();
  
  // match auto_share_state {
  //   Some(true) => {
  //     Ok(true)
  //   },
  //   _ => {
  //     Ok(false)
  //   }
  // }

  return auto_share_state;
}

async fn download_img(url: &str, img_folder: PathBuf, base64_img: Option<String>) -> Result<String, Box<dyn Error>> {
    println!("downloading... {} \n", url);
    let random_str = get_epoch_ms().to_string();
    let file_name = format!("{}.jpg", random_str);
    let img_file_path_buf = img_folder.join(file_name.clone());
    let file_path = img_file_path_buf.to_str().unwrap();

    let base64_val;
    match base64_img {
      Some(value) if !value.is_empty() => {
        let parts: Vec<String> = value.split(",").map(|s| s.to_string()).collect();
        base64_val = if parts[1].is_empty() { parts[0].clone() } else { parts[1].clone() };
      }
      _ => {
        base64_val = String::from("");
      }
    }

    let img_bin_data;
    if !base64_val.is_empty() {
      println!("decode base64 data");
      img_bin_data = base64::decode(base64_val).unwrap();
    } else {
      println!("base64 undefined download from remote");
      let res = reqwest::get(url).await?.bytes().await?;
      img_bin_data = res.as_ref().to_vec();
    }
    
    println!("create file at {} \n", file_path);

    let img = ImageReader::new(StdCursor::new(img_bin_data))
      .with_guessed_format()
      .unwrap()
      .decode()
      .unwrap();
    let save_res = img.save(file_path);

    // generate thumbnail
    let thumbnail_folder_path = img_folder.join("thumbnail");
    if !thumbnail_folder_path.exists() {
      create_dir(thumbnail_folder_path.clone())?;
    }
    let thumbnail_file_path = thumbnail_folder_path.join(file_name);
    let thumbnail_full_file_path = thumbnail_file_path.to_str().unwrap();
    let thumbnail = img.resize(300, 300, image::imageops::FilterType::Nearest);
    thumbnail.save(thumbnail_full_file_path)?;

    println!("file_path {} \n", file_path);
    
    Ok(file_path.to_string())
}

fn get_epoch_ms() -> u128 {
  SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap()
      .as_millis()
}


#[derive(Debug, Serialize, Deserialize)]
struct WpBody {
  url: String,
}

fn save2cloud(url: &str) {
  let api = "https://wall-paper.online/wp/addimg";
  println!("api {} \n", api);

  let p = WpBody {
    url: url.to_string()
  };

  tauri::async_runtime::spawn(async move {
    let res = reqwest::Client::new()
        .post(api)
        .json(&p)
        .send()
        .await;
    println!("res is {:?}", res);
  });
}
