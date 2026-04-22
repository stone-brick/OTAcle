//! Act 模块的 Action 业务状态管理
//!
//! 包含动作配置管理相关的全局状态

use super::types::{ActionList, HistoryEntry, InputBackend};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// 全局动作配置列表存储（保留顺序和名称）
    pub static ref ACTION_CONFIG_LIST: Mutex<Option<ActionList>> = Mutex::new(None);
    /// 没有明确后端的动作的全局默认输入后端
    pub static ref DEFAULT_INPUT_BACKEND: Mutex<InputBackend> = Mutex::new(InputBackend::Win32);
    /// 动作执行的目标窗口（HWND）
    pub static ref TARGET_WINDOW: Mutex<Option<isize>> = Mutex::new(None);
    /// 撤销历史栈
    pub static ref UNDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// 重做历史栈
    pub static ref REDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// 用于丢弃的原始状态（加载的或上次保存的）
    pub static ref ORIGINAL_CONFIG_SNAPSHOT: Mutex<Option<HistoryEntry>> = Mutex::new(None);
}
