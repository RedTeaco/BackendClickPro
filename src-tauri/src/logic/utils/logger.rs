use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use flate2::Compression;
use flate2::write::GzEncoder;
use lazy_static::lazy_static;
use tar::Builder;
use crate::logic::utils::storage;

pub struct Logger {
    current_log_path: PathBuf,
    max_logs: u32,
}

impl Logger {
    pub fn new(max_logs: u32) -> Self {
        let log_dir = storage::get_log_path();
        fs::create_dir_all(&log_dir).unwrap();

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log_path = log_dir.join(format!("clicker_{}.log", timestamp));

        Logger {
            current_log_path: log_path,
            max_logs,
        }
    }

    pub fn log(&self, message: &str) -> Result<(), String>{
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!("[{}] {}\n", timestamp, message);

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.current_log_path)
            .map_err(|e| e.to_string())?;

        file.write_all(log_line.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn rotate_logs(&self) -> Result<(), String> {
        let log_dir = self.current_log_path.parent().unwrap();
        let mut log_files: Vec<PathBuf> = fs::read_dir(log_dir)
            .map_err(|e| e.to_string())?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();
                if path.extension()? == "log" {
                    Some(path)
                } else {
                    None
                }
            })
            .collect();

        log_files.sort_by_key(|path| {
            fs::metadata(path).and_then(|m| m.modified()).ok()
        });

        while log_files.len() > self.max_logs as usize {
            if let Some(oldest) = log_files.first() {
                self.archive_log(oldest)?;
                fs::remove_file(oldest).map_err(|e| e.to_string())?;
                log_files.remove(0);
            }
        }
        Ok(())
    }

    fn archive_log(&self, log_path: &PathBuf) -> Result<(), String> {
        let tar_path = log_path.with_extension("tar.gz");
        let file = File::create(&tar_path).map_err(|e| e.to_string())?;
        let gz_encoder = GzEncoder::new(file, Compression::default());
        let mut tar_builder = Builder::new(gz_encoder);

        let file_name = log_path.file_name().unwrap().to_str().unwrap();
        tar_builder.append_file(file_name, &mut File::open(log_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;

        tar_builder.finish().map_err(|e| e.to_string())?;
        Ok(())
    }
}

lazy_static!{
    static ref LOGGER: Mutex<Logger> = Mutex::new(Logger::new(100));
}

pub fn log_message(message: &str) {
    if let Ok(logger) = LOGGER.lock() {
        let _ = logger.log(message);
        let _ = logger.rotate_logs();
    }
}