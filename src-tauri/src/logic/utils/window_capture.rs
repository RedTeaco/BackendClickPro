use std::ffi::c_void;
use windows::core::BOOL;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowLongW, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId, IsIconic, IsWindow, IsWindowVisible, GWL_EXSTYLE, WS_EX_TOOLWINDOW};

#[derive(Debug, serde::Serialize)]
pub struct WindowInfo {
    hwnd: usize, // 窗口句柄
    title: String, // 窗口标题
    process_id: u32, // 窗口进程ID
    is_minimized: bool, // 窗口是否最小化
}

// 枚举所有顶级窗口，回调函数
unsafe extern "system" fn enum_windows_proc(
    hwnd: HWND,
    lparam: LPARAM,
) -> BOOL {
    let windows = &mut *(lparam.0 as *mut Vec<WindowInfo>);

    // 检查窗口是否可见
    if !IsWindowVisible(hwnd).as_bool() {
        return BOOL(1);
    }

    // 过滤掉工具窗口
    let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
    if ex_style & WS_EX_TOOLWINDOW.0 != 0 {
        return BOOL(1);
    }

    // 获取窗口标题
    let title_len = GetWindowTextLengthW(hwnd);
    if title_len == 0 {
        return BOOL(1);
    }

    let mut title_buf = vec![0u16; (title_len + 1) as usize];
    GetWindowTextW(hwnd, &mut title_buf);
    let title = String::from_utf16_lossy(&title_buf[..title_len as usize]);

    // 检查是否最小化
    let is_minimized = IsIconic(hwnd).as_bool();

    // 获取进程ID
    let mut process_id = 0u32;
    GetWindowThreadProcessId(hwnd, Some(&mut process_id));

    windows.push(WindowInfo {
        hwnd: hwnd.0 as usize,
        title,
        process_id,
        is_minimized,
    });

    BOOL(1)
}

#[tauri::command]
pub fn get_windows() -> Result<Vec<WindowInfo>, String> {
    let mut windows = Vec::new();
    unsafe {
        // 枚举所有顶级窗口
        let result = EnumWindows(
            Some(enum_windows_proc),
            LPARAM(&mut windows as *mut Vec<WindowInfo> as isize),
        );

        if result.is_err() {
            return Err("Failed to enumerate windows".to_string());
        }
    }

    let non_minimized: Vec<WindowInfo> = windows
        .into_iter()
        .filter(|w: &WindowInfo| !w.is_minimized)
        .collect();

    Ok(non_minimized)
}

pub fn handle_selected_window(hwnd_u: usize) -> Option<HWND> {
    let hwnd_ptr = hwnd_u as *mut c_void;
    let hwnd = HWND(hwnd_ptr);

    // 验证窗口有效性
    if !unsafe { IsWindow(Some(hwnd)) }.as_bool() {
        return None;
    }

    // 在此处处理选中的窗口，例如获取窗口截图等
    return Some(hwnd)
//TODO: Some解包判断
}