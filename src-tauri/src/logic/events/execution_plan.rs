use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use serde::{Deserialize, Serialize};
use tokio::sync::Notify;
use tokio::task;
use async_recursion::async_recursion;

use crate::logic::events::events::{execute_event, InputEvent};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExecutionNode {
    #[serde(rename="event")]
    Event {data: InputEvent},
    #[serde(rename="group")]
    Group {
    mode: String,  // sync or sequence
    children: Vec<ExecutionNode>,
    },
    #[serde(rename="loop")]
    Loop {count: Option<u32>, child: Box<ExecutionNode>},
}

#[async_recursion]
pub async fn execute_node(node: &ExecutionNode, stop_signal: &Arc<AtomicBool>, stop_notify: &Arc<Notify>) {
    println!("[DEBUG] execute_node 进入，节点类型: {:?}", node);
    match node {
        ExecutionNode::Event { data } => {
            execute_event(data, stop_signal, stop_notify).await;
        }
        ExecutionNode::Group { mode, children } => {
            if children.is_empty() {
                return;
            }
            match mode.as_str() {
                "sync" => {
                    let handles: Vec<_> = children
                        .iter()
                        .map(|child| {
                            let stop_sig = stop_signal.clone();
                            let stop_noti = stop_notify.clone();
                            let c = child.clone();
                            task::spawn(async move {
                                execute_node(&c, &stop_sig, &stop_noti).await;
                            })
                        })
                        .collect();
                    for h in handles {
                        let _ = h.await;
                    }
                }
                "sequence" => {
                    for child in children {
                        if stop_signal.load(Ordering::SeqCst) {
                            break;
                        }
                        execute_node(child, &stop_signal.clone(), &stop_notify.clone()).await;
                    }
                }
                _ => eprintln!("[execute_node] Unknown group mode: {}", mode)
            }
        }
        ExecutionNode::Loop { count, child } => {
            match count {
                Some(c) => {
                    for _ in 0..*c {
                        if stop_signal.load(Ordering::SeqCst) { break; }
                        execute_node(child, stop_signal, stop_notify).await;
                    }
                },
                None => {
                    loop {
                        if stop_signal.load(Ordering::SeqCst) { break; }
                        execute_node(child, stop_signal, stop_notify).await;
                    }
                }
            }
        }
    }
}