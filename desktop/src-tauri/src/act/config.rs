//! 动作配置的 JSON 解析和验证

use super::history;
use super::state;
use super::types::{Action, ActionData, ActionList, InputBackend};
use crate::input;
use serde::Deserialize;
use std::fs;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

// history 函数已改为纯函数，config 负责应用状态
pub use super::validation::{validate_action, validate_config};

/// JSON 配置结构（与 ObserveConfig 区分，命名更明确）
#[derive(Debug, Clone, Deserialize)]
pub struct ActionConfigJson {
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
    pub action: ActionData,
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
///     { "name": "按空格", "type": "key", "key": "space" },
///     { "type": "key_sequence", "keys": [{"key": "ctrl"}, {"key": "a"}], "default_interval_ms": 5 },
///     { "type": "mouse_click", "button": "left", "count": 2 },
///     { "type": "mouse_move", "x": 100, "y": 200 },
///     { "type": "mouse_scroll", "direction": "up", "amount": 3 },
///     { "type": "delay", "duration_ms": 1000 },
///     { "type": "text", "content": "Hello World" }
///   ]
/// }
/// ```
pub fn load_config(path: &str) -> Result<(ActionList, InputBackend), String> {
    if !std::path::Path::new(path).exists() {
        let action_list: ActionList = Vec::new();
        let mut global_list = state::ACTION_CONFIG_LIST
            .lock()
            .map_err(|_| "Failed to lock action list".to_string())?;
        *global_list = Some(action_list.clone());
        let mut global_backend = state::DEFAULT_INPUT_BACKEND
            .lock()
            .map_err(|_| "Failed to lock default backend".to_string())?;
        *global_backend = InputBackend::Win32;
        return Ok((action_list, InputBackend::Win32));
    }
    let content =
        fs::read_to_string(path).map_err(|e| format!("Failed to read config file: {}", e))?;

    let config: ActionConfigJson =
        serde_json::from_str(&content).map_err(|e| format!("Failed to parse config: {}", e))?;

    let mut action_list: ActionList = Vec::new();

    for item in config.actions.into_iter() {
        action_list.push(Action {
            name: item.name,
            data: item.action,
        });
    }

    validate_config(&action_list)?;

    // 保存列表以供后续访问
    let mut global_list = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;
    *global_list = Some(action_list.clone());

    Ok((action_list, config.default_backend))
}

/// 将配置加载到全局存储
pub fn load_config_with_backend(path: &str, _backend: InputBackend) -> Result<(), String> {
    // 后端现在在每个配置文件中指定，而不是每次调用
    let (action_list, default_backend) = load_config(path)?;

    let mut global_backend = state::DEFAULT_INPUT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;

    *global_backend = default_backend.clone();

    // 设置原始条目用于丢弃
    history::set_baseline_snapshot(super::types::HistoryEntry {
        actions: action_list,
        default_backend,
    })?;

    Ok(())
}

/// 获取带名称的动作列表（保留顺序和名称）
pub fn get_action_list() -> Result<ActionList, String> {
    let global = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    global
        .clone()
        .ok_or_else(|| "No configuration loaded. Call load_action_config first.".to_string())
}

/// 从加载的配置获取默认后端
pub fn get_default_backend() -> Result<InputBackend, String> {
    let global = state::DEFAULT_INPUT_BACKEND
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
                && search.process_name.is_none()
            {
                let hwnd = unsafe { GetForegroundWindow() };
                if hwnd.0.is_null() {
                    return Err("No foreground window found".to_string());
                }
                hwnd.0 as isize
            } else {
                // 将窗口规格解析为 HWND
                input::find_window(&search).ok_or_else(|| format!("Window not found: {}", spec))?
            }
        }
        None => {
            // 清除目标窗口
            let mut global = state::TARGET_WINDOW
                .lock()
                .map_err(|_| "Failed to lock target window".to_string())?;
            *global = None;
            return Ok(());
        }
    };

    let mut global = state::TARGET_WINDOW
        .lock()
        .map_err(|_| "Failed to lock target window".to_string())?;
    *global = Some(hwnd);
    Ok(())
}

