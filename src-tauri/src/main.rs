use tauri::Manager;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Write;
use id3::{Tag, Error};
use serde::Serialize;

#[derive(Serialize)]
struct Mp3Metadata {
    file: String,
    title: String,
    artist: String,
}

#[tauri::command]
fn get_os() -> String {
    std::env::consts::OS.to_string()
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;

    pub fn check_cd() -> bool {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "where", "DriveType=5", "get", "DeviceID"])
            .output()
            .expect("Failed to execute `wmic` command");

        let cd_info = String::from_utf8_lossy(&output.stdout);

        for line in cd_info.lines() {
            if line.contains(':') {
                let drive_letter = line.trim().to_string();
                let drive_path = format!("{}\\", drive_letter);
                match fs::read_dir(&drive_path) {
                    Ok(entries) => {
                        if entries.count() > 0 {
                            println!("CD Detected with files");
                            return true; // CD and files detected
                        } else {
                            println!("CD Detected but no files");
                            return false; // CD detected but no files
                        }
                    }
                    Err(_) => {
                        println!("Permission error: Drive not ready or accessible");
                        return false; // CD detected but drive not ready
                    }
                }
            }
        }

        println!("No CD Detected");
        false // No CD detected
    }
    

    pub fn extract_mp3_metadata(drive: &str) -> Result<Vec<Mp3Metadata>, Error> {
        let mut metadata_list = Vec::new();
        for entry in fs::read_dir(drive)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().map(|ext| ext == "mp3").unwrap_or(false) {
                let tag = Tag::read_from_path(&path)?;
                let title = tag.title().unwrap_or("Unknown Title").to_string();
                let artist = tag.artist().unwrap_or("Unknown Artist").to_string();
                metadata_list.push(Mp3Metadata {
                    file: path.to_string_lossy().to_string(),
                    title,
                    artist,
                });
            }
        }
        Ok(metadata_list)
    }

    pub fn generate_metadata_json(drive: &str, output_path: &str) -> Result<(), String> {
        let metadata_list = extract_mp3_metadata(drive).map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&metadata_list).map_err(|e| e.to_string())?;
        let mut file = File::create(output_path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(target_os = "linux")]
mod platform {
    // Implement Linux-specific logic here
    pub fn check_cd() -> bool {
        false
    }

    pub fn generate_metadata_json(_drive: &str, _output_path: &str) -> Result<(), String> {
        Err("Not implemented".into())
    }
}

#[cfg(target_os = "macos")]
mod platform {
    // Implement macOS-specific logic here
    pub fn check_cd() -> bool {
        false
    }

    pub fn generate_metadata_json(_drive: &str, _output_path: &str) -> Result<(), String> {
        Err("Not implemented".into())
    }
}

#[tauri::command]
fn check_cd_inserted() -> bool {
    platform::check_cd()
}

#[tauri::command]
fn generate_metadata_json(drive: &str, output_path: &str) -> Result<(), String> {
    platform::generate_metadata_json(drive, output_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_os, check_cd_inserted, generate_metadata_json])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
