// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::thread;
use tokio::task;


type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

async fn start_script_refresh() -> Result<()> {
    let file = String::from("/usr/share/pkg-test/scripts/update_system.sh");

    let _output = Command::new("konsole")
        .arg("-e")
        .arg("sudo")
        .arg("bash")
        .arg(&file)
        .output()
        .expect("[!] Failed to execute process...");

    Ok(())
}

#[tokio::main]
async fn run_refresh() -> Result<()> {
    let script_thread = task::spawn(start_script_refresh());

    let _result = script_thread.await??;

    Ok(())
}

#[tauri::command]
async fn update() {
    let handle = thread::spawn(|| {
        match run_refresh() {
            // If function returned OK...
            Ok(_) => {
                print!("\n\n");
                println!("[+] Process finished successfully!");
            }
            // Otherwise...
            Err(_) => {
                print!("\n\n");
                println!("[!] Async function returned errors...");
            },
        }
    });

    
    handle.join().unwrap();
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, update])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