/// 获取当前目标窗口（HWND）
pub fn get_target_window() -> Option<isize> {
    state::TARGET_WINDOW.lock().map(|g| *g).unwrap_or(None)
}

/// 设置默认后端（带历史跟踪）
pub fn set_default_backend(backend: InputBackend) -> Result<(), String> {
    // 获取当前状态用于历史记录
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;

    // 修改前保存当前状态到历史记录
    history::save_to_history(actions, current_backend)?;

    let mut global = state::DEFAULT_INPUT_BACKEND
        .lock()
        .map_err(|_| "Failed to lock default backend".to_string())?;
    *global = backend;
    Ok(())
}

/// 获取下一个可用动作索引（列表长度，如果为空则为 0）
pub fn get_next_available_index() -> Result<u32, String> {
    let list = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    Ok(list.as_ref().map(|items| items.len()).unwrap_or(0) as u32)
}

/// 在内存中创建新动作并返回其分配的索引
pub fn create_action(action: ActionData, name: Option<String>) -> Result<u32, String> {
    // 获取当前状态用于历史记录
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;

    // 修改前保存当前状态到历史记录
    history::save_to_history(actions.clone(), current_backend)?;

    // 首先验证动作
    validate_action(&action)?;

    let mut list = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    // 索引是列表的长度（追加到末尾）
    let index = list.as_ref().map(|items| items.len()).unwrap_or(0) as u32;

    let item = Action { name, data: action };

    // 确保列表已初始化
    if list.is_none() {
        *list = Some(Vec::new());
    }

    if let Some(ref mut items) = *list {
        items.push(item);
    }

    Ok(index)
}

/// 按索引更新现有动作
pub fn update_action(index: u32, action: ActionData, name: Option<String>) -> Result<(), String> {
    // 获取当前状态用于历史记录
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;

    // 修改前保存当前状态到历史记录
    history::save_to_history(actions, current_backend)?;

    // 首先验证动作
    validate_action(&action)?;

    let mut list = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    if let Some(ref mut items) = *list {
        if let Some(item) = items.get_mut(index as usize) {
            item.data = action;
            item.name = name;
            Ok(())
        } else {
            Err(format!("Action {} not found", index))
        }
    } else {
        Err("No action list loaded".to_string())
    }
}

/// 按索引删除动作
pub fn delete_action(index: u32) -> Result<(), String> {
    // 获取当前状态用于历史记录
    let actions = get_action_list()?;
    let current_backend = get_default_backend()?;

    // 修改前保存当前状态到历史记录
    history::save_to_history(actions, current_backend)?;

    let mut list = state::ACTION_CONFIG_LIST
        .lock()
        .map_err(|_| "Failed to lock action list".to_string())?;

    if let Some(ref mut items) = *list {
        if (index as usize) < items.len() {
            items.remove(index as usize);
            Ok(())
        } else {
            Err(format!("Action {} not found", index))
        }
    } else {
        Err("No action list loaded".to_string())
    }
}

/// 将配置保存到 JSON 文件
pub fn save_config(
    path: &str,
    default_backend: InputBackend,
    actions: &ActionList,
) -> Result<(), String> {
    use serde::Serialize;

    #[derive(Serialize)]
    struct SaveConfig<'a> {
        default_backend: InputBackend,
        actions: &'a ActionList,
    }

    let config = SaveConfig {
        default_backend: default_backend.clone(),
        actions,
    };

    let json = serde_json::to_string_pretty(&config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;

    fs::write(path, json).map_err(|e| format!("Failed to write config file: {}", e))?;

    // 将保存的配置重新加载到全局状态
    load_config_with_backend(path, default_backend)?;

    Ok(())
}
