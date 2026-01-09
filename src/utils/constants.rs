// pub const API_URL: &str = "https://quickshare-latest.onrender.com";
pub const STORAGE_KEY: &str = "quickshare_uploaded_files";

#[cfg(feature = "dev")]
pub const API_URL: &str = "http://0.0.0.0:7777";

#[cfg(feature = "release")]
pub const API_URL: &str = "https://quickshareui.pages.dev/";
