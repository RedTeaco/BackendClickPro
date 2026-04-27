use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{Emitter, State, Window};
use crate::logic::events::execution_plan::{execute_node, ExecutionNode};
use crate::logic::utils::{logger, storage};
use crate::logic::utils::thread_manager::{ThreadManager};

#[tauri::command]
pub async fn execute_plan(
    plan: ExecutionNode,
    thread_manager: State<'_, Mutex<ThreadManager>>,
    window: Window,
) -> Result<(), String> {
    let (stop_sig, stop_noti) = {
        let manager = thread_manager.lock().unwrap();
        manager.stop();
        (manager.stop_signal.clone(), manager.stop_notify.clone())
    };

    // 重置停止标志
    stop_sig.store(false, Ordering::SeqCst);

    tokio::spawn(async move {
        execute_node(&plan, &stop_sig, &stop_noti).await;
        let _ = window.emit("execution-completed",());
    });
    // 等待任务完成（无论是正常结束还是因停止信号退出）
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
pub fn stop_execution(thread_manager: State<'_, Mutex<ThreadManager>>) -> Result<(), String> {
    // 获取互斥锁的所有权，如果获取失败则触发 panic
    let manager = thread_manager.lock().unwrap();
    // 调用线程管理器的 stop 方法来停止执行
    manager.stop();
    Ok(())
    //TODO 停止前触发所有事件的shutdown(抬起)
}

#[tauri::command]
pub fn get_execution_status(thread_manager: State<'_, Mutex<ThreadManager>>) -> bool {
    let manager = thread_manager.lock().unwrap();
    manager.is_running()
}

#[tauri::command]
pub fn save_root_groups(groups: Vec<serde_json::Value>) -> Result<(), String> {
    storage::save_root_groups(&groups)
}

#[tauri::command]
pub fn load_root_groups() -> Result<Vec<serde_json::Value>, String> {
    storage::load_root_groups()
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

#[tauri::command]
    // 加载现有的配置文件 deprecated
pub fn save_mode(mode: String) -> Result<(), String> {
    // 更新配置中的模式字段
    let mut config = storage::load_config();
    // 保存更新后的配置
    config.mode = mode;
    storage::save_config(&config)
}

#[tauri::command]
pub fn load_mode() -> Result<String, String> {
    let config = storage::load_config();
    Ok(config.mode)
}

// ========= 日志命令 ==========
#[tauri::command]
pub fn log_message(message: String) -> Result<(), String> {
    logger::log_message(&message);
    Ok(())
}

// ======== 权限命令 ==========
//TODO 前端提供以管理员身份启动的弹窗


