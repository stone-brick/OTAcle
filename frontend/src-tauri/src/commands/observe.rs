use crate::input;
use crate::observe;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

#[tauri::command]
pub fn observe_start(
    window: String,
    config: observe::types::ObserveConfig,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // Parse window spec
    let search = input::parse_window_spec(&window);
    let hwnd = input::find_window(&search)
        .ok_or_else(|| format!("Window not found: {}", window))?;

    // Get or create observe session
    let mut session_guard = observe::OBSERVE_SESSION.lock()
        .map_err(|_| "Failed to lock observe session".to_string())?;

    let session = session_guard.get_or_insert_with(observe::capture::CaptureSession::new);

    if session.is_running() {
        return Err("Observe session already running".to_string());
    }

    // Create ZMQ channel and publisher
    let (zmq_tx, zmq_rx) = std::sync::mpsc::channel();
    let zmq_addr = config.zmq.address.clone();
    let running = Arc::new(AtomicBool::new(false));
    let running_for_pub = running.clone();
    crate::zmq_pub::start_publisher(&zmq_addr, running_for_pub, zmq_rx)?;

    // Start capture session, passing zmq_tx and running
    session.start(hwnd as isize, config, app, zmq_tx, running)
}

#[tauri::command]
pub fn observe_stop() -> Result<(), String> {
    let session_guard = observe::OBSERVE_SESSION.lock()
        .map_err(|_| "Failed to lock observe session".to_string())?;

    if let Some(session) = session_guard.as_ref() {
        session.stop();
    }

    Ok(())
}

#[tauri::command]
pub fn observe_get_status() -> Result<bool, String> {
    let session_guard = observe::OBSERVE_SESSION.lock()
        .map_err(|_| "Failed to lock observe session".to_string())?;

    Ok(session_guard.as_ref().map(|s| s.is_running()).unwrap_or(false))
}

#[tauri::command]
pub fn observe_save_config(path: String, config: observe::types::ObserveConfig) -> Result<(), String> {
    observe::config::save_config(&path, &config)
}

#[tauri::command]
pub fn observe_load_config(path: String) -> Result<observe::types::ObserveConfig, String> {
    observe::config::load_config(&path)
}
