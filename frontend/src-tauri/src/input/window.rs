//! 使用 Windows API 的窗口激活，灵感来自 AutoHotkey 的 SetForegroundWindowEx

use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId, WaitForInputIdle};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowThreadProcessId, IsIconic, PostMessageW, SetForegroundWindow,
    ShowWindow, SW_RESTORE, WM_KEYDOWN, WM_KEYUP,
};

// VK_MENU = 0x12（Alt 键）
const VK_MENU: u32 = 0x12;

/// 通过 HWND 激活窗口，具有 AutoHotkey 风格的重试逻辑。
/// 如果窗口成功激活则返回 Ok(true)，激活失败则返回 Ok(false)。
///
/// 关键改进：
/// 1. 使用三路 AttachThreadInput（current→fore，fore→target）
/// 2. 最多重试 5 次并带有延迟
/// 3. 如果所有尝试都失败，则回退到 Alt-up 按键
/// 4. 通过检查前台窗口来验证激活成功
pub fn activate_window(hwnd: isize) -> Result<bool, String> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    // 防止空 HWND
    if hwnd.0.is_null() {
        return Err("Invalid window handle (null HWND)".to_string());
    }

    unsafe {
        let target_thread = GetWindowThreadProcessId(hwnd, None);
        let current_thread = GetCurrentThreadId();

        // 检查窗口是否已经处于活动状态
        if GetForegroundWindow() == hwnd {
            return Ok(true);
        }

        // 检查是否最小化并在需要时恢复
        if IsIconic(hwnd).as_bool() {
            let _ = ShowWindow(hwnd, SW_RESTORE);
        }

        // 获取前台窗口线程用于三路附加
        let fore_wnd = GetForegroundWindow();
        let fore_thread = if !fore_wnd.0.is_null() {
            GetWindowThreadProcessId(fore_wnd, None)
        } else {
            0
        };

        // 三路 AttachThreadInput（AutoHotkey 风格）：
        // 1. 将当前线程附加到前台窗口的线程
        // 2. 将前台窗口的线程附加到目标窗口的线程
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

        // 重试循环 - 最多 5 次尝试，类似 AutoHotkey
        let mut attempted_alt_up = false;

        for i in 0..5 {
            // 如果之前的尝试失败，在第一次重试时尝试 Alt-up（AutoHotkey 的 sTriedKeyUp 逻辑）
            if i == 1 && !attempted_alt_up {
                attempted_alt_up = true;
                // 发送 Alt-up 事件以解锁前台
                send_alt_up();
            }

            if SetForegroundWindow(hwnd).as_bool() {
                break;
            }

            // 尝试之间休眠（SLEEP_INTERVAL ~10ms）
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        // 分离线程输入（关键，避免下次出现问题）
        if attached_my_to_fore {
            let _ = AttachThreadInput(current_thread, fore_thread, false);
        }
        if attached_fore_to_target {
            let _ = AttachThreadInput(fore_thread, target_thread, false);
        }

        // 等待窗口完成处理输入
        WaitForInputIdle(hwnd, 5000);
    }

    // 通过检查前台窗口来验证激活成功
    unsafe {
        std::thread::sleep(std::time::Duration::from_millis(50));
        if GetForegroundWindow() == hwnd {
            Ok(true)
        } else {
            // 短暂延迟后再重试一次
            std::thread::sleep(std::time::Duration::from_millis(100));
            if GetForegroundWindow() == hwnd {
                Ok(true)
            } else {
                Ok(false) // 窗口被另一个应用抢走了
            }
        }
    }
}

/// 发送 Alt-up 事件以解锁前台锁
fn send_alt_up() {
    unsafe {
        let fore_wnd = GetForegroundWindow();
        if !fore_wnd.0.is_null() {
            // 发送 WM_KEYDOWN 然后 WM_KEYUP 表示 Alt（VK_MENU = 0x12）
            let _ = PostMessageW(fore_wnd, WM_KEYDOWN, WPARAM(VK_MENU as usize), LPARAM(0));
            let _ = PostMessageW(fore_wnd, WM_KEYUP, WPARAM(VK_MENU as usize), LPARAM(0));
        }
    }
}
