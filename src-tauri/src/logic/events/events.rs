use crate::logic::api::keyboard_api::{keyboard_click, keyboard_down, keyboard_up};
use crate::logic::api::mouse_api::mouse_action;
use crate::logic::utils::window_capture::is_window_valid;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::select;
use tokio::sync::Notify;

/// 事件动作枚举，定义了不同类型的输入事件
#[derive(Debug, Clone, Serialize, Deserialize)]
// 事件定义
pub enum EventAction {
    Click {
        // 对于点击事件，有如下选择：
        interval_ms: u64,
        count: Option<u32>, // None表示无限
    },
    Hold {
        // 对于长按事件，有如下选择：
        duration_ms: u64,   // 长按的持续时间
        interval_ms: u64,   // 间隔时间
        count: Option<u32>, // 循环次数 None 表示无限
        #[serde(default)]
        continuous: bool,
    },
    Scroll {
        delta: i32, // 滚动量
        interval_ms: u64,
        count: Option<u32>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputEvent {
    Mouse {
        hwnd: usize,
        btn: String,
        action: EventAction,
        x: Option<i32>,
        y: Option<i32>,
    },
    Keyboard {
        hwnd: usize,
        key: String,
        action: EventAction,
    },
}

/// RAII 事件清理守卫
struct EventGuard {
    event: InputEvent,
}

impl EventGuard {
    fn new(event: InputEvent) -> Self {
        Self { event }
    }
}

impl Drop for EventGuard {
    fn drop(&mut self) {
        shutdown_event(&self.event);
    }
}

async fn sleep_interruptible_ms(ms: u64, stop_signal: &Arc<AtomicBool>, notify: &Arc<Notify>) {
    select! {
        _ = tokio::time::sleep(Duration::from_millis(ms)) => {},
        _ = notify.notified() => {}
    }

    if stop_signal.load(Ordering::SeqCst) {
        return;
    }
}

pub async fn execute_event(
    event: &InputEvent,
    stop_signal: &Arc<AtomicBool>,
    notify: &Arc<Notify>,
) {
    let _guard = EventGuard::new(event.clone());
    match event {
        InputEvent::Mouse {
            hwnd,
            btn,
            action,
            x,
            y,
        } => {
            if !is_window_valid(*hwnd) {
                eprintln!("无效窗口句柄: {}", hwnd);
                return;
            }
            match action {
                EventAction::Click { interval_ms, count } => match count {
                    Some(c) => {
                        for i in 0..*c {
                            if stop_signal.load(Ordering::SeqCst) {
                                break;
                            }
                            let _ = mouse_action(*hwnd, btn, "click", *x, *y, None);
                            if i != c - 1 {
                                sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                            }
                        }
                    }
                    None => loop {
                        if stop_signal.load(Ordering::SeqCst) {
                            break;
                        }
                        let _ = mouse_action(*hwnd, btn, "click", *x, *y, None);
                        sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                    },
                },
                EventAction::Hold {
                    duration_ms,
                    interval_ms,
                    count,
                    continuous,
                } => {
                    if *continuous {
                        // 持续模式
                        let _ = mouse_action(*hwnd, btn, "down", *x, *y, None);
                        while !stop_signal.load(Ordering::SeqCst) {
                            sleep_interruptible_ms(50, stop_signal, notify).await;
                        }
                    } else {
                        match count {
                            Some(c) => {
                                for i in 0..*c {
                                    if stop_signal.load(Ordering::SeqCst) {
                                        break;
                                    };
                                    let _ = mouse_action(*hwnd, btn, "down", *x, *y, None);
                                    sleep_interruptible_ms(*duration_ms, stop_signal, notify).await;
                                    let _ = mouse_action(*hwnd, btn, "up", *x, *y, None);
                                    if i != c - 1 {
                                        sleep_interruptible_ms(*interval_ms, stop_signal, notify)
                                            .await;
                                    }
                                }
                            }
                            None => loop {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                };
                                let _ = mouse_action(*hwnd, btn, "down", *x, *y, None);
                                sleep_interruptible_ms(*duration_ms, stop_signal, notify).await;
                                let _ = mouse_action(*hwnd, btn, "up", *x, *y, None);
                                sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                            },
                        }
                    }
                }
                EventAction::Scroll {
                    delta,
                    interval_ms,
                    count,
                } => {
                    match count {
                        Some(c) => {
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                };
                                // 滚轮动作,button为wheel
                                let _ =
                                    mouse_action(*hwnd, "wheel", "scroll", *x, *y, Some(*delta));

                                if i != c - 1 {
                                    sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                                }
                            }
                        }
                        None => loop {
                            if stop_signal.load(Ordering::SeqCst) {
                                break;
                            };
                            let _ = mouse_action(*hwnd, "wheel", "scroll", *x, *y, Some(*delta));
                            sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                        },
                    }
                }
            }
        }
        InputEvent::Keyboard { hwnd, key, action } => {
            if !is_window_valid(*hwnd) {
                eprintln!("无效窗口句柄: {}", hwnd);
                return;
            }
            match action {
                EventAction::Click { interval_ms, count } => {
                    match count {
                        Some(c) => {
                            // ------------------ DEBUG --------------------
                            println!("[KEYBOARD] Expected count: {}", c);
                            // ---------------------------------------------
                            for i in 0..*c {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = keyboard_click(*hwnd, key);
                                if i != c - 1 {
                                    sleep_interruptible_ms(*interval_ms - 50, stop_signal, notify)
                                        .await;
                                }
                            }
                        }
                        None => {
                            // ------------------ DEBUG --------------------
                            println!("[KEYBOARD] Infinite loop");
                            // ---------------------------------------------
                            loop {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = keyboard_click(*hwnd, key);
                                sleep_interruptible_ms(*interval_ms - 50, stop_signal, notify)
                                    .await;
                            }
                            println!("[KEYBOARD] Finished")
                        }
                    }
                }
                EventAction::Hold {
                    duration_ms,
                    interval_ms,
                    count,
                    continuous,
                } => {
                    if *continuous {
                        let _ = keyboard_down(*hwnd, key);
                        while !stop_signal.load(Ordering::SeqCst) {
                            sleep_interruptible_ms(50, stop_signal, notify).await;
                        }
                    } else {
                        match count {
                            Some(c) => {
                                for i in 0..*c {
                                    if stop_signal.load(Ordering::SeqCst) {
                                        break;
                                    }
                                    let _ = keyboard_down(*hwnd, key);
                                    sleep_interruptible_ms(*duration_ms, stop_signal, notify).await;
                                    let _ = keyboard_up(*hwnd, key);
                                    if i != c - 1 {
                                        sleep_interruptible_ms(*interval_ms, stop_signal, notify)
                                            .await;
                                    }
                                }
                            }
                            None => loop {
                                if stop_signal.load(Ordering::SeqCst) {
                                    break;
                                }
                                let _ = keyboard_down(*hwnd, key);
                                sleep_interruptible_ms(*duration_ms, stop_signal, notify).await;
                                let _ = keyboard_up(*hwnd, key);
                                sleep_interruptible_ms(*interval_ms, stop_signal, notify).await;
                            },
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
        InputEvent::Mouse {
            hwnd, btn, x, y, ..
        } => {
            // 执行鼠标抬起操作，参数包括窗口句柄、按钮类型、坐标和额外信息
            if is_window_valid(*hwnd) {
                let _ = mouse_action(*hwnd, btn, "up", *x, *y, None);
            }
        }
        // 匹配键盘事件，包含窗口句柄和按键信息
        InputEvent::Keyboard { hwnd, key, .. } => {
            // 执行键盘按键释放操作
            if is_window_valid(*hwnd) {
                let _ = keyboard_up(*hwnd, key);
            }
        }
    }
}
