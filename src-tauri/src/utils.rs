use std::path::Path;

pub const IMAGE_EXTENSIONS: &[&str] = &[
    "jpg", "jpeg", "png", "heic", "tif", "tiff", "raf", "arw", "cr2", "cr3", "nef", "dng",
];

pub fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        IMAGE_EXTENSIONS.contains(&ext_str.as_str())
    } else {
        false
    }
}

pub fn has_dcim_folder(volume_path: &Path) -> bool {
    let dcim_path = volume_path.join("DCIM");
    dcim_path.exists() && dcim_path.is_dir()
}

pub fn generate_unique_filename(dest_dir: &Path, original_name: &str) -> String {
    let path = dest_dir.join(original_name);

    if !path.exists() {
        return original_name.to_string();
    }

    // Split filename and extension
    let stem = Path::new(original_name)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(original_name);

    let ext = Path::new(original_name)
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    // Try suffixes (1), (2), etc.
    for i in 1..=9999 {
        let new_name = if ext.is_empty() {
            format!("{} ({})", stem, i)
        } else {
            format!("{} ({}).{}", stem, i, ext)
        };

        let new_path = dest_dir.join(&new_name);
        if !new_path.exists() {
            return new_name;
        }
    }

    // Fallback: append timestamp
    let timestamp = chrono::Utc::now().timestamp();
    if ext.is_empty() {
        format!("{}_{}", stem, timestamp)
    } else {
        format!("{}_{}.{}", stem, timestamp, ext)
    }
}

