use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct StoredFile {
    pub file_id: String,
    pub filename: String,
    pub download_url: String,
    pub expires_at: String,
    pub uploaded_at: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct UploadResponse {
    pub file_id: String,
    pub download_url: String,
    pub expires_at: String,
}