use tauri::Manager;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Write;
use id3::{Tag, frame::Picture, frame::PictureType}; // Import Picture and PictureType
use serde::Serialize;
use std::path::Path;

#[derive(Serialize)]
struct Mp3Metadata {
    file: String,
    title: String,
    artist: String,
    cover_art: Option<String>, // Add cover_art field
}

use std::env;
use tauri::command;

#[command]
fn get_current_dir() -> Result<String, String> {
    let mut current_dir = env::current_dir().map_err(|err| err.to_string())?;
    current_dir.pop(); // Move up one directory
    Ok(current_dir.display().to_string())
}

#[tauri::command]
fn get_os() -> String {
    std::env::consts::OS.to_string()
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;

    pub fn check_cd() -> Result<String, String> {
        let output = Command::new("wmic")
            .args(&["logicaldisk", "where", "DriveType=5", "get", "DeviceID"])
            .output()
            .map_err(|e| e.to_string())?;
        
        let cd_info = String::from_utf8_lossy(&output.stdout);
        
        for line in cd_info.lines() {
            if line.contains(':') {
                let drive_letter = line.trim().to_string();
                let drive_path = format!("{}\\", drive_letter);
                match fs::read_dir(&drive_path) {
                    Ok(entries) => {
                        if entries.count() > 0 {
                            println!("CD Detected with files");
                            return Ok(drive_letter); // Return the drive letter with CD
                        } else {
                            return Err("CD detected but no files".into());
                        }
                    }
                    Err(_) => {
                        return Err("Drive not ready or accessible".into());
                    }
                }
            }
        }
        
        Err("No CD Detected".into())
    }
    
    pub fn extract_mp3_metadata(drive: &str) -> Result<Vec<Mp3Metadata>, String> {
        let mut metadata_list = Vec::new();
        for entry in fs::read_dir(drive).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            if path.extension().map(|ext| ext == "mp3").unwrap_or(false) {
                let tag = Tag::read_from_path(&path).map_err(|e| e.to_string())?;
                let title = tag.title().unwrap_or("Unknown Title").to_string();
                let artist = tag.artist().unwrap_or("Unknown Artist").to_string();
                
                // Extract cover art
                let cover_art = tag
                    .pictures()
                    .find(|pic| pic.picture_type == PictureType::CoverFront)
                    .map(|pic| {
                        let cover_art_path = path.with_extension("jpg");
                        fs::write(&cover_art_path, &pic.data).ok();
                        cover_art_path.to_string_lossy().to_string()
                    });
                
                metadata_list.push(Mp3Metadata {
                    file: path.to_string_lossy().to_string(),
                    title,
                    artist,
                    cover_art,
                });
            }
        }
        Ok(metadata_list)
    }
    

    pub fn generate_metadata_json(drive: &str, output_path: &str) -> Result<(), String> {
        let path = Path::new(output_path);
        println!("Received output path: {:?}", path);
        
        // Check if path exists or if there's an issue
        if !path.exists() {
            println!("Path does not exist: {:?}", path);
        }
        
        let output_dir = path.parent().ok_or("Invalid output path")?;
        println!("Output directory: {:?}", output_dir);
        
        fs::create_dir_all(output_dir).map_err(|e| e.to_string())?;
        
        let metadata_list = extract_mp3_metadata(drive).map_err(|e| e.to_string())?;
        let json = serde_json::to_string_pretty(&metadata_list).map_err(|e| e.to_string())?;
        let mut file = File::create(path).map_err(|e| e.to_string())?;
        file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[tauri::command]
fn check_cd_inserted() -> Result<String, String> {
    platform::check_cd()
}

#[tauri::command]
fn generate_metadata_json(drive: &str, output_path: &str) -> Result<(), String> {
    platform::generate_metadata_json(drive, output_path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_os, check_cd_inserted, generate_metadata_json, get_current_dir])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
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
