//! 动作配置的 JSON 解析和验证

use crate::input;
use super::types::{Action, ActionConfig, ActionConfigList, ActionItem, InputBackend};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::sync::Mutex;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

lazy_static! {
    /// 全局动作配置存储（将索引映射到 Action）
    static ref ACTION_CONFIG: Mutex<Option<ActionConfig>> = Mutex::new(None);
    /// 全局动作列表存储（保留顺序和名称）
    static ref ACTION_LIST: Mutex<Option<ActionConfigList>> = Mutex::new(None);
    /// 没有明确后端的动作的全局默认后端
    static ref DEFAULT_BACKEND: Mutex<InputBackend> = Mutex::new(InputBackend::Win32);
    /// 动作执行的目标窗口（HWND）
    pub static ref TARGET_WINDOW: Mutex<Option<isize>> = Mutex::new(None);
    /// 全局执行后端覆盖（None 表示未设置，使用默认后端）
    pub static ref EXECUTION_BACKEND: Mutex<Option<InputBackend>> = Mutex::new(None);
}

/// 最大历史记录条目数
const MAX_HISTORY_SIZE: usize = 50;

/// 撤销/重做的历史记录条目
#[derive(Clone)]
pub struct HistoryEntry {
    pub actions: ActionConfigList,
    pub default_backend: InputBackend,
}

lazy_static! {
    /// 撤销历史栈
    pub static ref UNDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// 重做历史栈
    pub static ref REDO_STACK: Mutex<Vec<HistoryEntry>> = Mutex::new(Vec::new());
    /// 用于丢弃的原始状态（加载的或上次保存的）
    pub static ref ORIGINAL_ENTRY: Mutex<Option<HistoryEntry>> = Mutex::new(None);
}

/// JSON 配置结构
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// 动作未指定时的默认输入后端
    #[serde(default = "default_win32")]
    pub default_backend: InputBackend,
    /// 动作项列表
    pub actions: Vec<ActionItemWrapper>,
}

/// 用于从 JSON 解析动作项的包装器
#[derive(Debug, Clone, Deserialize)]
pub struct ActionItemWrapper {
    /// 可选的动作名称
    #[serde(default)]
    pub name: Option<String>,
    #[serde(flatten)]
    pub action: Action,
}

fn default_win32() -> InputBackend {
    InputBackend::Win32
}

/// 从 JSON 文件加载动作配置
///
/// **JSON 格式**：
/// ```json
/// {
///   "default_backend": "win32",
///   "actions": [
///     {"index": 0, "name": "jump", "type": "key", "key": "space"},
///     {"index": 1, "type": "key", "key": "ctrl+c", "backend": "enigo"}
///   ]
/// }
/// ```
pub fn load_config(path: &str) -> Result<(ActionConfig, InputBackend), String> {
    let content = fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: Config = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse config: {}", e))?;

    let mut result: ActionConfig = HashMap::new();
    let mut action_list: ActionConfigList = Vec::new();

    for (idx, item) in config.actions.into_iter().enumerate() {
        let index = idx as u32;
        result.insert(index, item.action.clone());
        action_list.push(super::types::ActionItem {
            name: item.name,
            data: item.action,
        });
    }

    validate_config(&result)?;

    // 保存列表以供后续访问
    let mut global_list = ACTION_LIST.lock()
        .map_err(|_| "Failed to lock action list".to_string())?;
    *global_list = Some(action_list);

    Ok((result, config.default_backend))
}

/// 将配置加载到全局存储
pub fn load_config_with_backend(path: &str, _backend: InputBackend) -> Result<(), String> {
    // 后端现在在每个配置文件中指定，而不是每次调用
    let (config, default_backend) = load_config(path)?;

    let mut global_config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    let mut global_backend = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;

    *global_config = Some(config);
    *global_backend = default_backend.clone();

    // 设置原始条目用于丢弃
    let action_list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    let mut original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    *original = Some(HistoryEntry {
        actions: action_list.clone().unwrap_or_default(),
        default_backend,
    });

    Ok(())
}

