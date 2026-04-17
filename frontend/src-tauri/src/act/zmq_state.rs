//! Act 模块的 ZMQ PULL 状态管理
//!
//! 用于 Act 模块从 Python 接收命令的 ZMQ PULL 状态

use lazy_static::lazy_static;
use std::sync::{Arc, atomic::AtomicBool, Mutex};

lazy_static! {
    /// ZMQ 接收者状态（用于 act_zmq_* 命令）
    pub static ref ZMQ_STATE: Mutex<Option<ZmqState>> = Mutex::new(None);
}

/// ZMQ 接收者状态
pub struct ZmqState {
    pub running: Mutex<Option<Arc<AtomicBool>>>,
    pub address: Mutex<String>,
}

impl Default for ZmqState {
    fn default() -> Self {
        Self {
            running: Mutex::new(None),
            address: Mutex::new("tcp://127.0.0.1:5555".to_string()),
        }
    }
}
