use crate::communication;
use crate::communication::types::{FrameMessage, PubState};
use crate::observe::types::FullFrameMessage;
use crate::input;
use crate::observe;
use crate::observe::state::{GLOBAL_STATS, SessionHandle};
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::sync::Mutex;

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

    // Check if already running
    {
        let sessions = observe::SESSIONS.lock()
            .map_err(|_| "Failed to lock sessions".to_string())?;
        if sessions.contains_key(&hwnd) {
            return Err("Observe already running for this window".to_string());
        }
    }

    // Create shared running flag
    let running = Arc::new(AtomicBool::new(false));
    let running_for_pub = running.clone();
    let running_for_capture = running.clone();

    // Create PUB state
    let pub_state = Arc::new(Mutex::new(PubState::new()));
    let pub_state_clone = pub_state.clone();

    // 创建帧通道和发布线程
    let (frame_tx, frame_rx) = std::sync::mpsc::channel();
    let pub_addr = communication::get_pub_address()?;
    let publisher_handle = communication::start_publisher(
        &pub_addr,
        running_for_pub,
        frame_rx,
        pub_state_clone,
        app.clone(),
    )?;

    // Create session stats
    let stats = Arc::new(observe::state::SessionStats::new());

    // Create latest full frame storage (供前端轮询)
    let latest_full_frame = Arc::new(Mutex::new(None));
    let latest_full_frame_for_capture = latest_full_frame.clone();

    // Start capture and get handle
    let capture_handle = observe::capture::start_capture(
        hwnd,
        config.clone(),
        app,
        frame_tx,
        running_for_capture,
        stats,
        latest_full_frame_for_capture,
    )?;

    // Create session handle with full info
    let session = SessionHandle::new(
        hwnd,
        config,
        running,
        Some(capture_handle),
        Some(publisher_handle),
        latest_full_frame,
    );

    // Register session
    let mut sessions = observe::SESSIONS.lock()
        .map_err(|_| "Failed to lock sessions".to_string())?;
    sessions.insert(hwnd, session);

    // Update global stats
    if let Ok(mut global) = GLOBAL_STATS.lock() {
        global.active_sessions = sessions.len();
    }

    Ok(())
}

#[tauri::command]
pub fn observe_stop() -> Result<(), String> {
    // 获取 sessions 的锁
    let mut sessions = observe::SESSIONS.lock()
        .map_err(|_| "Failed to lock sessions".to_string())?;

    // 1. 设置 running = false 通知线程停止
    for (_hwnd, handle) in sessions.iter() {
        handle.running.store(false, std::sync::atomic::Ordering::SeqCst);
    }

    // 2. 使用 drain 取出所有 SessionHandle，以便在锁外 join
    let handles: Vec<(Option<std::thread::JoinHandle<()>>, Option<std::thread::JoinHandle<()>>)> = sessions
        .drain()
        .map(|(_hwnd, mut handle)| {
            let capture = handle.capture_handle.take();
            let publisher = handle.publisher_handle.take();
            (capture, publisher)
        })
        .collect();

    // sessions 现在为空，更新全局统计
    if let Ok(mut global) = GLOBAL_STATS.lock() {
        global.active_sessions = 0;
    }

    // 锁在这里自动释放

    // 3. 等待所有线程结束
    //    - publisher 线程会在收到 Disconnected 后退出（channel 被 drop）
    //    - capture 线程会在下一帧到达时检查 running == false 并退出
    for (capture_handle, publisher_handle) in handles {
        if let Some(h) = capture_handle {
            let _ = h.join();
        }
        if let Some(h) = publisher_handle {
            let _ = h.join();
        }
    }

    Ok(())
}

#[tauri::command]
pub fn observe_get_status() -> Result<observe::status::ObserveStatus, String> {
    observe::status::get_full_status()
}

#[tauri::command]
pub fn observe_save_config(path: String, config: observe::types::ObserveConfig) -> Result<(), String> {
    observe::config::save_config(&path, &config)
}

#[tauri::command]
pub fn observe_load_config(path: String) -> Result<observe::types::ObserveConfig, String> {
    observe::config::load_config(&path)
}

#[tauri::command]
pub fn observe_add_crop_region(region: observe::types::CropRegion) -> Result<(), String> {
    observe::config::add_crop_region(region)
}

#[tauri::command]
pub fn observe_remove_crop_region(index: usize) -> Result<(), String> {
    observe::config::remove_crop_region(index)
}

#[tauri::command]
pub fn observe_get_config() -> Result<observe::types::ObserveConfig, String> {
    observe::config::get_config()
}

#[tauri::command]
pub fn observe_capture_preview(
    window: String,
    config: observe::types::ObserveConfig,
) -> Result<FrameMessage, String> {
    let search = input::parse_window_spec(&window);
    let hwnd = input::find_window(&search)
        .ok_or_else(|| format!("Window not found: {}", window))?;

    observe::capture::capture_screenshot(hwnd, config)
}

#[tauri::command]
pub fn observe_capture_full_frame(
    window: String,
    config: observe::types::ObserveConfig,
) -> Result<FullFrameMessage, String> {
    let search = input::parse_window_spec(&window);
    let hwnd = input::find_window(&search)
        .ok_or_else(|| format!("Window not found: {}", window))?;

    observe::capture::capture_full_frame(hwnd, config)
}

