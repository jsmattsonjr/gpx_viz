use crate::config::SRTM_CACHE_DIR;
use std::path::{Path, PathBuf};

// Will be expanded in later milestones
// For now, just basic structure

pub fn get_cache_dir() -> PathBuf {
    PathBuf::from(SRTM_CACHE_DIR)
}

pub fn ensure_cache_dir_exists() -> std::io::Result<()> {
    let dir = get_cache_dir();
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok(())
}

pub fn is_srtm_file_cached(file_name: &str) -> bool {
    let path = get_cache_dir().join(file_name);
    path.exists()
}

pub fn get_cached_srtm_path(file_name: &str) -> PathBuf {
    get_cache_dir().join(file_name)
}

// Will be expanded in later milestones to handle caching SRTM files
