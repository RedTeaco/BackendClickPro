use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::logic::events::events::{EventAction, InputEvent};
use crate::logic::utils::utils::APP_DIR_NAME;

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
    pub max_logs: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            shortcuts:ShortcutConfig::default(),
            events: Vec::new(),
            max_logs: 20,
        }
    }
}

pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().expect("Failed to get config directory");
    let app_dir = config_dir.join(APP_DIR_NAME);
    fs::create_dir_all(&app_dir).ok();
    app_dir.join("config.json")
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

pub fn input_event_to_stored(event: &InputEvent) -> StoredEvent {
    match event {
        InputEvent::Mouse { hwnd: _, btn, action, x, y} => {
            let (action_type, button, duration_ms, interval_ms, count, delta) = match action {
                EventAction::Click { interval_ms, count } => {
                    ("click".to_string(), Some(btn.clone()), None, *interval_ms, *count, None)
                }
                EventAction::Hold { duration_ms, interval_ms, count } => {
                    ("hold".to_string(), Some(btn.clone()), Some(*duration_ms), *interval_ms, *count, None)
                }
                EventAction::Scroll { delta , interval_ms, count} => {
                    ("scroll".to_string(), Some("wheel".to_string()), None, *interval_ms, *count, Some(*delta))
                }
            };
            StoredEvent {
                event_type: "mouse".to_string(),
                action_type,
                button,
                key: None,
                x: *x,
                y: *y,
                delta,
                duration_ms,
                interval_ms,
                count,
            }
        }
        InputEvent::Keyboard { hwnd: _, key, action} => {
            let (action_type, duration_ms, interval_ms, count) = match action {
                EventAction::Click {interval_ms, count} => {
                    ("click".to_string(), None, *interval_ms, *count)
                }
                EventAction::Hold { duration_ms, interval_ms, count } => {
                    ("hold".to_string(), Some(*duration_ms), *interval_ms, *count)
                }
                EventAction::Scroll { .. } => {
                    ("unsupported".to_string(), None, 0, None)
                }
            };
            StoredEvent {
                event_type: "keyboard".to_string(),
                action_type,
                button: None,
                key: Some(key.clone()),
                x: None,
                y: None,
                delta: None,
                duration_ms,
                interval_ms,
                count,
            }
        }
    }
}