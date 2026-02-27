use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

use crate::types::{ImportManifestEntry, ImportProgress};
use crate::utils::generate_unique_filename;
use crate::volume::find_all_images;

pub struct ImportManager {
    active_imports: Arc<Mutex<HashMap<String, bool>>>, // import_id -> cancelled flag
}

impl ImportManager {
    pub fn new() -> Self {
        Self {
            active_imports: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn start_import(
        &self,
        app: AppHandle,
        import_id: String,
        volume_path: String,
        destination_root: String,
        folder_name: String,
    ) {
        let active = self.active_imports.clone();

        // Register this import
        {
            let mut imports = active.lock().unwrap();
            imports.insert(import_id.clone(), false);
        }

        // Spawn import task
        tauri::async_runtime::spawn(async move {
            let result = run_import(
                app.clone(),
                import_id.clone(),
                volume_path,
                destination_root,
                folder_name,
                active.clone(),
            )
            .await;

            // Clean up
            let mut imports = active.lock().unwrap();
            imports.remove(&import_id);

            if let Err(e) = result {
                let _ = app.emit(
                    "import-progress",
                    ImportProgress {
                        import_id: import_id.clone(),
                        total_files: 0,
                        copied_files: 0,
                        current_file: String::new(),
                        status: "failed".to_string(),
                        error: Some(e),
                    },
                );
            }
        });
    }

    pub fn cancel_import(&self, import_id: &str) -> Result<(), String> {
        let mut imports = self.active_imports.lock().unwrap();

        if let Some(cancelled) = imports.get_mut(import_id) {
            *cancelled = true;
            Ok(())
        } else {
            Err("Import not found".to_string())
        }
    }

    fn is_cancelled(&self, import_id: &str) -> bool {
        let imports = self.active_imports.lock().unwrap();
        imports.get(import_id).copied().unwrap_or(false)
    }
}

async fn run_import(
    app: AppHandle,
    import_id: String,
    volume_path: String,
    destination_root: String,
    folder_name: String,
    active_imports: Arc<Mutex<HashMap<String, bool>>>,
) -> Result<(), String> {
    let volume = Path::new(&volume_path);
    let dest_root = Path::new(&destination_root);
    let dest_folder = dest_root.join(&folder_name);

    // Create destination folder
    fs::create_dir_all(&dest_folder)
        .map_err(|e| format!("Failed to create destination folder: {}", e))?;

    // Find all images
    let image_files = find_all_images(volume)?;
    let total_files = image_files.len();

    let mut manifest: Vec<ImportManifestEntry> = Vec::new();
    let mut copied_files = 0;

    // Determine base path for relative paths
    let base_path = if volume.join("DCIM").exists() {
        volume.join("DCIM")
    } else {
        volume.to_path_buf()
    };

    for source_path in image_files {
        // Check if cancelled
        {
            let imports = active_imports.lock().unwrap();
            if imports.get(&import_id).copied().unwrap_or(false) {
                let _ = app.emit(
                    "import-progress",
                    ImportProgress {
                        import_id: import_id.clone(),
                        total_files,
                        copied_files,
                        current_file: String::new(),
                        status: "cancelled".to_string(),
                        error: Some("Import cancelled by user".to_string()),
                    },
                );
                return Err("Import cancelled".to_string());
            }
        }

        // Calculate relative path to preserve structure
        let relative_path = source_path
            .strip_prefix(&base_path)
            .unwrap_or(&source_path);

        let dest_path = dest_folder.join(relative_path);

        // Create parent directories
        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Handle duplicate filenames
        let final_dest_path = if dest_path.exists() {
            let parent = dest_path.parent().unwrap_or(Path::new(""));
            let original_name = dest_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown");
            let unique_name = generate_unique_filename(parent, original_name);
            parent.join(unique_name)
        } else {
            dest_path
        };

        // Copy file
        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        fs::copy(&source_path, &final_dest_path)
            .map_err(|e| format!("Failed to copy {}: {}", file_name, e))?;

        // Get file size
        let metadata = fs::metadata(&source_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        // Add to manifest
        manifest.push(ImportManifestEntry {
            source_path: source_path.to_string_lossy().to_string(),
            dest_path: final_dest_path.to_string_lossy().to_string(),
            size_bytes: metadata.len(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        });

        copied_files += 1;

        // Emit progress
        let _ = app.emit(
            "import-progress",
            ImportProgress {
                import_id: import_id.clone(),
                total_files,
                copied_files,
                current_file: file_name,
                status: "running".to_string(),
                error: None,
            },
        );
    }

    // Write manifest
    let manifest_path = dest_folder.join("import-manifest.json");
    let manifest_json = serde_json::to_string_pretty(&manifest)
        .map_err(|e| format!("Failed to serialize manifest: {}", e))?;

    fs::write(&manifest_path, manifest_json)
        .map_err(|e| format!("Failed to write manifest: {}", e))?;

    // Emit completion
    let _ = app.emit(
        "import-progress",
        ImportProgress {
            import_id: import_id.clone(),
            total_files,
            copied_files,
            current_file: String::new(),
            status: "completed".to_string(),
            error: None,
        },
    );

    Ok(())
}

