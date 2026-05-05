//! OTAcle 的动作执行逻辑
//!
//! 根据动作 ID 执行配置的动作。

use super::config;
use super::types::{
    ActionData, DelayAction, InputBackend, KeyAction, KeySequenceAction, MouseButton,
    MouseClickAction, MouseMoveAction, MouseScrollAction, ScrollDirection, TextAction, Variable,
};
use crate::input;
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Mouse, Settings};
use log::{error, info};
use std::collections::HashMap;

/// 按键事件之间的默认间隔（毫秒）
const DEFAULT_KEY_INTERVAL_MS: u64 = 2;

/// 要由输入抑制剂处理的按键事件
#[derive(Debug, Clone)]
struct KeyEvent<'a> {
    key: &'a str,
    direction: KeyDirection,
    /// 此事件之前的延迟（毫秒）
    delay_ms: u64,
}

#[derive(Debug, Clone, Copy)]
enum KeyDirection {
    Press,
    Release,
}

impl KeyDirection {
    fn as_str(&self) -> &'static str {
        match self {
            KeyDirection::Press => "press",
            KeyDirection::Release => "release",
        }
    }
}

/// 执行上下文 - 后端和目标窗口
struct ExecContext {
    backend: InputBackend,
    target_hwnd: Option<isize>,
}

impl ExecContext {
    fn new(backend: InputBackend, target_hwnd: Option<isize>) -> Self {
        Self {
            backend,
            target_hwnd,
        }
    }
}

/// 获取动作的有效后端
/// 优先级：action.backend > default_backend
fn resolve_backend(action: &ActionData, default_backend: InputBackend) -> InputBackend {
    // First priority: action's own backend setting
    if let Some(backend) = get_action_backend(action) {
        return backend;
    }

    // Second priority: config default_backend
    default_backend
}

/// 如果指定了则从动作获取后端
fn get_action_backend(action: &ActionData) -> Option<InputBackend> {
    match action {
        ActionData::Key(a) => a.backend.clone(),
        ActionData::KeySequence(a) => a.backend.clone(),
        ActionData::MouseClick(a) => a.backend.clone(),
        ActionData::MouseMove(a) => a.backend.clone(),
        ActionData::MouseScroll(a) => a.backend.clone(),
        ActionData::Delay(_) => None, // 延迟不使用后端
        ActionData::Text(a) => a.backend.clone(),
    }
}

/// 使用指定后端按正确时序执行一系列按键事件
fn execute_key_events(events: &[KeyEvent], ctx: &ExecContext) -> Result<(), String> {
    let mut first = true;
    for event in events {
        if !first && event.delay_ms > 0 {
            std::thread::sleep(std::time::Duration::from_millis(event.delay_ms));
        }
        first = false;
        send_key_event(event.key, event.direction.as_str(), ctx)?;
    }
    Ok(())
}

