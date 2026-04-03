use std::sync::mpsc::{self,Receiver,Sender};
use std::thread;
use crate::logic::events::events::{execute_event, InputEvent};

enum ConsumerCommand {
    Execute(Vec<InputEvent>, Sender<()>),
    Shutdown,
}

pub struct Consumer {
    _handle: thread::JoinHandle<()>,
    sender: Sender<ConsumerCommand>,
}

impl Consumer {
    pub fn new(name: &str, filter: fn(&InputEvent) -> bool) -> Self {
        let (tx, rx): (Sender<ConsumerCommand>, Receiver<ConsumerCommand>) = mpsc::channel();
        //TODO : HWND类型无法在线程间安全传递，因此需要使用usize类型，传递后再转换为HWND类型
        let handle = thread::spawn(move || {
            for cmd in rx {
                match cmd {
                    ConsumerCommand::Execute(events, done_tx) => {
                        for event in events {
                            if filter(&event) {
                                execute_event(&event);
                            }
                        }
                        let _ = done_tx.send(());
                    }
                    ConsumerCommand::Shutdown => break,
                }
            }
        });
        Consumer {
            _handle: handle,
            sender: tx,
        }
    }

    // 提交一组事件，返回一个接收器，等待执行完成
    fn submit(&self, events: Vec<InputEvent>) -> Receiver<()> {
        let (tx, rx) = mpsc::channel();
        self.sender
            .send(ConsumerCommand::Execute(events, tx))
            .unwrap();
        rx
    }

    fn shutdown(&self) {
        let _ = self.sender.send(ConsumerCommand::Shutdown);
    }
}