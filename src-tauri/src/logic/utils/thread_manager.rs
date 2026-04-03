use crate::logic::consumer::consumer::Consumer;
use crate::logic::events::events::InputEvent;

pub struct ThreadManager {
    mouse_consumer: Consumer,
    keyboard_consumer: Consumer,
}

impl ThreadManager {
    pub fn new() -> Self {
        // 过滤器：对应consumer
        let mouse_consumer = Consumer::new("mouse", |e| matches!(e, InputEvent::Mouse {.. }));
        let keyboard_consumer = Consumer::new("keyboard", |e| matches!(e, InputEvent::Key {.. }));
        ThreadManager {
            mouse_consumer,
            keyboard_consumer,
        }
    }
}