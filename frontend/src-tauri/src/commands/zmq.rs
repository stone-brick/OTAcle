use crate::act::zmq_state::ZMQ_STATE;
use crate::act;
use crate::act::zmq_pull::{ZmqCommand, ZmqPuller};
use tauri::Emitter;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[tauri::command]
pub fn zmq_start(
    addr: String,
    app: tauri::AppHandle,
) -> Result<(), String> {
    // 停止现有的接收者（如果有）
    {
        let mut state_guard = ZMQ_STATE.lock().map_err(|_| "Lock failed")?;
        if let Some(zmq_state) = state_guard.as_mut() {
            let mut running_guard = zmq_state.running.lock().map_err(|_| "Lock failed")?;
            if let Some(running) = running_guard.take() {
                running.store(false, Ordering::SeqCst);
            }
        }
    }

    // 保存新地址
    {
        let mut state_guard = ZMQ_STATE.lock().map_err(|_| "Lock failed")?;
        let zmq_state = state_guard.get_or_insert_with(crate::act::zmq_state::ZmqState::default);
        let mut addr_guard = zmq_state.address.lock().map_err(|_| "Lock failed")?;
        *addr_guard = addr.clone();
    }

    // 创建新的接收者
    let puller = ZmqPuller::new(&addr)?;

    // 创建运行标志
    let running = Arc::new(AtomicBool::new(true));

    // 克隆用于存储
    let running_for_state = running.clone();
    let running_for_thread = running.clone();

    // 保存运行标志
    {
        let mut state_guard = ZMQ_STATE.lock().map_err(|_| "Lock failed")?;
        let zmq_state = state_guard.as_mut().unwrap();
        let mut running_guard = zmq_state.running.lock().map_err(|_| "Lock failed")?;
        *running_guard = Some(running_for_state);
    }

    // 克隆 app handle 用于回调
    let app_for_callback = app.clone();

    // 在独立线程中开始监听
    puller.start(running_for_thread, move |cmd: ZmqCommand| {
        let default_backend = act::config::get_default_backend()
            .unwrap_or(act::types::InputBackend::Win32);

        let result = act::executor::execute_actions(
            cmd.execute,
            cmd.params,
            default_backend,
        );

        match result {
            Ok(()) => {
                let msg = "[ZMQ] Actions executed successfully".to_string();
                let _ = app_for_callback.emit("zmq:log", msg);
            }
            Err(e) => {
                let msg = format!("[ZMQ] Error: {}", e);
                let _ = app_for_callback.emit("zmq:error", msg);
            }
        }
    })?;

    Ok(())
}

#[tauri::command]
pub fn zmq_stop() -> Result<(), String> {
    let mut state_guard = ZMQ_STATE.lock().map_err(|_| "Lock failed")?;
    if let Some(zmq_state) = state_guard.as_mut() {
        let mut running_guard = zmq_state.running.lock().map_err(|_| "Lock failed")?;
        if let Some(running) = running_guard.take() {
            running.store(false, Ordering::SeqCst);
        }
    }
    Ok(())
}

#[tauri::command]
pub fn zmq_get_status() -> Result<(bool, String), String> {
    let state_guard = ZMQ_STATE.lock().map_err(|_| "Lock failed")?;

    let (connected, address) = if let Some(zmq_state) = state_guard.as_ref() {
        let running_guard = zmq_state.running.lock().map_err(|_| "Lock failed")?;
        let addr_guard = zmq_state.address.lock().map_err(|_| "Lock failed")?;
        (running_guard.is_some(), addr_guard.clone())
    } else {
        (false, "tcp://127.0.0.1:5555".to_string())
    };

    Ok((connected, address))
}
