use std::{
    fs,
    path::{Path, PathBuf},
    io::{Read, Write},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CachedTrack {
    pub title: String,
    pub artist: String,
    pub bpm: f32,
    pub duration: f32,
    pub key: String,
}

pub fn get_cache_folder() -> PathBuf {
    let mut folder = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    folder.push(".dj-cache");
    fs::create_dir_all(&folder).ok();
    folder
}

pub fn cache_path_for(track_path: &Path) -> PathBuf {
    let hash = blake3::hash(track_path.to_string_lossy().as_bytes()).to_hex().to_string();
    let mut path = get_cache_folder();
    path.push(format!("{}.json", hash));
    path
}

pub fn load_cached_metadata(track_path: &Path) -> Option<CachedTrack> {
    let path = cache_path_for(track_path);
    if path.exists() {
        let mut file = fs::File::open(path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        serde_json::from_str(&contents).ok()
    } else {
        None
    }
}

pub fn write_cached_metadata(track_path: &Path, data: &CachedTrack) {
    let path = cache_path_for(track_path);
    if let Ok(json) = serde_json::to_string_pretty(data) {
        let _ = fs::write(path, json);
    }
}
