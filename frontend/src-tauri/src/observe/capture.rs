//! 使用 Windows Graphics Capture API 实现窗口截图
//!
//! 使用 windows-capture crate 封装 Windows.Graphics.Capture API。
//! 可以捕获传统 GDI 无法捕获的 GPU 加速窗口（游戏、WebView2、DirectX/Vulkan/OpenGL）。

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
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
use crate::observe::types::{CropBlock, FrameMessage, ObserveConfig};

/// 限制预览帧发送到前端的频率（毫秒）
const PREVIEW_THROTTLE_MS: u64 = 200;

/// 通过 Settings::Flags 传递给 WgcFrameHandler 的数据
struct WgcHandlerData {
    app: AppHandle,
    config: ObserveConfig,
    zmq_tx: std::sync::mpsc::Sender<FrameMessage>,
    frame_id: Arc<AtomicU64>,
    last_preview_time: Mutex<Instant>,
    running: Arc<AtomicBool>,
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

        let (width, height) = (frame.width(), frame.height());

        // 1. 获取帧缓冲区
        let mut buffer = frame.buffer()
            .map_err(|e| format!("Failed to get frame buffer: {}", e))?;

        // 先尝试 as_nopadding_buffer（可能因 API bug 返回空）
        let mut rgba = Vec::new();
        let _clean = buffer.as_nopadding_buffer(&mut rgba);

        // 变通方案：如果 as_nopadding_buffer 返回空，则手动从原始缓冲区去除填充
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

        // 2. 如有需要则缩放
        let scaled = if self.data.config.capture.target_width != width
            || self.data.config.capture.target_height != height
        {
            let processor = ImageProcessor {};
            processor.scale(
                &rgba,
                width,
                height,
                self.data.config.capture.target_width,
                self.data.config.capture.target_height,
            )
        } else {
            rgba
        };

        // 3. 裁剪区域或发送完整帧
        let crop_blocks: Vec<CropBlock> = if self.data.config.crop_regions.is_empty() {
            vec![CropBlock {
                x: 0,
                y: 0,
                w: self.data.config.capture.target_width,
                h: self.data.config.capture.target_height,
                image: base64::engine::general_purpose::STANDARD.encode(&scaled),
            }]
        } else {
            let processor = ImageProcessor {};
            processor.crop_regions(
                &scaled,
                self.data.config.capture.target_width,
                self.data.config.capture.target_height,
                &self.data.config.crop_regions,
            )
        };

        // 4. 构建 FrameMessage
        let frame_msg = FrameMessage {
            width: self.data.config.capture.target_width,
            height: self.data.config.capture.target_height,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            frame_id: self.data.frame_id.fetch_add(1, Ordering::SeqCst),
            data: crop_blocks,
        };

        // 5. 通过 ZMQ 发送（通道接收线程）
        let _ = self.data.zmq_tx.send(frame_msg.clone());

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

/// 使用 Windows Graphics Capture API 的窗口屏幕捕获会话
pub struct CaptureSession {
    running: Arc<AtomicBool>,
    frame_id: Arc<AtomicU64>,
}

impl CaptureSession {
    /// 创建新的捕获会话
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            frame_id: Arc::new(AtomicU64::new(0)),
        }
    }

    /// 启动捕获会话
    ///
    /// # 参数
    /// * `hwnd` - 要捕获的窗口句柄
    /// * `config` - Observe 配置
    /// * `app` - 用于事件的 Tauri app handle
    /// * `zmq_tx` - ZMQ 发布者的通道发送者
    /// * `running` - 用于控制运行状态的原子标志
    pub fn start(
        &self,
        hwnd: isize,
        config: ObserveConfig,
        app: AppHandle,
        zmq_tx: std::sync::mpsc::Sender<FrameMessage>,
        running: Arc<AtomicBool>,
    ) -> Result<(), String> {
        // 使用传入的 running 标志，而不是 self.running
        if running.load(Ordering::SeqCst) {
            return Err("Capture session already running".to_string());
        }

        running.store(true, Ordering::SeqCst);
        // 保存运行标志，以便 stop/is_running 使用
        self.running.store(true, Ordering::SeqCst);

        let frame_id = self.frame_id.clone();
        let config_clone = config.clone();
        let app_clone = app.clone();

        // 从 HWND 创建 Window
        let window = Window::from_raw_hwnd(hwnd as *mut std::ffi::c_void);

        // 构建处理器数据（使用 Arc 以便与处理器共享）
        let handler_data = Arc::new(WgcHandlerData {
            app: app_clone,
            config: config_clone,
            zmq_tx,
            frame_id: frame_id.clone(),
            last_preview_time: Mutex::new(Instant::now()),
            running,
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
        let _handle = thread::spawn(move || {
            let _control = WgcFrameHandler::start_free_threaded(settings);
            // 当线程退出时 _control 被丢弃，这会停止捕获。
            // 但是，如果 stop() 被调用，处理器将在下一帧到达时
            // 已经调用 capture_control.stop()。
        });

        Ok(())
    }

    /// 停止捕获会话
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        // 处理器将在下一帧到达时检测到 running=false
        // 并调用 capture_control.stop() 以优雅地停止捕获。
    }

    /// 检查捕获会话是否正在运行
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for CaptureSession {
    fn default() -> Self {
        Self::new()
    }
}
