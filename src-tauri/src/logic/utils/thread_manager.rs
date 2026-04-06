use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::thread::JoinHandle;
use tauri::{Emitter, State, Window};
use crate::logic::events::events::{execute_event, shutdown_event, InputEvent};

#[derive(Clone, Copy, PartialEq)]
pub enum ExecutionMode {
    Sync, // 同步模式
    Sequence,
}

pub struct ThreadManager {
    mode: Arc<Mutex<ExecutionMode>>,
    events: Arc<Mutex<Vec<InputEvent>>>,
    stop_signal: Arc<AtomicBool>,
    current_handles: Arc<Mutex<Vec<JoinHandle<()>>>>,
}

impl ThreadManager {
    pub fn new() -> Self {
        ThreadManager {
            mode: Arc::new(Mutex::new(ExecutionMode::Sync)),
            events: Arc::new(Mutex::new(Vec::new())),
            stop_signal: Arc::new(AtomicBool::new(false)),
            current_handles: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn set_mode(&self, mode: ExecutionMode) {
        *self.mode.lock().unwrap() = mode;
    }

    pub fn get_mode(&self) -> ExecutionMode {
        *self.mode.lock().unwrap()
    }

    pub fn set_events(&self, events: Vec<InputEvent>) {
        *self.events.lock().unwrap() = events;
    }

    pub fn is_running(&self) -> bool {
        !self.stop_signal.load(Ordering::SeqCst)
    }

    pub fn start(&self) -> Receiver<()>{
        let (tx, rx) = channel();

        self.stop();
        self.stop_signal.store(false, Ordering::SeqCst);

        let mode = *self.mode.lock().unwrap();
        let events = self.events.lock().unwrap().clone();
        if events.is_empty() {
            let _ = tx.send(());
            return rx;
        }

        let stop_signal = Arc::clone(&self.stop_signal);
        let handles = match mode {
            ExecutionMode::Sync => {
                let handles: Vec<_> = self.start_sync(events, stop_signal);
                thread::spawn(move || {
                    for h in handles {
                        let _ = h.join();
                    }
                    let _ = tx.send(());
                })
            }
            ExecutionMode::Sequence => {
                thread::spawn(move || {
                    for event in events {
                        if stop_signal.load(Ordering::SeqCst) { break; }
                        execute_event(&event, &stop_signal);
                    }
                    let _ = tx.send(());
                })
            },
        };
        *self.current_handles.lock().unwrap() = vec![handles];
        rx
    }

    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::SeqCst);

        let mut handles_guard = self.current_handles.lock().unwrap();
        for handle in handles_guard.drain(..) {
            let _ = handle.join();
        }

        let events = self.events.lock().unwrap();
        for event in events.iter() {
            shutdown_event(event);
        }
    }

    pub fn restart(&self) {
        self.stop();
        self.start();
    }

    fn start_sync(&self, events: Vec<InputEvent>, stop_signal: Arc<AtomicBool>) -> Vec<JoinHandle<()>> {
        events
            .into_iter()
            .map(|event| {
                let stop = Arc::clone(&stop_signal);
                thread::spawn(move || {
                    execute_event(&event, &stop);
                })
            })
            .collect()
    }

    fn start_sequence(&self, events: Vec<InputEvent>, stop_signal: Arc<AtomicBool>) -> Vec<JoinHandle<()>> {
        vec![thread::spawn(move || {
            for event in events {
                if stop_signal.load(Ordering::SeqCst) {
                    break;
                }
                execute_event(&event, &stop_signal);
            }
        })]
    }
}

impl Default for ThreadManager {
    fn default() -> Self {
        Self::new()
    }
}