/// 获取当前加载的配置（将索引映射到 Action）
pub fn get_config() -> Result<ActionConfig, String> {
    let global = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// 获取带名称的动作列表（保留顺序和名称）
pub fn get_action_list() -> Result<ActionConfigList, String> {
    let global = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// 从加载的配置获取默认后端
pub fn get_default_backend() -> Result<InputBackend, String> {
    let global = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;

    Ok(global.clone())
}

/// 设置动作执行的目标窗口
pub fn set_target_window(window: Option<String>) -> Result<(), String> {
    let hwnd = match window {
        Some(spec) => {
            let search = input::parse_window_spec(&spec);

            // 空规格表示前台窗口
            if search.title.is_none()
                && search.class_name.is_none()
                && search.hwnd.is_none()
                && search.pid.is_none()
                && search.exe_name.is_none()
            {
                let hwnd = unsafe { GetForegroundWindow() };
                if hwnd.0.is_null() {
                    return Err("No foreground window found".to_string());
                }
                hwnd.0 as isize
            } else {
                // 将窗口规格解析为 HWND
                input::find_window(&search)
                    .ok_or_else(|| format!("Window not found: {}", spec))?
            }
        }
        None => {
            // 清除目标窗口
            let mut global = TARGET_WINDOW
                .lock()
                .map_err(|_| "Failed to lock target window".to_string())?;
            *global = None;
            return Ok(());
        }
    };

    let mut global = TARGET_WINDOW
        .lock()
        .map_err(|_| "Failed to lock target window".to_string())?;
    *global = Some(hwnd);
    Ok(())
}

/// 获取当前目标窗口（HWND）
pub fn get_target_window() -> Option<isize> {
    TARGET_WINDOW
        .lock()
        .map(|g| g.clone())
        .unwrap_or(None)
}

/// 设置执行后端覆盖
pub fn set_execution_backend(backend: InputBackend) -> Result<(), String> {
    let mut global = EXECUTION_BACKEND
        .lock()
        .map_err(|_| "Failed to lock execution backend".to_string())?;
    *global = Some(backend);
    Ok(())
}

/// 获取当前执行后端覆盖（None 表示未设置）
pub fn get_execution_backend() -> Option<InputBackend> {
    EXECUTION_BACKEND
        .lock()
        .map(|g| g.clone())
        .unwrap_or(None)
}

/// 设置默认后端（带历史跟踪）
pub fn set_default_backend(backend: InputBackend) -> Result<(), String> {
    // 修改前保存当前状态到历史记录
    save_to_history()?;

    let mut global = DEFAULT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;
    *global = backend;
    Ok(())
}

/// 验证单个动作
fn validate_action(action: &Action) -> Result<(), String> {
    match action {
        Action::Key(key_action) => {
            if key_action.key.is_empty() {
                return Err("Key name cannot be empty".to_string());
            }
        }
        Action::KeySequence(seq_action) => {
            if seq_action.keys.is_empty() {
                return Err("Key sequence cannot be empty".to_string());
            }
            for item in &seq_action.keys {
                if item.key.is_empty() {
                    return Err("Key in sequence cannot be empty".to_string());
                }
            }
        }
        Action::MouseClick(click_action) => {
            if click_action.count == 0 {
                return Err("Click count must be at least 1".to_string());
            }
        }
        Action::MouseMove(_) => {
            // x, y 可以是任意值 - 无需验证
        }
        Action::MouseScroll(scroll_action) => {
            if scroll_action.amount == 0 {
                return Err("Scroll amount must not be zero".to_string());
            }
        }
        Action::Delay(_delay_action) => {
            // duration 可以是任意值 - 允许 0 表示无操作
        }
        Action::Text(text_action) => {
            if text_action.content.is_empty() {
                return Err("Text content cannot be empty".to_string());
            }
        }
    }
    Ok(())
}

/// 验证整个配置
pub fn validate_config(config: &ActionConfig) -> Result<(), String> {
    if config.is_empty() {
        return Err("Configuration is empty".to_string());
    }

    for (id, action) in config {
        validate_action(action).map_err(|e| format!("Action {}: {}", id, e))?;
    }

    Ok(())
}

// 保存当前状态到撤销历史
pub fn save_to_history() -> Result<(), String> {
    let actions = get_action_list()?;
    let backend = get_default_backend()?;

    let entry = HistoryEntry {
        actions,
        default_backend: backend,
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

/// 获取下一个可用动作索引（列表长度，如果为空则为 0）
pub fn get_next_available_index() -> Result<u32, String> {
    let list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    Ok(list.as_ref().map(|items| items.len()).unwrap_or(0) as u32)
}

/// 从当前动作列表重建 HashMap 以确保索引与位置匹配
fn rebuild_config_from_list() -> Result<(), String> {
    let list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    let new_config: ActionConfig = list
        .as_ref()
        .map(|items| {
            items
                .iter()
                .enumerate()
                .map(|(idx, item)| (idx as u32, item.data.clone()))
                .collect()
        })
        .unwrap_or_default();

    let mut config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;
    *config = Some(new_config);

    Ok(())
}

/// 在内存中创建新动作并返回其分配的索引
pub fn create_action(action: Action, name: Option<String>) -> Result<u32, String> {
    // 修改前保存当前状态到历史记录
    save_to_history()?;

    // 首先验证动作
    validate_action(&action)?;

    let mut list = ACTION_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    // 索引是列表的长度（追加到末尾）
    let index = list.as_ref().map(|items| items.len()).unwrap_or(0) as u32;

    let item = ActionItem {
        name,
        data: action.clone(),
    };

    // 确保列表已初始化
    if list.is_none() {
        *list = Some(Vec::new());
    }

    if let Some(ref mut items) = *list {
        items.push(item);
    }

    // 同时更新 HashMap
    let mut config = ACTION_CONFIG
        .lock()
        .map_err(|_| "Failed to lock configuration".to_string())?;
    if let Some(ref mut map) = *config {
        map.insert(index, action);
    } else {
        let mut map = HashMap::new();
        map.insert(index, action);
        *config = Some(map);
    }

    Ok(index)
}

/// 按索引更新现有动作
pub fn update_action(index: u32, action: Action, name: Option<String>) -> Result<(), String> {
    // 修改前保存当前状态到历史记录
    save_to_history()?;

    // 首先验证动作
    validate_action(&action)?;

    // 在 ACTION_LIST 中更新
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            if let Some(item) = items.get_mut(index as usize) {
                item.data = action.clone();
                item.name = name;
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found {
            return Err(format!("Action {} not found", index));
        }
    }

    // 在 ACTION_CONFIG HashMap 中更新
    {
        let mut config = ACTION_CONFIG
            .lock()
            .map_err(|_| "Failed to lock configuration".to_string())?;

        if let Some(ref mut map) = *config {
            if let Some(existing) = map.get_mut(&index) {
                *existing = action;
            } else {
                return Err(format!("Action {} not found in config map", index));
            }
        } else {
            return Err("No configuration loaded".to_string());
        }
    }

    Ok(())
}

/// 按索引删除动作
pub fn delete_action(index: u32) -> Result<(), String> {
    // 修改前保存当前状态到历史记录
    save_to_history()?;

    // 从 ACTION_LIST 中删除
    {
        let mut list = ACTION_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;

        let found = if let Some(ref mut items) = *list {
            if (index as usize) < items.len() {
                items.remove(index as usize);
                true
            } else {
                false
            }
        } else {
            false
        };

        if !found {
            return Err(format!("Action {} not found", index));
        }
    }

    // 从列表重建 HashMap 以维护索引到位置的对应关系
    rebuild_config_from_list()?;

    Ok(())
}

/// 将配置保存到 JSON 文件
pub fn save_config(path: &str, default_backend: InputBackend, actions: &ActionConfigList) -> Result<(), String> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct SaveConfig<'a> {
        default_backend: InputBackend,
        actions: &'a ActionConfigList,
    }

    let config = SaveConfig {
        default_backend: default_backend.clone(),
        actions,
    };

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json)
        .map_err(|e| format!("Failed to write config file: {}", e))?;

    // 将保存的配置重新加载到全局状态
    load_config_with_backend(path, default_backend)?;

    Ok(())
}

// 撤销 - 恢复上一个状态
pub fn undo() -> Result<(), String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if undo.is_empty() {
        return Err("Nothing to undo".to_string());
    }

    // 保存当前状态到重做栈
    let current = HistoryEntry {
        actions: get_action_list()?,
        default_backend: get_default_backend()?,
    };
    redo.push(current);

    // 恢复上一个状态
    let prev = undo.pop().unwrap();
    reload_from_entry(&prev)?;

    Ok(())
}

