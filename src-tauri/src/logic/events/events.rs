use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::logic::api::keyboard_api::{keyboard_click, keyboard_down, keyboard_up};
use crate::logic::api::mouse_api::mouse_action;
use crate::logic::utils::window_capture::handle_selected_window;

/// 事件动作枚举，定义了不同类型的输入事件
#[derive(Debug, Clone, Serialize, Deserialize)]
// 事件定义
pub enum EventAction {
    Click { // 对于点击事件，有如下选择：
        interval_ms: u64,
        count: Option<u32>, // None表示无限
    },
    Hold { // 对于长按事件，有如下选择：
        duration_ms: u64, // 长按的持续时间
        interval_ms: u64, // 间隔时间
        count: Option<u32>, // 循环次数 None 表示无限
    },
    Scroll {
        delta: i32, // 滚动量
        interval_ms: u64,
        count: Option<u32>,
    },
}

#[derive(Debug, Clone,Serialize, Deserialize)]
pub enum InputEvent {
    Mouse {
        hwnd: usize,
        btn: String,
        action: EventAction,
        x: Option<i32>,
        y: Option<i32>,
    },
    Key {
        hwnd: usize,
        key: String,
        action: EventAction,
    },
}


pub fn execute_event(event: &InputEvent, stop_signal: &Arc<AtomicBool>) {
    match event {
        InputEvent::Mouse {hwnd, btn, action,x,y} => {
            let hwnd_ptr = match handle_selected_window(*hwnd) {
                Some(h) => h,
                None => {
                    eprintln!("Invalid window handle: {}", hwnd);
                    return;
                }
            };

            match action {
                EventAction::Click {interval_ms, count} => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) { break; }
                                let _ = mouse_action(hwnd_ptr, btn, "click", *x, *y, None);
                                if i != c - 1 {
                                    thread::sleep(Duration::from_millis(*interval_ms));
                                }
                            }
                        }
                        None => {
                            loop {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = mouse_action(hwnd_ptr, btn, "click", *x, *y, None);
                                thread::sleep(Duration::from_millis(*interval_ms));
                            }
                        }
                    }
                }
                EventAction::Hold {duration_ms, interval_ms, count} => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) {break };
                                let _ = mouse_action(hwnd_ptr, btn, "down", *x, *y, None);
                                thread::sleep(Duration::from_millis(*duration_ms));
                                let _ = mouse_action(hwnd_ptr, btn, "up", *x, *y, None);
                                if i != c - 1 {
                                    thread::sleep(Duration::from_millis(*interval_ms));
                                }
                            }
                        }
                        None => {
                            loop {
                                if stop_signal.load(Ordering::SeqCst) {break };
                                let _ = mouse_action(hwnd_ptr, btn, "down", *x, *y, None);
                                thread::sleep(Duration::from_millis(*duration_ms));
                                let _ = mouse_action(hwnd_ptr, btn, "up", *x, *y, None);
                                thread::sleep(Duration::from_millis(*interval_ms));
                            }
                        }
                    }
                }
                EventAction::Scroll {delta, interval_ms, count} => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) {break };
                                // 滚轮动作,button为wheel
                                let _ = mouse_action(hwnd_ptr, "wheel", "scroll", *x, *y, Some(*delta));

                                if i != c - 1 {
                                    thread::sleep(Duration::from_millis(*interval_ms));
                                }
                            }
                        }
                        None => {
                            loop {
                                if stop_signal.load(Ordering::SeqCst) {break };
                                let _ = mouse_action(hwnd_ptr, "wheel", "scroll", *x, *y, Some(*delta));
                                thread::sleep(Duration::from_millis(*interval_ms));
                            }
                        }
                    }
                }
            }
        },
        InputEvent::Key {hwnd, key, action} => {
            let hwnd_ptr = match handle_selected_window(*hwnd){
                Some(h) => h,
                None => {
                    eprintln!("Invalid window handle: {}", hwnd);
                    return;
                }
            };

            match action {
                EventAction::Click {interval_ms, count} => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = keyboard_click(hwnd_ptr, key);
                                if i != c - 1 {
                                    thread::sleep(Duration::from_millis(*interval_ms));
                                }
                            }
                        }
                        None => {
                            loop {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = keyboard_click(hwnd_ptr, key);
                                thread::sleep(Duration::from_millis(*interval_ms));
                            }
                        }
                    }
                }
                EventAction::Hold {duration_ms, interval_ms, count} => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) { break; }
                                let _ = keyboard_down(hwnd_ptr, key);
                                thread::sleep(Duration::from_millis(*duration_ms));
                                let _ = keyboard_up(hwnd_ptr, key);
                                if i != c - 1 {
                                    thread::sleep(Duration::from_millis(*interval_ms));
                                }
                            }
                        }
                        None => {
                            loop {
                                if stop_signal.load(Ordering::SeqCst) { break; }
                                let _ = keyboard_down(hwnd_ptr, key);
                                thread::sleep(Duration::from_millis(*duration_ms));
                                let _ = keyboard_up(hwnd_ptr, key);
                                thread::sleep(Duration::from_millis(*interval_ms));
                            }
                        }
                    }
                }
                EventAction::Scroll { .. } => {
                    eprintln!("Warning: Scroll event is not supported for keyboard events");
                }
            }
        }
    }
}

/// 处理停止事件的函数，根据输入事件类型执行相应的停止操作
///
/// # 参数
///
/// * `event` - 一个输入事件引用，可能是鼠标事件或键盘事件
///
pub fn shutdown_event(event: &InputEvent) {
    // 使用 match 语句匹配不同的输入事件类型
    match event {
        // 匹配鼠标事件，包含窗口句柄、鼠标按钮和坐标信息
        InputEvent::Mouse { hwnd, btn, x, y, .. } => {
            // 尝试获取选中的窗口句柄
            if let Some(hwnd_ptr) = handle_selected_window(*hwnd) {
                // 执行鼠标抬起操作，参数包括窗口句柄、按钮类型、坐标和额外信息
                let _ = mouse_action(hwnd_ptr, btn, "up", *x, *y, None);
            }
        }
        // 匹配键盘事件，包含窗口句柄和按键信息
        InputEvent::Key { hwnd, key, .. } => {
            // 尝试获取选中的窗口句柄
            if let Some(hwnd_ptr) = handle_selected_window(*hwnd) {
                // 执行键盘按键释放操作
                let _ = keyboard_up(hwnd_ptr, key);
            }
        }
    }
}