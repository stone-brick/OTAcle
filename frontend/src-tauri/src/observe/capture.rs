//! 使用 Windows Graphics Capture API 实现窗口截图
//!
//! 使用 windows-capture crate 封装 Windows.Graphics.Capture API。
//! 可以捕获传统 GDI 无法捕获的 GPU 加速窗口（游戏、WebView2、DirectX/Vulkan/OpenGL）。

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use base64::Engine;
use tauri::{AppHandle, Emitter};

use windows_capture::capture::{Context, GraphicsCaptureApiHandler};
use windows_capture::frame::Frame;
use windows_capture::graphics_capture_api::InternalCaptureControl;
use windows_capture::settings::{
    ColorFormat, CursorCaptureSettings, DirtyRegionSettings, DrawBorderSettings,
    MinimumUpdateIntervalSettings, SecondaryWindowSettings, Settings,
};
use windows_capture::window::Window;

use crate::observe::processor::ImageProcessor;
use crate::observe::types::ObserveConfig;
use crate::observe::state::SessionStats;
use crate::communication::types::{CropBlock, FrameMessage};

/// 限制预览帧发送到前端的频率（毫秒）
const PREVIEW_THROTTLE_MS: u64 = 200;

/// 通过 Settings::Flags 传递给 WgcFrameHandler 的数据
struct WgcHandlerData {
    app: AppHandle,
    config: ObserveConfig,
    frame_tx: std::sync::mpsc::Sender<FrameMessage>,
    frame_id: Arc<AtomicU64>,
    last_preview_time: Mutex<Instant>,
    running: Arc<AtomicBool>,
    stats: Arc<SessionStats>,
}

/// 通过 Settings::Flags 传递给 OneShotFrameHandler 的数据
struct OneShotHandlerData {
    config: ObserveConfig,
    frame_id: Arc<AtomicU64>,
    result_tx: std::sync::mpsc::Sender<Result<FrameMessage, String>>,
}

/// 实现 GraphicsCaptureApiHandler trait 的帧处理器
struct WgcFrameHandler {
    data: Arc<WgcHandlerData>,
}

impl GraphicsCaptureApiHandler for WgcFrameHandler {
    type Flags = Arc<WgcHandlerData>;
    type Error = String;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self { data: ctx.flags })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        // 检查是否请求停止
        if !self.data.running.load(Ordering::SeqCst) {
            capture_control.stop();
            return Ok(());
        }

        // 2. 处理帧（使用 process_frame 消除重复代码）
        let frame_msg = process_frame(
            frame,
            &self.data.config,
            &self.data.frame_id,
        )?;

        // 5. 更新统计
        self.data.stats.add_frames_captured(1);
        // 估算字节数：base64 编码后的大小
        let estimated_bytes: u64 = frame_msg.data.iter().map(|b| b.image.len() as u64).sum();
        self.data.stats.add_bytes_sent(estimated_bytes);

        // 6. 通过 ZMQ 发送（通道接收线程）
        let _ = self.data.frame_tx.send(frame_msg.clone());

        // 6. 发送到前端预览（节流）
        let now = Instant::now();
        let mut last_time = self.data.last_preview_time.lock().unwrap();
        let elapsed = now.duration_since(*last_time).as_millis() as u64;
        if elapsed >= PREVIEW_THROTTLE_MS {
            *last_time = now;
            drop(last_time); // 在发送前释放锁
            if let Err(e) = self.data.app.emit("observe:frame", &frame_msg) {
                eprintln!("Tauri emit error: {}", e);
            }
        }

        Ok(())
    }
}

/// 从帧缓冲区去除行填充，获取干净的 RGBA 数据
fn remove_padding(width: u32, height: u32, buffer: &mut windows_capture::frame::FrameBuffer) -> Vec<u8> {
    let mut rgba = Vec::new();
    let _clean = buffer.as_nopadding_buffer(&mut rgba);

    if rgba.is_empty() || rgba.len() != (width as usize * height as usize * 4) {
        let row_pitch = buffer.row_pitch();
        let raw = buffer.as_raw_buffer();
        let pixel_stride = 4; // RGBA8 = 每像素 4 字节

        rgba.clear();
        for row in 0..height {
            let row_start = (row * row_pitch) as usize;
            let row_end = row_start + (width * pixel_stride) as usize;
            if row_end <= raw.len() {
                rgba.extend_from_slice(&raw[row_start..row_end]);
            }
        }
    }
    rgba
}

/// 处理帧并构建 FrameMessage
fn process_frame(
    frame: &mut Frame,
    config: &ObserveConfig,
    frame_id: &AtomicU64,
) -> Result<FrameMessage, String> {
    let (width, height) = (frame.width(), frame.height());

    let mut buffer = frame.buffer()
        .map_err(|e| format!("Failed to get frame buffer: {}", e))?;

    let rgba = remove_padding(width, height, &mut buffer);

    let scaled = if config.capture.target_width != width
        || config.capture.target_height != height
    {
        let processor = ImageProcessor {};
        processor.scale(
            &rgba,
            width,
            height,
            config.capture.target_width,
            config.capture.target_height,
        )
    } else {
        rgba
    };

    let crop_blocks: Vec<CropBlock> = if config.crop_regions.is_empty() {
        vec![CropBlock {
            x: 0,
            y: 0,
            w: config.capture.target_width,
            h: config.capture.target_height,
            image: base64::engine::general_purpose::STANDARD.encode(&scaled),
        }]
    } else {
        let processor = ImageProcessor {};
        processor.crop_regions(
            &scaled,
            config.capture.target_width,
            config.capture.target_height,
            &config.crop_regions,
        )
    };

    Ok(FrameMessage {
        width: config.capture.target_width,
        height: config.capture.target_height,
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
        frame_id: frame_id.fetch_add(1, Ordering::SeqCst),
        data: crop_blocks,
    })
}

