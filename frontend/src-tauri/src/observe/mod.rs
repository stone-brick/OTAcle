//! Observe module - window capture and preprocessing for Python side
//!
//! This module provides:
//! - Window screenshot capture (Windows Graphics Capture API via windows-capture)
//! - Image preprocessing (scaling, cropping)
//! - ZMQ PUB for sending frames to Python
//! - Tauri events for frontend preview

pub mod types;
pub mod config;
pub mod processor;
pub mod capture;

pub use types::*;
pub use capture::CaptureSession;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    /// Global capture session
    pub static ref CAPTURE_SESSION: Mutex<Option<CaptureSession>> = Mutex::new(None);
}

/// Start observe capture
#[tauri::command]
pub fn start_observe(
    window: String,
    config: ObserveConfig,
    app: tauri::AppHandle,
) -> Result<(), String> {
    use crate::input;

    // Parse window spec
    let search = input::parse_window_spec(&window);
    let hwnd = input::find_window(&search)
        .ok_or_else(|| format!("Window not found: {}", window))?;

    // Get or create capture session
    let mut session_guard = CAPTURE_SESSION.lock()
        .map_err(|_| "Failed to lock capture session".to_string())?;

    let session = session_guard.get_or_insert_with(CaptureSession::new);

    if session.is_running() {
        return Err("Capture session already running".to_string());
    }

    session.start(hwnd as isize, config, app)
}

/// Stop observe capture
#[tauri::command]
pub fn stop_observe() -> Result<(), String> {
    let session_guard = CAPTURE_SESSION.lock()
        .map_err(|_| "Failed to lock capture session".to_string())?;

    if let Some(session) = session_guard.as_ref() {
        session.stop();
    }

    Ok(())
}

/// Get observe status
#[tauri::command]
pub fn get_observe_status() -> Result<bool, String> {
    let session_guard = CAPTURE_SESSION.lock()
        .map_err(|_| "Failed to lock capture session".to_string())?;

    Ok(session_guard.as_ref().map(|s| s.is_running()).unwrap_or(false))
}

/// Save observe configuration to a JSON file
#[tauri::command]
pub fn save_observe_config(path: String, config: ObserveConfig) -> Result<(), String> {
    crate::observe::config::save_config(&path, &config)
}

/// Load observe configuration from a JSON file
#[tauri::command]
pub fn load_observe_config(path: String) -> Result<ObserveConfig, String> {
    crate::observe::config::load_config(&path)
}
