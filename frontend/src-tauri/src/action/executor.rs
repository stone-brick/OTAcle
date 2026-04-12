//! Action execution logic for OTAcle
//!
//! Executes configured actions based on action IDs.

use super::config;
use super::resolver::ActionResolver;
use super::types::{Action, DelayAction, InputBackend, KeyAction, KeySequenceAction, MouseButton, MouseClickAction, MouseMoveAction, MouseScrollAction, ParamValue, ScrollDirection, TextAction};
use crate::input;
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Mouse, Settings};
use std::collections::HashMap;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

/// Default interval between key events (milliseconds)
const DEFAULT_KEY_INTERVAL_MS: u64 = 2;

/// A key event to be processed by the input inhibitor
#[derive(Debug, Clone)]
struct KeyEvent<'a> {
    key: &'a str,
    direction: KeyDirection,
    /// Delay before this event (milliseconds)
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

/// Execution context - backend and target window
struct ExecContext {
    backend: InputBackend,
    target_hwnd: Option<isize>,
}

impl ExecContext {
    fn new(backend: InputBackend, target_hwnd: Option<isize>) -> Self {
        Self { backend, target_hwnd }
    }
}

/// Get the effective backend for an action
/// Priority: action.backend > default_backend > call_parameter
fn resolve_backend(action: &Action, default_backend: InputBackend, call_backend: Option<&str>) -> InputBackend {
    // First priority: action's own backend setting
    if let Some(backend) = get_action_backend(action) {
        return backend;
    }

    // Second priority: call parameter (if provided)
    if let Some(param) = call_backend {
        return match param {
            "enigo" => InputBackend::Enigo,
            _ => InputBackend::Win32,
        };
    }

    // Third priority: config default_backend
    default_backend
}

/// Get backend from action if specified
fn get_action_backend(action: &Action) -> Option<InputBackend> {
    match action {
        Action::Key(a) => a.backend.clone(),
        Action::KeySequence(a) => a.backend.clone(),
        Action::MouseClick(a) => a.backend.clone(),
        Action::MouseMove(a) => a.backend.clone(),
        Action::MouseScroll(a) => a.backend.clone(),
        Action::Delay(_) => None, // Delay doesn't use backend
        Action::Text(a) => a.backend.clone(),
    }
}

/// Execute a sequence of key events with proper timing using specified backend
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

/// Send a key event using the configured backend
fn send_key_event(key: &str, direction: &str, ctx: &ExecContext) -> Result<(), String> {
    match ctx.backend {
        InputBackend::Enigo => {
            // Enigo mode: activate window first, then send
            if let Some(hwnd) = ctx.target_hwnd {
                input::activate_window(hwnd)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }
            input::enigo::send_key(key, direction)
        }
        InputBackend::Win32 => {
            // Win32 mode: send directly to target window
            let hwnd = ctx.target_hwnd
                .ok_or("Win32 backend requires target window")?;
            input::send_key_to_window(hwnd, key, direction)
        }
    }
}

/// Send text using the configured backend
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
            let hwnd = ctx.target_hwnd
                .ok_or("Win32 backend requires target window")?;
            input::send_text_to_window(hwnd, text)
        }
    }
}