// 重做 - 恢复下一个状态
pub fn redo() -> Result<(), String> {
    let mut undo = UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?;
    let mut redo = REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?;

    if redo.is_empty() {
        return Err("Nothing to redo".to_string());
    }

    // 保存当前状态到撤销栈
    let current = HistoryEntry {
        actions: get_action_list()?,
        default_backend: get_default_backend()?,
    };
    undo.push(current);

    // 恢复下一个状态
    let next = redo.pop().unwrap();
    reload_from_entry(&next)?;

    Ok(())
}

// 丢弃所有更改 - 恢复到原始状态（可撤销）
pub fn discard_changes() -> Result<(), String> {
    // 保存当前状态到历史记录（以便我们可以撤销）
    save_to_history()?;

    // 从原始状态恢复
    let original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    let entry = original
        .clone()
        .ok_or_else(|| "No original state to discard to".to_string())?;
    drop(original);

    reload_from_entry(&entry)?;

    Ok(())
}

// 丢弃特定动作 - 恢复到原始状态（可撤销）
pub fn discard_action(index: u32) -> Result<(), String> {
    // 保存当前状态到历史记录（以便我们可以撤销）
    save_to_history()?;

    // 获取此索引处的原始动作
    let original = ORIGINAL_ENTRY.lock().map_err(|_| "Failed to lock original")?;
    let entry = original
        .clone()
        .ok_or_else(|| "No original state to discard to".to_string())?;

    let original_action = entry
        .actions
        .get(index as usize)
        .ok_or_else(|| format!("No action at index {}", index))?;
    drop(original);

    // 在全局状态中更新特定动作
    let mut config = ACTION_CONFIG.lock().map_err(|_| "Failed to lock config")?;
    if let Some(ref mut c) = *config {
        c.insert(index, original_action.data.clone());
    }

    let mut list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    if let Some(ref mut l) = *list {
        l[index as usize] = original_action.clone();
    }

    Ok(())
}

