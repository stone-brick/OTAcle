use crate::act;
use crate::communication::{ZmqPuller, ZmqPullState, types::ZmqCommand, state as comm_state};
use tauri::Emitter;
use std::sync::atomic::Ordering;

#[tauri::command]
pub fn zmq_set_address(addr: String) -> Result<(), String> {
    comm_state::set_pull_address(addr)
}

#[tauri::command]
pub fn zmq_start(app: tauri::AppHandle) -> Result<(), String> {
    // 从状态中获取地址
    let addr = {
        let state_guard = comm_state::ZMQ_PULL_STATE.lock().map_err(|_| "Lock failed")?;
        if let Some(ref zmq_state) = *state_guard {
            zmq_state.address.clone()
        } else {
            "tcp://127.0.0.1:5555".to_string()
        }
    };

    // 停止现有的接收者（如果有）
    {
        let mut state_guard = comm_state::ZMQ_PULL_STATE.lock().map_err(|_| "Lock failed")?;
        if let Some(ref mut zmq_state) = *state_guard {
            zmq_state.running.store(false, Ordering::SeqCst);
        }
    }

    // 创建新的 PULL 状态
    let pull_state = ZmqPullState::new(addr.clone());
    let running = pull_state.running.clone();
    let running_for_thread = running.clone();

    // 保存 PULL 状态
    comm_state::set_pull_state(pull_state)?;

    // 克隆 app handle 用于回调
    let app_for_callback = app.clone();

    // 创建接收者
    let puller = ZmqPuller::new(&addr)?;

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
    let mut state_guard = comm_state::ZMQ_PULL_STATE.lock().map_err(|_| "Lock failed")?;
    if let Some(ref mut zmq_state) = *state_guard {
        zmq_state.running.store(false, Ordering::SeqCst);
    }
    Ok(())
}

#[tauri::command]
pub fn zmq_get_status() -> Result<(bool, String), String> {
    let state_guard = comm_state::ZMQ_PULL_STATE.lock().map_err(|_| "Lock failed")?;

    let (connected, address) = if let Some(ref zmq_state) = *state_guard {
        (zmq_state.running.load(Ordering::SeqCst), zmq_state.address.clone())
    } else {
        (false, "tcp://127.0.0.1:5555".to_string())
    };

    Ok((connected, address))
}
