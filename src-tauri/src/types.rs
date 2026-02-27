use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanResult {
    pub has_images: bool,
    pub image_count: usize,
    pub sample_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportProgress {
    pub import_id: String,
    pub total_files: usize,
    pub copied_files: usize,
    pub current_file: String,
    pub status: String, // "running", "completed", "failed", "cancelled"
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImportManifestEntry {
    pub source_path: String,
    pub dest_path: String,
    pub size_bytes: u64,
    pub timestamp: String,
}

