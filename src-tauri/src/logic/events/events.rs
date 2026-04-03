use std::thread;
use std::time::Duration;
use crate::logic::api::keyboard_api::{keyboard_click, keyboard_down, keyboard_up};
use crate::logic::utils::window_capture::handle_selected_window;

#[derive(Debug, Clone)]
// 事件定义
pub enum EventAction {
    Click {
        interval_ms: u64,
        count: u32,
    },
    Hold {
        duration_ms: u64,
    }
}

#[derive(Debug, Clone)]
pub enum InputEvent {
    Mouse {
        hwnd: usize,
        btn: String,
        action: EventAction,
    },
    Key {
        hwnd: usize,
        key: String,
        action: EventAction,
    },
}

// 执行单个事件 同步阻塞
pub fn execute_event(event: &InputEvent) {
    match event {
        InputEvent::Mouse {hwnd, btn, action} => {
            let hwnd_unpack = handle_selected_window(*hwnd).unwrap();
            match action {
                EventAction::Click {interval_ms, count} => {
                    for i in 0..*count {
                        //TODO mouse_api
                        if i != count - 1 {
                            thread::sleep(Duration::from_millis(*interval_ms));
                        }
                    }
                }
                EventAction::Hold {duration_ms} => {
                    //TODO mouse_api
                }
            }
        },
        InputEvent::Key {hwnd, key, action} => {
            let hwnd_unpack = handle_selected_window(*hwnd).unwrap();

            match action {

                EventAction::Click {interval_ms, count} => {
                    for i in 0..*count {
                        keyboard_click(hwnd_unpack, key).expect("TODO: panic message");
                        if i != count - 1 {
                            thread::sleep(Duration::from_millis(*interval_ms-10));
                        }
                    }
                }
                EventAction::Hold {duration_ms} => {
                    keyboard_down(hwnd_unpack, key).expect("TODO: panic message");
                    thread::sleep(Duration::from_millis(*duration_ms)); // 持续时间
                    keyboard_up(hwnd_unpack, key).expect("TODO: panic message");
                }
            }
        }
    }
}