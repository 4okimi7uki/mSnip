// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use chrono::Local;
use std::path::PathBuf;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
async fn screencap_interactive(app: tauri::AppHandle) -> Result<String, String> {
    let dir = dirs::home_dir()
        .ok_or("home_dir not found")?
        .join("mSnip");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

    let filename = format!("{}.png", Local::now().format("%Y-%m-%d_%H-%M-%S"));
    let path: PathBuf = dir.join(filename);

    // screencapture 実行（-i: インタラクティブ, -x: 無音, -t png: 明示）
    let shell = app.shell();
    let cmd = shell.command("screencapture");
    let out = cmd
        .args(["-i", "-x", "-t", "png", path.to_string_lossy().as_ref()])
        .output()
        .await
        .map_err(|e| e.to_string())?;

    // ユーザーがキャンセルすると非0で返る場合がある
    if !out.status.success() {
        return Err("capture cancelled or failed".into());
    }

    Ok(path.to_string_lossy().to_string())
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![screencap_interactive])
        .run(tauri::generate_context!())
        .expect("error while running mSnip");
}