/// Execute an action by its ID
///
/// # Arguments
/// * `action_id` - The numeric ID of the action to execute
/// * `window` - Optional window spec to activate before executing
/// * `backend` - Optional call parameter to override; if not provided, uses config default
/// * `default_backend` - The default backend from loaded config (used when action has no backend)
pub fn execute_action(
    action_id: u32,
    window: Option<String>,
    backend: Option<String>,
    default_backend: InputBackend,
) -> Result<(), String> {
    // Get config
    let actions = config::get_config()?;

    let action = actions.get(&action_id)
        .ok_or_else(|| format!("Action {} not found in configuration", action_id))?;

    // Resolve the effective backend: action > default > call_param
    let resolved_backend = resolve_backend(action, default_backend, backend.as_deref());

    // Resolve target hwnd if window spec provided
    let target_hwnd = if let Some(ref spec) = window {
        Some(get_hwnd_from_spec(spec)?)
    } else {
        None
    };

    // Activate window if using Enigo backend (Win32 doesn't need activation)
    if resolved_backend == InputBackend::Enigo {
        if let Some(hwnd) = target_hwnd {
            input::activate_window(hwnd)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    let ctx = ExecContext::new(resolved_backend, target_hwnd);

    // Execute the action
    execute_action_impl(action, &ctx)
}

/// Execute an action by its ID with runtime parameters
///
/// This function allows dynamically resolving placeholders in action configurations
/// using parameters provided at runtime via ZMQ or other interfaces.
///
/// # Arguments
/// * `action_id` - The numeric ID of the action to execute
/// * `window` - Optional window spec to activate before executing
/// * `backend` - Optional call parameter to override; if not provided, uses config default
/// * `default_backend` - The default backend from loaded config
/// * `params` - Optional HashMap of parameter values for placeholder substitution
pub fn execute_action_with_params(
    action_id: u32,
    window: Option<String>,
    backend: Option<String>,
    default_backend: InputBackend,
    params: Option<HashMap<String, ParamValue>>,
) -> Result<(), String> {
    // Get config
    let actions = config::get_config()?;

    let action = actions.get(&action_id)
        .ok_or_else(|| format!("Action {} not found in configuration", action_id))?;

    // Resolve placeholders if params are provided
    let resolved_action = if let Some(p) = params {
        let resolver = ActionResolver::new(p);
        resolver.resolve_action(action)?
    } else {
        action.clone()
    };

    // Resolve the effective backend: action > default > call_param
    let resolved_backend = resolve_backend(&resolved_action, default_backend, backend.as_deref());

    // Resolve target hwnd if window spec provided
    let target_hwnd = if let Some(ref spec) = window {
        Some(get_hwnd_from_spec(spec)?)
    } else {
        None
    };

    // Activate window if using Enigo backend (Win32 doesn't need activation)
    if resolved_backend == InputBackend::Enigo {
        if let Some(hwnd) = target_hwnd {
            input::activate_window(hwnd)?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    let ctx = ExecContext::new(resolved_backend, target_hwnd);

    // Execute the action
    execute_action_impl(&resolved_action, &ctx)
}

/// Execute multiple actions based on the execute vector
///
/// # Arguments
/// * `execute` - Vector of booleans where index corresponds to action index
/// * `params` - Optional HashMap of parameter values for placeholder substitution
/// * `default_backend` - The default backend from loaded config
///
/// Returns Ok(()) if all executed actions succeed, Err on first failure
pub fn execute_actions(
    execute: Vec<bool>,
    params: Option<HashMap<String, ParamValue>>,
    default_backend: InputBackend,
) -> Result<(), String> {
    let actions = config::get_config()?;

    // 先收集所有需要执行的 action
    let actions_to_execute: Vec<(u32, &Action)> = execute
        .iter()
        .enumerate()
        .filter(|(_, &should_exec)| should_exec)
        .filter_map(|(idx, _)| {
            let action_idx = idx as u32;
            actions.get(&action_idx).map(|a| (action_idx, a))
        })
        .collect();

    // 如果有 params，检查是否满足所有需要执行的 action 的占位符
    if let Some(ref p) = params {
        let mut missing = Vec::new();

        for (_, action) in &actions_to_execute {
            for placeholder in ActionResolver::extract_needed_placeholders(action) {
                if !p.contains_key(&placeholder) && !missing.contains(&placeholder) {
                    missing.push(placeholder);
                }
            }
        }

        if !missing.is_empty() {
            return Err(format!("Missing parameters: {}", missing.join(", ")));
        }
    }

    let resolver = params.map(ActionResolver::new);

    for (_action_idx, action) in actions_to_execute {
        // Resolve placeholders
        let resolved_action = match &resolver {
            Some(r) => r.resolve_action(action)?,
            None => action.clone(),
        };

        // Resolve backend
        let resolved_backend = resolve_backend(&resolved_action, default_backend.clone(), None);

        // Create execution context (no window spec for ZMQ commands)
        let ctx = ExecContext::new(resolved_backend, None);

        // Execute
        execute_action_impl(&resolved_action, &ctx)?;
    }

    Ok(())
}

/// Helper to get HWND from a window spec string
fn get_hwnd_from_spec(spec: &str) -> Result<isize, String> {
    let search = input::parse_window_spec(spec);

    // Empty spec means foreground window
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
        return Ok(hwnd.0 as isize);
    }

    input::find_window(&search).ok_or_else(|| "Window not found".to_string())
}

/// Execute action implementation
fn execute_action_impl(action: &Action, ctx: &ExecContext) -> Result<(), String> {
    match action {
        Action::Key(key_action) => execute_key(key_action, ctx),
        Action::KeySequence(seq_action) => execute_key_sequence(seq_action, ctx),
        Action::MouseClick(click_action) => execute_mouse_click(click_action, ctx),
        Action::MouseMove(move_action) => execute_mouse_move(move_action, ctx),
        Action::MouseScroll(scroll_action) => execute_mouse_scroll(scroll_action, ctx),
        Action::Delay(delay_action) => execute_delay(delay_action),
        Action::Text(text_action) => execute_text(text_action, ctx),
    }
}

/// Execute a key action
///
/// Supports combination keys with `+` separator, e.g., "ctrl+c", "ctrl+shift+a"
/// For combination keys:
///   1. Press all modifier keys
///   2. Press the main key
///   3. Release all keys
fn execute_key(action: &KeyAction, ctx: &ExecContext) -> Result<(), String> {
    let key = &action.key;

    // Check if it's a combination key (contains '+')
    if key.contains('+') {
        execute_combination_key(key, action.hold_time_ms, ctx)?;
    } else {
        if action.hold_time_ms > 0 {
            // Press and hold
            send_key_event(key, "press", ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(action.hold_time_ms));
            send_key_event(key, "release", ctx)?;
        } else {
            // Simple click
            send_key_event(key, "click", ctx)?;
        }
    }
    Ok(())
}

/// Execute a combination key (e.g., "ctrl+c", "ctrl+shift+a")
///
/// For Win32 backend: uses SendInput to send all keys atomically
/// For Enigo backend: builds an event queue with proper timing
fn execute_combination_key(key: &str, hold_ms: u64, ctx: &ExecContext) -> Result<(), String> {
    let keys: Vec<&str> = key.split('+').map(|s| s.trim()).collect();

    if keys.is_empty() {
        return Err("Invalid combination key".to_string());
    }

    match ctx.backend {
        InputBackend::Win32 => {
            // Win32: use SendInput for atomic key sequence
            let mut key_pairs: Vec<(&str, bool)> = Vec::new();

            // Press all keys
            for k in &keys {
                key_pairs.push((k, true));
            }

            // Hold if specified
            if hold_ms > 0 {
                // Release main key first
                key_pairs.push((keys.last().unwrap(), false));
                // Then release modifier keys in reverse order (skip main key)
                for k in keys.iter().rev().skip(1) {
                    key_pairs.push((k, false));
                }
            } else {
                // Release all keys in reverse order
                for k in keys.iter().rev() {
                    key_pairs.push((k, false));
                }
            }

            // Activate window first
            if let Some(hwnd) = ctx.target_hwnd {
                input::activate_window(hwnd)?;
                std::thread::sleep(std::time::Duration::from_millis(10));
            }

            input::send_key_sequence(&key_pairs)
        }
        InputBackend::Enigo => {
            // Enigo: use existing event-based approach
            let mut events: Vec<KeyEvent> = Vec::new();

            // Press all keys with delay before each
            for k in &keys {
                events.push(KeyEvent {
                    key: k,
                    direction: KeyDirection::Press,
                    delay_ms: DEFAULT_KEY_INTERVAL_MS,
                });
            }

            // Hold if specified
            if hold_ms > 0 {
                // First release the main key after holding
                events.push(KeyEvent {
                    key: keys.last().unwrap(), // main key
                    direction: KeyDirection::Release,
                    delay_ms: hold_ms,
                });
                // Then release all modifier keys in reverse order (skip the main key)
                for k in keys.iter().rev().skip(1) {
                    events.push(KeyEvent {
                        key: k,
                        direction: KeyDirection::Release,
                        delay_ms: DEFAULT_KEY_INTERVAL_MS,
                    });
                }
            } else {
                // Release all keys in reverse order
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

/// Execute a key sequence action
fn execute_key_sequence(action: &KeySequenceAction, ctx: &ExecContext) -> Result<(), String> {
    let default_interval = action.default_interval_ms;

    for (i, item) in action.keys.iter().enumerate() {
        // Execute the key press
        if item.hold_time_ms > 0 {
            // Press and hold
            send_key_event(&item.key, "press", ctx)?;
            std::thread::sleep(std::time::Duration::from_millis(item.hold_time_ms));
            send_key_event(&item.key, "release", ctx)?;
        } else {
            // Simple click
            send_key_event(&item.key, "click", ctx)?;
        }

        // Sleep before next key (unless this is the last key)
        if i < action.keys.len() - 1 {
            let interval = item.interval_ms.unwrap_or(default_interval);
            if interval > 0 {
                std::thread::sleep(std::time::Duration::from_millis(interval));
            }
        }
    }

    Ok(())
}

/// Execute a mouse click action
fn execute_mouse_click(action: &MouseClickAction, ctx: &ExecContext) -> Result<(), String> {
    // Parse string count to u32
    let count = action.count.parse::<u32>()
        .map_err(|_| format!("Invalid click count: {}", action.count))?;

    // Note: For now, mouse clicks always use Enigo as it's more reliable for absolute positioning
    // This could be enhanced to support Win32 backend as well
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

    let button = match action.button {
        MouseButton::Left => Button::Left,
        MouseButton::Right => Button::Right,
        MouseButton::Middle => Button::Middle,
    };

    let interval = action.interval_ms.unwrap_or(0);
    let hold_time = action.hold_time_ms;

    for i in 0..count {
        // Press, hold, then release
        enigo.button(button, Direction::Press)
            .map_err(|e| format!("Failed to press mouse button: {:?}", e))?;

        if hold_time > 0 {
            std::thread::sleep(std::time::Duration::from_millis(hold_time));
        }

        enigo.button(button, Direction::Release)
            .map_err(|e| format!("Failed to release mouse button: {:?}", e))?;

        // Don't sleep after the last click
        if i < count - 1 && interval > 0 {
            std::thread::sleep(std::time::Duration::from_millis(interval));
        }
    }

    Ok(())
}

/// Execute a mouse move action
fn execute_mouse_move(action: &MouseMoveAction, ctx: &ExecContext) -> Result<(), String> {
    // Parse string coordinates to i32
    let x = action.x.parse::<i32>()
        .map_err(|_| format!("Invalid x coordinate: {}", action.x))?;
    let y = action.y.parse::<i32>()
        .map_err(|_| format!("Invalid y coordinate: {}", action.y))?;

    let duration = action.duration_ms.unwrap_or(0);

    match ctx.backend {
        InputBackend::Win32 => {
            // Win32: use SetCursorPos
            let start = input::get_mouse_position()?;
            input::smooth_move(start.0, start.1, x, y, duration)?;
        }
        InputBackend::Enigo => {
            if duration == 0 {
                // Instant move using enigo
                let mut enigo = Enigo::new(&Settings::default())
                    .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;
                enigo.move_mouse(x, y, Coordinate::Abs)
                    .map_err(|e| format!("Failed to move mouse: {:?}", e))?;
            } else {
                // Smooth move: get current position then interpolate
                let start = input::get_mouse_position()?;
                input::smooth_move(start.0, start.1, x, y, duration)?;
            }
        }
    }

    Ok(())
}

/// Execute a mouse scroll action
fn execute_mouse_scroll(action: &MouseScrollAction, ctx: &ExecContext) -> Result<(), String> {
    // Windows default wheel delta is 120 per "click"
    let delta = (action.amount as i32) * 120;

    match ctx.backend {
        InputBackend::Win32 => {
            let hwnd = ctx.target_hwnd
                .ok_or("Win32 backend requires target window for mouse scroll")?;
            input::win32_input::send_mouse_scroll(hwnd, delta)?;
        }
        InputBackend::Enigo => {
            let mut enigo = Enigo::new(&Settings::default())
                .map_err(|e| format!("Failed to create Enigo: {:?}", e))?;

            // Enigo scroll: length (positive=down/right, negative=up/left), axis
            let (length, axis) = match action.direction {
                ScrollDirection::Up => (-(action.amount as i32), Axis::Vertical),
                ScrollDirection::Down => (action.amount as i32, Axis::Vertical),
                ScrollDirection::Left => (-(action.amount as i32), Axis::Horizontal),
                ScrollDirection::Right => (action.amount as i32, Axis::Horizontal),
            };

            enigo.scroll(length, axis)
                .map_err(|e| format!("Failed to scroll: {:?}", e))?;
        }
    }

    Ok(())
}

/// Execute a delay action
fn execute_delay(action: &DelayAction) -> Result<(), String> {
    std::thread::sleep(std::time::Duration::from_millis(action.duration_ms));
    Ok(())
}

/// Execute a text input action
fn execute_text(action: &TextAction, ctx: &ExecContext) -> Result<(), String> {
    send_text_event(&action.content, ctx)
}
