// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;

use tauri::Manager;

// This could really just use the tauri shell API.
#[tauri::command]
async fn run_command(
    command: &str, 
    arg: &str,
    workdir: &str,
) -> Result<String, String> {
    println!("Executed command: {command} with args: {arg}");
    let out = Command::new(command)
        .current_dir(workdir)
        .arg(arg)
        .output();

    match out {
        Ok(x) => Ok(String::from_utf8_lossy(&x.stdout).into_owned()),
        Err(x) => Err(x.to_string())
    }
}


// We need a command to show the window so the screen doesn't flash white
#[tauri::command]
fn show_window(window: tauri::Window) -> String {
    format!("{:?}", window.get_window("main").unwrap().show())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_command, show_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
