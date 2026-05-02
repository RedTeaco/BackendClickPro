use crate::logic::events::execution_plan::{execute_node, ExecutionNode};
use crate::logic::utils::thread_manager::ThreadManager;
use crate::logic::utils::{logger, storage};
use serde_json::json;
use std::sync::atomic::Ordering;
use std::sync::Mutex;
use tauri::{Emitter, State, Window};

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
        let _ = window.emit("execution-completed", ());
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
}

#[tauri::command]
pub fn get_execution_status(thread_manager: State<'_, Mutex<ThreadManager>>) -> bool {
    let manager = thread_manager.lock().unwrap();
    manager.is_running()
}

#[tauri::command]
pub fn save_root_groups(
    groups: Vec<serde_json::Value>,
    total_loop_count: Option<u32>,
) -> Result<(), String> {
    storage::save_root_groups(&groups, total_loop_count)
}

#[tauri::command]
pub fn load_root_groups() -> Result<serde_json::Value, String> {
    let (groups, total_loop_count) = storage::load_root_groups()?;
    Ok(json!({
        "rootGroups": groups,
        "totalLoopCount": total_loop_count
    }))
}

#[tauri::command]
pub fn save_shortcuts(shortcuts: storage::ShortcutConfig) -> Result<(), String> {
    storage::save_shortcuts(shortcuts)
}

#[tauri::command]
pub fn load_shortcuts() -> Result<storage::ShortcutConfig, String> {
    Ok(storage::load_shortcuts())
}

// ========= 日志命令 ==========
#[tauri::command]
pub fn log_message(message: String) -> Result<(), String> {
    logger::log_message(&message);
    Ok(())
}

// ======== 权限命令 ==========
//TODO 前端提供以管理员身份启动的弹窗