/// 使用配置的后端发送按键事件
fn send_key_event(key: &str, direction: &str, ctx: &ExecContext) -> Result<(), String> {
    match ctx.backend {
        InputBackend::Enigo => {
            // Enigo 模式：先激活窗口，然后发送
            if let Some(hwnd) = ctx.target_hwnd {
                input::activate_window(hwnd)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            input::enigo::send_key(key, direction)
        }
        InputBackend::Win32 => {
            // Win32 模式：直接发送到目标窗口
            let hwnd = ctx
                .target_hwnd
                .ok_or("Win32 backend requires target window")?;
            input::send_key_to_window(hwnd, key, direction)
        }
    }
}

/// 使用配置的后端发送文本
fn send_text_event(text: &str, ctx: &ExecContext) -> Result<(), String> {
    match ctx.backend {
        InputBackend::Enigo => {
            if let Some(hwnd) = ctx.target_hwnd {
                input::activate_window(hwnd)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            input::enigo::send_text(text)
        }
        InputBackend::Win32 => {
            let hwnd = ctx
                .target_hwnd
                .ok_or("Win32 backend requires target window")?;
            input::send_text_to_window(hwnd, text)
        }
    }
}

/// 按索引执行动作
///
/// # 参数
/// * `action_idx` - 要执行的动作的数组索引
/// * `default_backend` - 加载的配置中的默认后端（当动作没有后端时使用）
///
/// 目标窗口和执行后端从全局配置读取。
pub fn execute_action(action_idx: u32, default_backend: InputBackend) -> Result<(), String> {
    // 获取配置
    let actions = config::get_action_list()?;

    let action_item = actions
        .get(action_idx as usize)
        .ok_or_else(|| format!("Action {} not found in configuration", action_idx))?;
    let action = &action_item.data;

    // 解析后端：action.backend > default_backend
    let resolved_backend = resolve_backend(action, default_backend);

    // 获取全局目标窗口（已经解析为 HWND）
    let target_hwnd = config::get_target_window();

    // 如果使用 Enigo 后端则激活窗口（Win32 不需要激活）
    if resolved_backend == InputBackend::Enigo {
        if let Some(hwnd) = target_hwnd {
            input::activate_window(hwnd)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    let ctx = ExecContext::new(resolved_backend, target_hwnd);

    // 执行动作
    match execute_action_impl(action, &ctx) {
        Ok(()) => {
            info!("Action {} executed successfully", action_idx);
            Ok(())
        }
        Err(e) => {
            error!("Action {} failed: {}", action_idx, e);
            Err(e)
        }
    }
}

/// 使用可选参数按索引执行动作以进行变量替换
///
/// # 参数
/// * `action_idx` - 要执行的动作的数组索引
/// * `params` - 用于动态字段替换的可选参数
/// * `default_backend` - 加载的配置中的默认后端
///
/// 目标窗口和执行后端从全局配置读取。
pub fn execute_action_with_params(
    action_idx: u32,
    params: HashMap<String, serde_json::Value>,
    default_backend: InputBackend,
) -> Result<(), String> {
    // 获取配置
    let actions = config::get_action_list()?;

    let action_item = actions
        .get(action_idx as usize)
        .ok_or_else(|| format!("Action {} not found in configuration", action_idx))?;

    // 如果提供了则应用动态参数
    let resolved_action = if !params.is_empty() {
        apply_params(&action_item.data, &params)?
    } else {
        action_item.data.clone()
    };

    // 解析后端：action.backend > default_backend
    let resolved_backend = resolve_backend(&resolved_action, default_backend);

    // 获取全局目标窗口（已经解析为 HWND）
    let target_hwnd = config::get_target_window();

    // 如果使用 Enigo 后端则激活窗口（Win32 不需要激活）
    if resolved_backend == InputBackend::Enigo {
        if let Some(hwnd) = target_hwnd {
            input::activate_window(hwnd)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    let ctx = ExecContext::new(resolved_backend, target_hwnd);

    // 执行动作
    execute_action_impl(&resolved_action, &ctx)
}

/// 根据执行向量执行多个动作
///
/// # 参数
/// * `execute` - 布尔向量，其中索引对应动作索引
/// * `params` - 用于动态字段替换的可选参数
/// * `default_backend` - 加载的配置中的默认后端
///
/// 如果所有执行的动作都成功则返回 Ok(()), 否则返回第一个错误
pub fn execute_actions(
    execute: Vec<bool>,
    params: HashMap<String, serde_json::Value>,
    default_backend: InputBackend,
) -> Result<(), String> {
    let actions = config::get_action_list()?;

    // 获取全局目标窗口（已经解析为 HWND）
    let target_hwnd = config::get_target_window();

    // 先收集所有需要执行的 action 索引
    let actions_to_execute: Vec<u32> = execute
        .iter()
        .enumerate()
        .filter(|(_, &should_exec)| should_exec)
        .filter_map(|(idx, _)| {
            if idx < actions.len() {
                Some(idx as u32)
            } else {
                None
            }
        })
        .collect();

    let total = actions_to_execute.len();
    for &action_idx in &actions_to_execute {
        let action = &actions
            .get(action_idx as usize)
            .expect("action not found")
            .data;
        // 如果提供了则应用动态参数
        let resolved_action = if !params.is_empty() {
            apply_params(action, &params)?
        } else {
            action.clone()
        };

        // 解析后端：action.backend > default_backend
        let resolved_backend = resolve_backend(&resolved_action, default_backend.clone());

        // 使用全局目标窗口创建执行上下文
        let ctx = ExecContext::new(resolved_backend, target_hwnd);

        // 执行
        if let Err(e) = execute_action_impl(&resolved_action, &ctx) {
            error!("Batch action failed: {}", e);
            return Err(e);
        }
    }

    info!("Batch actions executed: {} actions", total);
    Ok(())
}

/// 根据变量的定义将动态参数应用到动作
///
/// 每个 Variable 条目将 param_name（来自 ZMQ params）映射到 field_name（动作结构字段）。
/// 只有在 variables 中明确列出的字段才能在运行时覆盖。
fn apply_params(
    action: &ActionData,
    params: &HashMap<String, serde_json::Value>,
) -> Result<ActionData, String> {
    // 将 ActionData 序列化为 JSON
    let json = serde_json::to_value(action)
        .map_err(|e| format!("Failed to serialize action: {}", e))?;

    // 确保是对象类型
    let mut map = match json {
        serde_json::Value::Object(m) => m,
        _ => return Err("Action serialization produced non-object".to_string()),
    };

    // 根据动作类型应用变量覆盖
    let new_json = match action {
        ActionData::Key(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::KeySequence(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::MouseClick(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::MouseMove(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::MouseScroll(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::Delay(a) => apply_variables(&mut map, &a.variables, params)?,
        ActionData::Text(a) => apply_variables(&mut map, &a.variables, params)?,
    };

    // 反序列化回 ActionData
    serde_json::from_value(new_json)
        .map_err(|e| format!("Failed to deserialize action: {}", e))
}

/// 根据 variables 将 params 应用到 JSON map
fn apply_variables(
    map: &mut serde_json::Map<String, serde_json::Value>,
    variables: &[Variable],
    params: &HashMap<String, serde_json::Value>,
) -> Result<serde_json::Value, String> {
    for var in variables {
        if let Some(value) = params.get(&var.param_name) {
            if let Some(existing) = map.get(&var.field_name) {
                // 类型检查
                if existing.is_number() && !value.is_number() {
                    return Err(format!(
                        "Type mismatch for '{}': expected number, got {}",
                        var.field_name, value
                    ));
                }
                if existing.is_string() && !value.is_string() && !value.is_null() {
                    return Err(format!(
                        "Type mismatch for '{}': expected string, got {}",
                        var.field_name, value
                    ));
                }
                map.insert(var.field_name.clone(), value.clone());
            } else {
                return Err(format!(
                    "Unknown field '{}' for this action type (available fields: {:?})",
                    var.field_name,
                    map.keys().collect::<Vec<_>>()
                ));
            }
        }
        // 如果未提供参数，使用配置中的默认值（不执行任何操作）
    }
    Ok(serde_json::Value::Object(map.clone()))
}

/// 执行动作实现
fn execute_action_impl(action: &ActionData, ctx: &ExecContext) -> Result<(), String> {
    match action {
        ActionData::Key(key_action) => execute_key(key_action, ctx),
        ActionData::KeySequence(seq_action) => execute_key_sequence(seq_action, ctx),
        ActionData::MouseClick(click_action) => execute_mouse_click(click_action, ctx),
        ActionData::MouseMove(move_action) => execute_mouse_move(move_action, ctx),
        ActionData::MouseScroll(scroll_action) => execute_mouse_scroll(scroll_action, ctx),
        ActionData::Delay(delay_action) => execute_delay(delay_action),
        ActionData::Text(text_action) => execute_text(text_action, ctx),
    }
}

/// 执行按键动作
///
/// 支持以 `+` 分隔符组合键，例如 "ctrl+c"、"ctrl+shift+a"
/// 对于组合键：
///   1. 按下所有修饰符键
///   2. 按下主键
///   3. 释放所有键
fn execute_key(action: &KeyAction, ctx: &ExecContext) -> Result<(), String> {
    let key = &action.key;

    // 检查是否是组合键（包含 '+'）
    if key.contains('+') {
        execute_combination_key(key, action.hold_time_ms, ctx)?;
    } else {
        if action.hold_time_ms > 0 {
            // 按住
            send_key_event(key, "press", ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(action.hold_time_ms));
            send_key_event(key, "release", ctx)?;
        } else {
            // 简单点击
            send_key_event(key, "click", ctx)?;
        }
    }
    Ok(())
}

/// 执行组合键（例如 "ctrl+c"、"ctrl+shift+a"）
///
/// 对于 Win32 后端：使用 SendInput 原子地发送所有键
/// 对于 Enigo 后端：构建具有正确时序的事件队列
fn execute_combination_key(key: &str, hold_ms: u64, ctx: &ExecContext) -> Result<(), String> {
    let keys: Vec<&str> = key.split('+').map(|s| s.trim()).collect();

    if keys.is_empty() {
        return Err("Invalid combination key".to_string());
    }

    match ctx.backend {
        InputBackend::Win32 => {
            // Win32：使用 SendInput 进行原子键序列
            let mut key_pairs: Vec<(&str, bool)> = Vec::new();

            // 按下所有键
            for k in &keys {
                key_pairs.push((k, true));
            }

            // 如果指定了则保持
            if hold_ms > 0 {
                // 首先释放主键
                key_pairs.push((keys.last().unwrap(), false));
                // 然后按相反顺序释放修饰符键（跳过主键）
                for k in keys.iter().rev().skip(1) {
                    key_pairs.push((k, false));
                }
            } else {
                // 按相反顺序释放所有键
                for k in keys.iter().rev() {
                    key_pairs.push((k, false));
                }
            }

            // 首先激活窗口
            if let Some(hwnd) = ctx.target_hwnd {
                input::activate_window(hwnd)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            input::send_key_sequence(&key_pairs)
        }
        InputBackend::Enigo => {
            // Enigo：使用现有的基于事件的方法
            let mut events: Vec<KeyEvent> = Vec::new();

            // 按下每个键之前带有延迟
            for k in &keys {
                events.push(KeyEvent {
                    key: k,
                    direction: KeyDirection::Press,
                    delay_ms: DEFAULT_KEY_INTERVAL_MS,
                });
            }

            // 如果指定了则保持
            if hold_ms > 0 {
                // 保持后首先释放主键
                events.push(KeyEvent {
                    key: keys.last().unwrap(), // 主键
                    direction: KeyDirection::Release,
                    delay_ms: hold_ms,
                });
                // 然后按相反顺序释放所有修饰符键（跳过主键）
                for k in keys.iter().rev().skip(1) {
                    events.push(KeyEvent {
                        key: k,
                        direction: KeyDirection::Release,
                        delay_ms: DEFAULT_KEY_INTERVAL_MS,
                    });
                }
            } else {
                // 按相反顺序释放所有键
                for k in keys.iter().rev() {
                    events.push(KeyEvent {
                        key: k,
                        direction: KeyDirection::Release,
                        delay_ms: DEFAULT_KEY_INTERVAL_MS,
                    });
                }
            }

            execute_key_events(&events, ctx)
        }
    }
}

/// 执行按键序列动作
fn execute_key_sequence(action: &KeySequenceAction, ctx: &ExecContext) -> Result<(), String> {
    let default_interval = action.default_interval_ms;

    for (i, item) in action.keys.iter().enumerate() {
        // 执行按键
        if item.hold_time_ms > 0 {
            // 按住
            send_key_event(&item.key, "press", ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(item.hold_time_ms));
            send_key_event(&item.key, "release", ctx)?;
        } else {
            // 简单点击
            send_key_event(&item.key, "click", ctx)?;
        }

        // 在下一个键之前休眠（除非这是最后一个键）
        if i < action.keys.len() - 1 {
            let interval = item.interval_ms.unwrap_or(default_interval);
            if interval > 0 {
                std::thread::sleep(std::time::Duration::from_millis(interval));
            }
        }
    }

    Ok(())
}

/// 执行鼠标点击动作
fn execute_mouse_click(action: &MouseClickAction, ctx: &ExecContext) -> Result<(), String> {
    let count = action.count;
    let button = action.button;
    let interval = action.interval_ms.unwrap_or(0);
    let hold_time = action.hold_time_ms;

    match ctx.backend {
        InputBackend::Win32 => {
            let hwnd = ctx
                .target_hwnd
                .ok_or("Win32 backend requires target window for mouse click")?;
            for i in 0..count {
                input::win32_input::send_mouse_click(hwnd, 0, 0, button)?;
                if hold_time > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(hold_time));
                }
                if i < count - 1 && interval > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(interval));
                }
            }
            Ok(())
        }
        InputBackend::Enigo => {
            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

            let btn = match button {
                MouseButton::Left => Button::Left,
                MouseButton::Right => Button::Right,
                MouseButton::Middle => Button::Middle,
            };

            for i in 0..count {
                enigo
                    .button(btn, Direction::Press)
                    .map_err(|e| format!("Failed to press mouse button: {:?}", e))?;

                if hold_time > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(hold_time));
                }

                enigo
                    .button(btn, Direction::Release)
                    .map_err(|e| format!("Failed to release mouse button: {:?}", e))?;

                if i < count - 1 && interval > 0 {
                    std::thread::sleep(std::time::Duration::from_millis(interval));
                }
            }
            Ok(())
        }
    }
}

/// 执行鼠标移动动作
fn execute_mouse_move(action: &MouseMoveAction, ctx: &ExecContext) -> Result<(), String> {
    let x = action.x;
    let y = action.y;

    let duration = action.duration_ms.unwrap_or(0);

    match ctx.backend {
        InputBackend::Win32 => {
            // Win32：使用 SetCursorPos
            let start = input::get_mouse_position()?;
            input::smooth_move(start.0, start.1, x, y, duration)?;
        }
        InputBackend::Enigo => {
            if duration == 0 {
                // 使用 enigo 瞬间移动
                let mut enigo = Enigo::new(&Settings::default())
                    .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;
                enigo
                    .move_mouse(x, y, Coordinate::Abs)
                    .map_err(|e| format!("Failed to move mouse: {:?}", e))?;
            } else {
                // 平滑移动：获取当前位置然后插值
                let start = input::get_mouse_position()?;
                input::smooth_move(start.0, start.1, x, y, duration)?;
            }
        }
    }

    Ok(())
}

/// 执行鼠标滚动动作
fn execute_mouse_scroll(action: &MouseScrollAction, ctx: &ExecContext) -> Result<(), String> {
    // Windows 默认滚轮增量是每个"点击" 120
    let delta = (action.amount as i32) * 120;

    match ctx.backend {
        InputBackend::Win32 => {
            let hwnd = ctx
                .target_hwnd
                .ok_or("Win32 backend requires target window for mouse scroll")?;
            input::win32_input::send_mouse_scroll(hwnd, delta)?;
        }
        InputBackend::Enigo => {
            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

            // Enigo 滚动：length（正=下/右，负=上/左），axis
            let (length, axis) = match action.direction {
                ScrollDirection::Up => (-(action.amount as i32), Axis::Vertical),
                ScrollDirection::Down => (action.amount as i32, Axis::Vertical),
                ScrollDirection::Left => (-(action.amount as i32), Axis::Horizontal),
                ScrollDirection::Right => (action.amount as i32, Axis::Horizontal),
            };

            enigo
                .scroll(length, axis)
                .map_err(|e| format!("Failed to scroll: {:?}", e))?;
        }
    }

    Ok(())
}

/// 执行延迟动作
fn execute_delay(action: &DelayAction) -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_millis(action.duration_ms));
    Ok(())
}

/// 执行文本输入动作
fn execute_text(action: &TextAction, ctx: &ExecContext) -> Result<(), String> {
    send_text_event(&action.content, ctx)
}
