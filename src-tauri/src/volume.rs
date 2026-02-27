use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::types::ScanResult;
use crate::utils::{has_dcim_folder, is_image_file};

pub fn list_volumes() -> Result<Vec<String>, String> {
    let volumes_dir = Path::new("/Volumes");

    if !volumes_dir.exists() {
        println!("Warning: /Volumes does not exist");
        // Windows fallback - check for removable drives
        #[cfg(target_os = "windows")]
        {
            return list_windows_volumes();
        }

        #[cfg(not(target_os = "windows"))]
        return Ok(vec![]);
    }

    let mut volumes = Vec::new();

    match fs::read_dir(volumes_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(path) = entry.path().canonicalize() {
                    if path.is_dir() {
                        let path_str = path.to_string_lossy().to_string();
                        println!("Found volume: {}", path_str);
                        volumes.push(path_str);
                    }
                }
            }
        }
        Err(e) => return Err(format!("Failed to read volumes: {}", e)),
    }

    println!("Total volumes found: {}", volumes.len());
    Ok(volumes)
}

#[cfg(target_os = "windows")]
fn list_windows_volumes() -> Result<Vec<String>, String> {
    use std::fs;
    let mut volumes = Vec::new();

    for letter in b'A'..=b'Z' {
        let drive = format!("{}:\\", letter as char);
        if Path::new(&drive).exists() {
            volumes.push(drive);
        }
    }

    Ok(volumes)
}

pub fn scan_volume_for_images(volume_path: &str) -> Result<ScanResult, String> {
    let path = Path::new(volume_path);

    if !path.exists() {
        return Err(format!("Volume does not exist: {}", volume_path));
    }

    let mut image_paths = Vec::new();
    let search_path = if has_dcim_folder(path) {
        println!("Found DCIM folder in: {}", volume_path);
        path.join("DCIM")
    } else {
        println!("No DCIM folder, scanning entire volume: {}", volume_path);
        path.to_path_buf()
    };

    // Walk directory tree and find image files
    let walker = WalkDir::new(&search_path)
        .max_depth(10)
        .follow_links(false)
        .into_iter();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();

                if entry_path.is_file() && is_image_file(entry_path) {
                    if let Some(relative) = entry_path.strip_prefix(&search_path).ok() {
                        image_paths.push(relative.to_string_lossy().to_string());
                    } else {
                        image_paths.push(entry_path.to_string_lossy().to_string());
                    }

                    // Limit sample to first 100 for performance during scan
                    if image_paths.len() >= 100 {
                        println!("Found 100+ images, stopping scan for performance");
                        break;
                    }
                }
            }
            Err(e) => {
                // Skip directories we can't read (permissions, etc)
                eprintln!("Warning: Could not read entry: {}", e);
                continue;
            }
        }
    }

    let image_count = image_paths.len();
    let sample_paths = image_paths.into_iter().take(10).collect();

    println!("Scan complete: found {} images", image_count);

    Ok(ScanResult {
        has_images: image_count > 0,
        image_count,
        sample_paths,
    })
}

pub fn find_all_images(volume_path: &Path) -> Result<Vec<PathBuf>, String> {
    let mut image_files = Vec::new();

    let search_path = if has_dcim_folder(volume_path) {
        volume_path.join("DCIM")
    } else {
        volume_path.to_path_buf()
    };

    for entry in WalkDir::new(&search_path)
        .max_depth(10)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let entry_path = entry.path();

        if entry_path.is_file() && is_image_file(entry_path) {
            image_files.push(entry_path.to_path_buf());
        }
    }

    Ok(image_files)
}

