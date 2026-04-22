//! 撤销/重做历史管理
//!
//! 提供完整的撤销/重做功能，包括状态管理和历史栈操作。

use super::state::{
    ACTION_CONFIG_LIST, DEFAULT_INPUT_BACKEND, ORIGINAL_CONFIG_SNAPSHOT, REDO_STACK, UNDO_STACK,
};
use super::types::HistoryEntry;

/// 最大历史记录条目数
const MAX_HISTORY_SIZE: usize = 50;

/// 保存当前状态到撤销历史
///
/// # 参数
/// * `actions` - 当前的 ActionList (Vec<ActionItem>)
/// * `default_backend` - 当前的默认后端
pub fn save_to_history(
    actions: super::types::ActionList,
    default_backend: super::types::InputBackend,
) -> Result<(), String> {
    let entry = HistoryEntry {
        actions,
        default_backend,
    };

    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    undo.push(entry);

    // 限制历史大小
    if undo.len() > MAX_HISTORY_SIZE {
        undo.remove(0);
    }

    // 新动作清除重做栈
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;
    redo.clear();

    Ok(())
}

/// 撤销操作 - 从撤销栈弹出上一个状态并应用到全局
///
/// 返回 Err 如果撤销栈为空
pub fn undo() -> Result<(), String> {
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;
    let entry = pop_undo(actions, current_backend)?;
    apply_entry(entry)
}

/// 重做操作 - 从重做栈弹出下一个状态并应用到全局
///
/// 返回 Err 如果重做栈为空
pub fn redo() -> Result<(), String> {
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;
    let entry = pop_redo(actions, current_backend)?;
    apply_entry(entry)
}

/// 丢弃所有更改 - 恢复到原始加载状态
///
/// 返回 Err 如果没有原始快照
pub fn discard() -> Result<(), String> {
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;
    let entry = restore_original(actions, current_backend)?;
    apply_entry(entry)
}

/// 将 HistoryEntry 应用到全局状态
fn apply_entry(entry: HistoryEntry) -> Result<(), String> {
    let mut list = ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list")?;
    *list = Some(entry.actions);
    drop(list);
    let mut backend = DEFAULT_INPUT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock backend")?;
    *backend = entry.default_backend;
    Ok(())
}

/// 获取当前动作列表
fn get_action_list() -> Result<super::types::ActionList, String> {
    let list = ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list")?;
    list.clone()
        .ok_or_else(|| "No action list loaded".to_string())
}

/// 获取当前默认后端
fn get_default_backend() -> Result<super::types::InputBackend, String> {
    let backend = DEFAULT_INPUT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock backend")?;
    Ok(backend.clone())
}

/// 准备撤销操作 - 从撤销栈弹出状态（不直接应用）
///
/// 返回 HistoryEntry 如果可以撤销，Err 如果不能
fn pop_undo(
    actions: super::types::ActionList,
    default_backend: super::types::InputBackend,
) -> Result<HistoryEntry, String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if undo.is_empty() {
        return Err("Nothing to undo".to_string());
    }

    // 保存当前状态到重做栈
    let current = HistoryEntry {
        actions,
        default_backend,
    };
    redo.push(current);

    // 返回上一个状态
    let prev = undo.pop().unwrap();
    Ok(prev)
}

/// 准备重做操作 - 从重做栈获取状态（不直接应用）
///
/// 返回 HistoryEntry 如果可以重做，Err 如果不能
fn pop_redo(
    actions: super::types::ActionList,
    default_backend: super::types::InputBackend,
) -> Result<HistoryEntry, String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if redo.is_empty() {
        return Err("Nothing to redo".to_string());
    }

    // 保存当前状态到撤销栈
    let current = HistoryEntry {
        actions,
        default_backend,
    };
    undo.push(current);

    // 返回下一个状态
    let next = redo.pop().unwrap();
    Ok(next)
}

/// 准备丢弃操作 - 返回原始状态快照（保存当前到历史）
///
/// 返回 HistoryEntry 如果有原始快照，Err 如果没有
fn restore_original(
    actions: super::types::ActionList,
    default_backend: super::types::InputBackend,
) -> Result<HistoryEntry, String> {
    // 从原始状态恢复
    let original = ORIGINAL_CONFIG_SNAPSHOT
        .lock()
        .map_err(|_| "Failed to lock original")?;
    let entry = original
        .clone()
        .ok_or_else(|| "No original state to discard to".to_string())?;
    drop(original);

    // 保存当前状态到历史记录（以便我们可以撤销）
    let current = HistoryEntry {
        actions,
        default_backend,
    };
    save_to_history(current.actions.clone(), current.default_backend)?;

    Ok(entry)
}

/// 清除撤销/重做历史
pub fn clear_history() -> Result<(), String> {
    UNDO_STACK
        .lock()
        .map_err(|_| "Failed to lock undo stack")?
        .clear();
    REDO_STACK
        .lock()
        .map_err(|_| "Failed to lock redo stack")?
        .clear();
    Ok(())
}

/// 获取历史状态（撤销计数，重做计数）
pub fn get_history_status() -> (usize, usize) {
    let undo_len = UNDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    let redo_len = REDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    (undo_len, redo_len)
}

/// 设置原始条目（供 config 模块在加载配置时调用）
pub fn set_baseline_snapshot(entry: HistoryEntry) -> Result<(), String> {
    let mut original = ORIGINAL_CONFIG_SNAPSHOT
        .lock()
        .map_err(|_| "Failed to lock original")?;
    *original = Some(entry);
    Ok(())
}
