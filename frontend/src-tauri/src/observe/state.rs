//! Observe 模块的核心状态管理
//!
//! 参考 act/state.rs 的设计模式

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Instant;

use crate::communication::types::ZmqPubState;
use super::types::ObserveConfig;

lazy_static! {
    /// 所有活跃的 Observe 会话，key 为 hwnd
    pub static ref SESSIONS: Mutex<HashMap<isize, SessionHandle>> = Mutex::new(HashMap::new());

    /// Observe 模块全局统计（聚合所有会话）
    pub static ref GLOBAL_STATS: Mutex<ObserveGlobalStats> = Mutex::new(ObserveGlobalStats::default());
}

/// 会话运行时统计
#[derive(Debug, Clone, Default)]
pub struct SessionStats {
    pub frames_captured: Arc<AtomicU64>,
    pub bytes_sent: Arc<AtomicU64>,
    pub errors_count: Arc<AtomicU64>,
}

impl SessionStats {
    pub fn new() -> Self {
        Self {
            frames_captured: Arc::new(AtomicU64::new(0)),
            bytes_sent: Arc::new(AtomicU64::new(0)),
            errors_count: Arc::new(AtomicU64::new(0)),
        }
    }

    pub fn frames_captured(&self) -> u64 {
        self.frames_captured.load(Ordering::SeqCst)
    }

    pub fn bytes_sent(&self) -> u64 {
        self.bytes_sent.load(Ordering::SeqCst)
    }

    pub fn errors_count(&self) -> u64 {
        self.errors_count.load(Ordering::SeqCst)
    }

    pub fn add_frames_captured(&self, count: u64) {
        self.frames_captured.fetch_add(count, Ordering::SeqCst);
    }

    pub fn add_bytes_sent(&self, bytes: u64) {
        self.bytes_sent.fetch_add(bytes, Ordering::SeqCst);
    }

    pub fn add_errors(&self, count: u64) {
        self.errors_count.fetch_add(count, Ordering::SeqCst);
    }
}

/// 全局统计（聚合）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ObserveGlobalStats {
    pub total_frames_captured: u64,
    pub total_bytes_sent: u64,
    pub total_errors: u64,
    pub active_sessions: usize,
}

/// 扩展后的 SessionHandle
pub struct SessionHandle {
    /// 窗口句柄
    pub hwnd: isize,
    /// 配置副本（用于查询和调试）
    pub config: ObserveConfig,
    /// 启动时间
    pub start_time: Instant,
    /// 运行控制标志
    pub running: Arc<std::sync::atomic::AtomicBool>,
    /// 捕获线程句柄
    pub capture_handle: Option<JoinHandle<()>>,
    /// ZMQ 发布线程句柄
    pub zmq_handle: Option<JoinHandle<()>>,
    /// 会话级统计
    pub stats: SessionStats,
    /// ZMQ 连接状态
    pub zmq_state: ZmqPubState,
}

impl SessionHandle {
    pub fn new(
        hwnd: isize,
        config: ObserveConfig,
        running: Arc<std::sync::atomic::AtomicBool>,
        capture_handle: Option<JoinHandle<()>>,
        zmq_handle: Option<JoinHandle<()>>,
    ) -> Self {
        Self {
            hwnd,
            config,
            start_time: Instant::now(),
            running,
            capture_handle,
            zmq_handle,
            stats: SessionStats::new(),
            zmq_state: ZmqPubState::new(),
        }
    }

    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// 获取会话运行时长（秒）
    pub fn uptime(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}