/// 实现 GraphicsCaptureApiHandler trait 的单次截图处理器
struct OneShotFrameHandler {
    data: Arc<OneShotHandlerData>,
}

impl GraphicsCaptureApiHandler for OneShotFrameHandler {
    type Flags = Arc<OneShotHandlerData>;
    type Error = String;

    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self { data: ctx.flags })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        let result = process_frame(frame, &self.data.config, &self.data.frame_id);
        let _ = self.data.result_tx.send(result);
        capture_control.stop();
        Ok(())
    }
}

/// 单次截图函数
///
/// 捕获一帧后立即停止，不持续推流。
///
/// # 参数
/// * `hwnd` - 要捕获的窗口句柄
/// * `config` - Observe 配置
///
/// # 返回值
/// 返回 `FrameMessage` 或错误信息
pub fn capture_screenshot(hwnd: isize, config: ObserveConfig) -> Result<FrameMessage, String> {
    if config.capture.target_width == 0 || config.capture.target_height == 0 {
        return Err("Invalid target dimensions".to_string());
    }

    let (result_tx, result_rx) = std::sync::mpsc::channel();
    let frame_id = Arc::new(AtomicU64::new(0));

    let window = Window::from_raw_hwnd(hwnd as *mut std::ffi::c_void);

    let handler_data = Arc::new(OneShotHandlerData {
        config: config.clone(),
        frame_id: frame_id.clone(),
        result_tx,
    });

    let settings = Settings::new(
        window,
        CursorCaptureSettings::WithoutCursor,
        DrawBorderSettings::WithoutBorder,
        SecondaryWindowSettings::Default,
        MinimumUpdateIntervalSettings::Default,
        DirtyRegionSettings::Default,
        ColorFormat::Rgba8,
        handler_data.clone(),
    );

    let handle = thread::spawn(move || {
        let _control = OneShotFrameHandler::start_free_threaded(settings);
    });

    let result = result_rx
        .recv()
        .map_err(|_| "Channel disconnected".to_string())??;
    let _ = handle.join();
    Ok(result)
}

/// 启动窗口捕获会话
///
/// # 参数
/// * `hwnd` - 要捕获的窗口句柄
/// * `config` - Observe 配置
/// * `app` - 用于事件的 Tauri app handle
/// * `frame_tx` - ZMQ 发布者的通道发送者
/// * `running` - 用于控制运行状态的原子标志
/// * `stats` - 会话统计计数器
///
/// # 返回值
/// 返回 `JoinHandle<()>` 用于等待线程结束
pub fn start_capture(
    hwnd: isize,
    config: ObserveConfig,
    app: AppHandle,
    frame_tx: std::sync::mpsc::Sender<FrameMessage>,
    running: Arc<AtomicBool>,
    stats: Arc<SessionStats>,
) -> Result<JoinHandle<()>, String> {
    if running.load(Ordering::SeqCst) {
        return Err("Capture already running".to_string());
    }

    running.store(true, Ordering::SeqCst);

    let frame_id = Arc::new(AtomicU64::new(0));
    let config_clone = config.clone();
    let app_clone = app.clone();

    // 从 HWND 创建 Window
    let window = Window::from_raw_hwnd(hwnd as *mut std::ffi::c_void);

    // 构建处理器数据（使用 Arc 以便与处理器共享）
    let handler_data = Arc::new(WgcHandlerData {
        app: app_clone,
        config: config_clone,
        frame_tx,
        frame_id: frame_id.clone(),
        last_preview_time: Mutex::new(Instant::now()),
        running,
        stats,
    });

    // 构建设置
    let settings = Settings::new(
        window,
        CursorCaptureSettings::WithoutCursor,
        DrawBorderSettings::WithoutBorder,
        SecondaryWindowSettings::Default,
        MinimumUpdateIntervalSettings::Default,
        DirtyRegionSettings::Default,
        ColorFormat::Rgba8,
        handler_data.clone(),
    );

    // 生成调用 start_free_threaded 的线程
    // 捕获在由 windows-capture 管理的内部后台线程上运行。
    // running 标志指示何时停止 - 处理器在每帧到达时检查它
    // 当为 false 时调用 capture_control.stop()。
    let handle = thread::spawn(move || {
        let _control = WgcFrameHandler::start_free_threaded(settings);
        // 当线程退出时 _control 被丢弃，这会停止捕获。
        // 但是，如果 stop() 被调用，处理器将在下一帧到达时
        // 已经调用 capture_control.stop()。
    });

    Ok(handle)
}
