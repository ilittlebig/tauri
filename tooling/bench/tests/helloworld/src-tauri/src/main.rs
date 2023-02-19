// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

#[tauri::command]
fn app_loaded_successfully() {
  std::process::exit(0);
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![app_loaded_successfully])
    .run(tauri::tauri_build_context!())
    .expect("error while running tauri application");
}
