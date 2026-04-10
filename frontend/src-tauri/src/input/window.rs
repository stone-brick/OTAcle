//! Window activation using Windows API, inspired by AutoHotkey's SetForegroundWindowEx

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId, WaitForInputIdle};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, IsIconic, PostMessageW, SetForegroundWindow,
    ShowWindow, SW_RESTORE, WM_KEYDOWN, WM_KEYUP,
};

// VK_MENU = 0x12 (Alt key)
const VK_MENU: u32 = 0x12;

/// Activates a window by its HWND with AutoHotkey-style retry logic.
/// Returns Ok(true) if the window was successfully activated, Ok(false) if activation failed.
///
/// Key improvements:
/// 1. Uses three-way AttachThreadInput (current→fore, fore→target)
/// 2. Retries up to 5 times with delays
/// 3. Falls back to Alt-up keystroke if all attempts fail
/// 4. Verifies activation success by checking foreground window
pub fn activate_window(hwnd: isize) -> Result<bool, String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    // Guard against null HWND
    if hwnd.0.is_null() {
        return Err("Invalid window handle (null)".to_string());
    }

    unsafe {
        let target_thread = GetWindowThreadProcessId(hwnd, None);
        let current_thread = GetCurrentThreadId();

        // Check if window is already active
        if GetForegroundWindow() == hwnd {
            return Ok(true);
        }

        // Check if minimized and restore if needed
        if IsIconic(hwnd).as_bool() {
            ShowWindow(hwnd, SW_RESTORE);
        }

        // Get foreground window thread for three-way attach
        let fore_wnd = GetForegroundWindow();
        let fore_thread = if !fore_wnd.0.is_null() {
            GetWindowThreadProcessId(fore_wnd, None)
        } else {
            0
        };

        // Three-way AttachThreadInput (AutoHotkey style):
        // 1. Attach current thread to foreground window's thread
        // 2. Attach foreground window's thread to target window's thread
        let attached_my_to_fore;
        let attached_fore_to_target;

        if fore_thread != 0 && fore_thread != current_thread {
            attached_my_to_fore = AttachThreadInput(current_thread, fore_thread, true).as_bool();
        } else {
            attached_my_to_fore = false;
        }

        if fore_thread != 0 && target_thread != 0 && fore_thread != target_thread {
            attached_fore_to_target = AttachThreadInput(fore_thread, target_thread, true).as_bool();
        } else {
            attached_fore_to_target = false;
        }

        // Retry loop - up to 5 attempts like AutoHotkey
        let mut attempted_alt_up = false;

        for i in 0..5 {
            // Try Alt-up on first retry if previous attempts failed (AutoHotkey's sTriedKeyUp logic)
            if i == 1 && !attempted_alt_up {
                attempted_alt_up = true;
                // Send Alt-up event to unlock foreground
                send_alt_up();
            }

            if SetForegroundWindow(hwnd).as_bool() {
                break;
            }

            // Sleep between attempts (SLEEP_INTERVAL ~10ms)
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // Detach thread inputs (critical to avoid issues next time)
        if attached_my_to_fore {
            let _ = AttachThreadInput(current_thread, fore_thread, false);
        }
        if attached_fore_to_target {
            let _ = AttachThreadInput(fore_thread, target_thread, false);
        }

        // Wait for window to finish processing input
        WaitForInputIdle(hwnd, 5000);
    }

    // Verify activation success by checking foreground window
    unsafe {
        std::thread::sleep(std::time::Duration::from_millis(50));
        if GetForegroundWindow() == hwnd {
            Ok(true)
        } else {
            // One more retry after a short delay
            std::thread::sleep(std::time::Duration::from_millis(100));
            if GetForegroundWindow() == hwnd {
                Ok(true)
            } else {
                Ok(false) // Window was stolen by another app
            }
        }
    }
}

/// Sends Alt-up event to unlock foreground lock
fn send_alt_up() {
    unsafe {
        let fore_wnd = GetForegroundWindow();
        if !fore_wnd.0.is_null() {
            // Send WM_KEYDOWN then WM_KEYUP for Alt (VK_MENU = 0x12)
            let _ = PostMessageW(fore_wnd, WM_KEYDOWN, WPARAM(VK_MENU as usize), LPARAM(0));
            let _ = PostMessageW(fore_wnd, WM_KEYUP, WPARAM(VK_MENU as usize), LPARAM(0));
        }
    }
}
