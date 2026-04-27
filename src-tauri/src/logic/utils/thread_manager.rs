// src-tauri/logic/utils/thread_manager.rs

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Notify;

/// 线程管理器：提供停止信号和通知，用于跨任务协调
pub struct ThreadManager {
    pub stop_signal: Arc<AtomicBool>,
    pub stop_notify: Arc<Notify>,
}

impl ThreadManager {
    pub fn new() -> Self {
        Self {
            stop_signal: Arc::new(AtomicBool::new(false)),
            stop_notify: Arc::new(Notify::new()),
        }
    }

    /// 设置停止标志并唤醒所有等待中的任务
    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::SeqCst);
        self.stop_notify.notify_waiters();
    }

    /// 检查是否有任务正在运行（通过停止标志取反）
    pub fn is_running(&self) -> bool {
        !self.stop_signal.load(Ordering::SeqCst)
    }
}

impl Default for ThreadManager {
    fn default() -> Self {
        Self::new()
    }
}