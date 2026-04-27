use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

fn data_dir() -> PathBuf {
    static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();
    DATA_DIR.get_or_init(|| {
        let exe_path = std::env::current_exe().expect("无法获取可执行文件路径");
        let exe_dir = exe_path.parent().expect("无法获取可执行文件目录");
        let dir = exe_dir.join("data");
        fs::create_dir_all(&dir).ok();
        dir
    }).clone()
}

pub fn get_config_path() -> PathBuf {
    data_dir().join("config.json")
}

pub fn get_groups_path() -> PathBuf {
    data_dir().join("groups.json")
}

pub fn get_log_path() -> PathBuf {
    data_dir().join("logs")
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ShortcutConfig {
    pub start_stop: String,
    pub capture_window: String,
}

impl Default for ShortcutConfig {
    fn default() -> Self {
        Self {
            start_stop: "F8".to_string(),
            capture_window: "K".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct StoredEvent {
    pub event_type: String, // mouse | keyboard
    pub action_type: String, // click | hold | scroll
    pub button: Option<String>,
    pub key: Option<String>,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub delta: Option<i32>,
    pub duration_ms: Option<u64>,
    pub interval_ms: u64,
    pub count:Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct AppConfig {
    pub shortcuts: ShortcutConfig,
    pub events: Vec<StoredEvent>,
    pub mode: String, // 'sync' | 'sequence'
    pub max_logs: u32,
}
//TODO: 自定义数据存放目录

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            shortcuts:ShortcutConfig::default(),
            events: Vec::new(),
            mode: "sync".to_string(),
            max_logs: 20,
        }
    }
}

pub fn save_config(config: &AppConfig) -> Result<(), String> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load_config() -> AppConfig {
    let path = get_config_path();
    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(content) => {
                if let Ok(config) = serde_json::from_str::<AppConfig>(&content) {
                    return config;
                } else {
                    eprintln!("Failed to parse config file: {}, using default", content);
                }
            },
            Err(e) => {
            eprintln!("Failed to read config file: {}", e)
            }
        }
        }
    AppConfig::default()
}

pub fn save_root_groups(groups: &Vec<serde_json::Value>) -> Result<(), String> {
    let path = get_groups_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let json = serde_json::to_string_pretty(groups).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

pub fn load_root_groups() -> Result<Vec<serde_json::Value>, String> {
    let path = get_groups_path();
    if path.exists() {
        let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    } else {
        Ok(vec![])
    }
}

pub fn get_data_dir() -> PathBuf {
    data_dir()
}