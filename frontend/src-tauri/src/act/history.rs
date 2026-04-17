//! 撤销/重做历史管理
//!
//! 这些函数作为纯函数，接收当前状态值作为参数，返回新状态值。
//! 调用者（config）负责实际的状态读写。

use super::action_state::{UNDO_STACK, REDO_STACK, ORIGINAL_CONFIG_SNAPSHOT};
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

/// 准备撤销操作 - 返回应恢复到的新状态
///
/// 返回 HistoryEntry 如果可以撤销，Err 如果不能
pub fn prepare_undo(
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

/// 准备重做操作 - 返回应恢复到的新状态
///
/// 返回 HistoryEntry 如果可以重做，Err 如果不能
pub fn prepare_redo(
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

/// 丢弃所有更改 - 返回原始状态（可撤销）
pub fn prepare_discard(
    actions: super::types::ActionList,
    default_backend: super::types::InputBackend,
) -> Result<HistoryEntry, String> {
    // 从原始状态恢复
    let original = ORIGINAL_CONFIG_SNAPSHOT.lock().map_err(|_| "Failed to lock original")?;
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
    UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?.clear();
    REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?.clear();
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
    let mut original = ORIGINAL_CONFIG_SNAPSHOT.lock().map_err(|_| "Failed to lock original")?;
    *original = Some(entry);
    Ok(())
}
