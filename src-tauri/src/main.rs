// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{process::Command, time::Duration, env, collections::VecDeque, fs};

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

// Clean up temporary files after the application exits.
// Try to avoid eating up space on people's SSDs with GPU shader caches and other useless shit.
fn cleanup(directory: impl AsRef<str>) {
    std::thread::sleep(Duration::from_millis(250));
    println!("Deleting: {}", directory.as_ref());
    fs::remove_dir_all(directory.as_ref()).unwrap();
}

fn main() {
    let mut args: VecDeque<String> = std::env::args().collect();
    while !args.is_empty() {
        let cur = args.pop_front().unwrap(); 
        match cur.as_str() {
            "-cleanup" => {
                let dir = args.pop_front().expect("You need to provide a directory to cleanup");
                if !dir.contains("AppData") { panic!("This should only be invoked inside AppData. Something probably went wrong...")}
                cleanup(dir);
                return
            }
            _ => {}
        }
    }
    tauri::Builder::default()
        .on_window_event(|event| {
            match event.event() {
                tauri::WindowEvent::CloseRequested { api, .. } => {
                    api.prevent_close();

                    let window = event.window().clone();
                    let cache_dir = window.app_handle().path_resolver().app_cache_dir().unwrap().to_str().unwrap().to_owned();
                    Command::new(
                        std::env::current_exe().unwrap().to_str().unwrap() 
                    )
                    .arg("-cleanup")
                    .arg(cache_dir)
                    .spawn()
                    .expect("Could not re-invoke executable");
                
                    window.close().unwrap();
                    std::process::exit(0)
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![run_command, show_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
