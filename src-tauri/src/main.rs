// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use screenshots::Screen;
use chrono::Local;
use std::path::PathBuf;

#[tauri::command]
fn snip_to_file(x: i32, y: i32, w: u32, h: u32) -> Result<String, String> {
  let screen = Screen::from_point(x, y).map_err(|e| e.to_string())?;
  // 範囲指定キャプチャ
  let img = screen.capture_area(x, y, w, h).map_err(|e| e.to_string())?;

  // save path
  let dir = dirs::home_dir()
    .ok_or("picture_dir not found")?
    .join("mSnip");
  std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

  let filename = format!("{}.png", Local::now().format("%Y-%m-%d_%H-%M-%S"));
  let path: PathBuf = dir.join(filename);

  image::DynamicImage::ImageRgba8(img)
    .save(&path)
    .map_err(|e| e.to_string())?;

  Ok(path.to_string_lossy().to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![snip_to_file])
    .run(tauri::generate_context!())
    .expect("error while running mSnip");
}