// 从历史条目重新加载状态
fn reload_from_entry(entry: &HistoryEntry) -> Result<(), String> {
    // 使用数组位置作为索引更新全局 ACTION_CONFIG HashMap
    let mut config = ACTION_CONFIG.lock().map_err(|_| "Failed to lock config")?;
    *config = Some(
        entry
            .actions
            .iter()
            .enumerate()
            .map(|(idx, a)| (idx as u32, a.data.clone()))
            .collect(),
    );

    // 更新全局 ACTION_LIST
    let mut list = ACTION_LIST.lock().map_err(|_| "Failed to lock action list")?;
    *list = Some(entry.actions.clone());

    // 更新全局 DEFAULT_BACKEND
    let mut backend = DEFAULT_BACKEND.lock().map_err(|_| "Failed to lock default backend")?;
    *backend = entry.default_backend.clone();

    Ok(())
}

// 清除撤销/重做历史
pub fn clear_history() -> Result<(), String> {
    UNDO_STACK.lock().map_err(|_| "Failed to lock undo stack")?.clear();
    REDO_STACK.lock().map_err(|_| "Failed to lock redo stack")?.clear();
    Ok(())
}

// 获取历史状态（撤销计数，重做计数）
pub fn get_history_status() -> (usize, usize) {
    let undo_len = UNDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    let redo_len = REDO_STACK.lock().map(|g| g.len()).unwrap_or(0);
    (undo_len, redo_len)
}
