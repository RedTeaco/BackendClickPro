use std::sync::Mutex;
use std::thread;
use tauri::{Emitter, State, Window};
use crate::logic::events::events::InputEvent;
use crate::logic::utils::{logger, storage};
use crate::logic::utils::storage::{input_event_to_stored, StoredEvent};
use crate::logic::utils::thread_manager::{ExecutionMode, ThreadManager};

#[tauri::command]
pub fn set_mode(mode: String, thread_manager: State<Mutex<ThreadManager>>) -> Result<(), String> {
    let mode_enum = match mode.as_str() {
        "sync" => ExecutionMode::Sync,
        "sequence" => ExecutionMode::Sequence,
        _ => return Err("Invalid mode".into()),
    };
    let mut manager = thread_manager.lock().unwrap();
    manager.set_mode(mode_enum);
    Ok(())
}

/// 使用Tauri框架定义的命令函数，用于执行输入事件
///
/// # 参数
/// * `events` - 输入事件向量，包含要执行的事件列表
/// * `mode` - 执行模式的字符串表示，支持"sync"或"sequence"
/// * `thread_manager` - 线程管理器的互斥锁状态，用于管理执行过程
///
/// # 返回值
/// * `Ok(())` - 执行成功
/// * `Err(String)` - 执行失败，返回错误信息
#[tauri::command]
pub fn execute_events(
    events: Vec<InputEvent>,    // 输入事件列表
    mode: String,               // 执行模式，可以是"sync"或"sequence"
    thread_manager: State<Mutex<ThreadManager>>,
    window: Window
) -> Result<(), String> {    // 线程管理器的互斥锁状态
    // 设置模式
    let mode_enum = match mode.as_str() {
        "sync" => ExecutionMode::Sync,
        "sequence" => ExecutionMode::Sequence,
        _ => return Err("Invalid mode".into()),
    };
    let mut manager = thread_manager.lock().unwrap();
    manager.set_mode(mode_enum);
    // 设置事件
    manager.set_events(events);


    let rx = manager.start();
    drop(manager); // 释放锁

    // 在后台线程等待完成并发送事件
    thread::spawn(move || {
        match rx.recv() {
            Ok(_) => {
                let _ = window.emit("execution-completed", ());
            }
            Err(e) => {
                let _ = window.emit("execution-error", e.to_string());
            }
        }
    });
    Ok(())
}

/// 用于停止执行中的线程
///
/// 该函数通过获取线程管理器的互斥锁来安全地访问并停止线程
///
/// # 参数
/// * `thread_manager` - 一个包含 ThreadManager 的互斥锁的状态引用，用于线程管理
///
/// # 使用场景
/// 当用户需要中断或停止当前正在执行的线程时调用此函数
#[tauri::command]
pub fn stop_execution(thread_manager: State<Mutex<ThreadManager>>) -> Result<(), String> {
    // 获取互斥锁的所有权，如果获取失败则触发 panic
    let manager = thread_manager.lock().unwrap();
    // 调用线程管理器的 stop 方法来停止执行
    manager.stop();
    Ok(())
}

#[tauri::command]
pub fn get_execution_status(thread_manager: State<Mutex<ThreadManager>>) -> bool {
    let manager = thread_manager.lock().unwrap();
    manager.is_running()
}

// =========== 持久化存储命令 ===========
#[tauri::command]
pub fn save_events(events: Vec<InputEvent>) -> Result<(), String> {
    let mut config = storage::load_config();
    config.events = events.iter().map(|e| input_event_to_stored(e)).collect();
    storage::save_config(&config)
}

#[tauri::command]
pub fn load_events() -> Result<Vec<StoredEvent>, String> {
    let config = storage::load_config();
    Ok(config.events)
}

#[tauri::command]
pub fn save_shortcuts(shortcuts: storage::ShortcutConfig) -> Result<(), String> {
    let mut config = storage::load_config();
    config.shortcuts = shortcuts;
    storage::save_config(&config)
}

#[tauri::command]
pub fn load_shortcuts() -> Result<storage::ShortcutConfig, String> {
    let config = storage::load_config();
    Ok(config.shortcuts)
}

// ========= 日志命令 ==========
#[tauri::command]
pub fn log_message(message: String) -> Result<(), String> {
    logger::log_message(&message);
    Ok(())
}


