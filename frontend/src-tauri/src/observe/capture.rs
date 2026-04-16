//! Window capture implementation using Windows Graphics Capture API
//!
//! Uses the windows-capture crate which wraps the Windows.Graphics.Capture API.
//! This captures GPU-accelerated windows (games, WebView2, DirectX/Vulkan/OpenGL)
//! that traditional GDI cannot capture.

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

/// Throttle preview emit to frontend (ms)
const PREVIEW_THROTTLE_MS: u64 = 200;

/// Data passed to WgcFrameHandler via Settings::Flags
struct WgcHandlerData {
    app: AppHandle,
    config: ObserveConfig,
    zmq_tx: std::sync::mpsc::Sender<FrameMessage>,
    frame_id: Arc<AtomicU64>,
    last_preview_time: Mutex<Instant>,
    running: Arc<AtomicBool>,
}

/// The frame handler implementing GraphicsCaptureApiHandler trait
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
        // Check if stop was requested
        if !self.data.running.load(Ordering::SeqCst) {
            capture_control.stop();
            return Ok(());
        }

        let (width, height) = (frame.width(), frame.height());

        // 1. Get frame buffer
        let mut buffer = frame.buffer()
            .map_err(|e| format!("Failed to get frame buffer: {}", e))?;

        // Try as_nopadding_buffer first (may return empty due to API bug)
        let mut rgba = Vec::new();
        let _clean = buffer.as_nopadding_buffer(&mut rgba);

        // Workaround: if as_nopadding_buffer returns empty, manually strip padding from raw buffer
        if rgba.is_empty() || rgba.len() != (width as usize * height as usize * 4) {
            let row_pitch = buffer.row_pitch();
            let raw = buffer.as_raw_buffer();
            let pixel_stride = 4; // RGBA8 = 4 bytes per pixel

            rgba.clear();
            for row in 0..height {
                let row_start = (row * row_pitch) as usize;
                let row_end = row_start + (width * pixel_stride) as usize;
                if row_end <= raw.len() {
                    rgba.extend_from_slice(&raw[row_start..row_end]);
                }
            }
        }

        // 2. Scale if needed
        let scaled = if self.data.config.capture.target_width != width as u32
            || self.data.config.capture.target_height != height as u32
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

        // 3. Crop regions or send full frame
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

        // 4. Build FrameMessage
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

        // 5. Send via ZMQ (channel receiver thread)
        let _ = self.data.zmq_tx.send(frame_msg.clone());

        // 6. Emit to frontend for preview (throttled)
        let now = Instant::now();
        let mut last_time = self.data.last_preview_time.lock().unwrap();
        let elapsed = now.duration_since(*last_time).as_millis() as u64;
        if elapsed >= PREVIEW_THROTTLE_MS {
            *last_time = now;
            drop(last_time); // release lock before emit
            if let Err(e) = self.data.app.emit("observe:frame", &frame_msg) {
                eprintln!("Tauri emit error: {}", e);
            }
        }

        Ok(())
    }
}

/// Capture session for window screen capture using Windows Graphics Capture API
pub struct CaptureSession {
    running: Arc<AtomicBool>,
    frame_id: Arc<AtomicU64>,
}

impl CaptureSession {
    /// Create a new capture session
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            frame_id: Arc::new(AtomicU64::new(0)),
        }
    }

    /// Start the capture session
    pub fn start(
        &self,
        hwnd: isize,
        config: ObserveConfig,
        app: AppHandle,
    ) -> Result<(), String> {
        if self.running.load(Ordering::SeqCst) {
            return Err("Capture session already running".to_string());
        }

        self.running.store(true, Ordering::SeqCst);

        let frame_id = self.frame_id.clone();
        let config_clone = config.clone();
        let app_clone = app.clone();
        let running = self.running.clone();

        // Create ZMQ channel and sender thread
        let (zmq_tx, zmq_rx) = std::sync::mpsc::channel();
        let zmq_addr = config.zmq.address.clone();
        let zmq_tx_for_handler = zmq_tx.clone(); // handler data needs its own copy

        // Sender thread - owns zmq_rx, keeps channel alive while running
        thread::spawn(move || {
            let ctx = zmq::Context::new();
            let socket = match ctx.socket(zmq::PUB) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("ZMQ socket creation failed: {}", e);
                    return;
                }
            };
            if let Err(e) = socket.bind(&zmq_addr) {
                eprintln!("ZMQ bind failed for {}: {}", zmq_addr, e);
                return;
            }
            let _ = socket.set_conflate(true);

            while let Ok(frame) = zmq_rx.recv() {
                let json = match serde_json::to_string(&frame) {
                    Ok(j) => j,
                    Err(_) => continue,
                };
                if socket.send(json.as_bytes(), 0).is_err() {
                    break;
                }
            }
        });

        // Create Window from HWND
        let window = Window::from_raw_hwnd(hwnd as *mut std::ffi::c_void);

        // Build handler data (Arc so it can be shared with the handler)
        // zmq_tx_for_handler keeps the sender alive as long as the handler exists
        let handler_data = Arc::new(WgcHandlerData {
            app: app_clone,
            config: config_clone,
            zmq_tx: zmq_tx_for_handler,
            frame_id: frame_id.clone(),
            last_preview_time: Mutex::new(Instant::now()),
            running,
        });

        // Build settings
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

        // Spawn thread that calls start_free_threaded
        // The capture runs on an internal background thread managed by windows-capture.
        // The running flag signals when to stop - the handler checks it on each frame
        // and calls capture_control.stop() when false.
        let _handle = thread::spawn(move || {
            let _control = WgcFrameHandler::start_free_threaded(settings);
            // _control is dropped when the thread exits, which stops the capture.
            // However, if stop() was called, the handler will have already called
            // capture_control.stop() on the next frame arrival.
        });

        Ok(())
    }

    /// Stop the capture session
    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
        // The handler will detect running=false on the next frame arrival
        // and call capture_control.stop() to gracefully stop capture.
    }

    /// Check if capture session is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }
}

impl Default for CaptureSession {
    fn default() -> Self {
        Self::new()
    }
}
