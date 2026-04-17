//! 窗口查找功能
//!
//! 支持以下窗口规格格式：
//! - `(空)` 或 `"A"` - 当前前台窗口
//! - `"id:<hwnd>"` - 直接 HWND 规格
//! - `"class:<classname>"` - 窗口类名
//! - `"pid:<pid>"` - 进程 ID
//! - `"exe:<process>"` - 进程名（例如 "notepad.exe"）
//! - `"<title>"` - 窗口标题（带 TitleMatchMode）

use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowW, GetClassNameW, GetWindowTextW, GetWindowThreadProcessId,
    IsWindowVisible,
};

/// 用于前端显示的窗口信息
#[derive(Debug, Clone, serde::Serialize)]
pub struct WindowInfo {
    pub hwnd: i64,
    pub title: String,
    pub class_name: String,
    pub process_name: String,
    pub pid: u32,
    pub is_visible: bool,
}

/// 窗口标题搜索的标题匹配模式
#[derive(Debug, Clone, Copy, PartialEq)]
#[derive(Default)]
pub enum TitleMatchMode {
    #[default]
    Prefix = 1,  // 标题以指定文本开头（默认）
    Contains = 2, // 标题包含指定文本
}


/// 窗口搜索条件
#[derive(Debug, Clone)]
pub struct WindowSearch {
    /// 要匹配的窗口标题
    pub title: Option<String>,
    /// 要匹配的窗口类名（class:）
    pub class_name: Option<String>,
    /// 要验证的直接 HWND（id:）
    pub hwnd: Option<isize>,
    /// 要匹配的进程 ID（pid:）
    pub pid: Option<u32>,
    /// 要匹配的进程名（exe:），例如 "notepad.exe"
    pub process_name: Option<String>,
    /// 要排除的窗口标题
    pub exclude_title: Option<String>,
    /// 标题匹配模式
    pub match_mode: TitleMatchMode,
    /// 是否检测隐藏窗口
    pub detect_hidden: bool,
}

impl Default for WindowSearch {
    fn default() -> Self {
        Self {
            title: None,
            class_name: None,
            hwnd: None,
            pid: None,
            process_name: None,
            exclude_title: None,
            match_mode: TitleMatchMode::Prefix,
            detect_hidden: true,
        }
    }
}

/// 将窗口规格字符串解析为 WindowSearch
///
/// 支持的格式：
/// - `"A"` - 当前前台窗口
/// - `"id:0x12345"` 或 `"id:12345"` - 直接 HWND
/// - `"class:Notepad"` - 窗口类名
/// - `"pid:1234"` - 进程 ID
/// - `"exe:notepad.exe"` - 进程名
/// - `"Untitled - Notepad"` - 窗口标题
pub fn parse_window_spec(spec: &str) -> WindowSearch {
    let spec = spec.trim();

    // 处理 "A" 或空 - 表示前台窗口
    if spec.is_empty() || spec.eq_ignore_ascii_case("A") {
        return WindowSearch::default();
    }

    // 检查 id:
    if spec.to_lowercase().starts_with("id:") {
        let value = spec[3..].trim();
        if let Ok(hwnd) = parse_hwnd(value) {
            return WindowSearch {
                hwnd: Some(hwnd),
                ..Default::default()
            };
        }
    }

    // 检查纯 HWND（纯数字，例如 "395542" 或 "0x60916"）
    if let Ok(hwnd) = parse_hwnd(spec) {
        return WindowSearch {
            hwnd: Some(hwnd),
            ..Default::default()
        };
    }

    // 检查 class:
    if spec.to_lowercase().starts_with("class:") {
        let class_name = spec[6..].trim().to_string();
        return WindowSearch {
            class_name: Some(class_name),
            ..Default::default()
        };
    }

    // 检查 pid:
    if spec.to_lowercase().starts_with("pid:") {
        let value = spec[4..].trim();
        if let Ok(pid) = value.parse::<u32>() {
            return WindowSearch {
                pid: Some(pid),
                ..Default::default()
            };
        }
    }

    // 检查 exe:
    if spec.to_lowercase().starts_with("exe:") {
        let exe_name = spec[4..].trim().to_string();
        return WindowSearch {
            process_name: Some(exe_name),
            ..Default::default()
        };
    }

    // 否则，作为窗口标题处理
    WindowSearch {
        title: Some(spec.to_string()),
        ..Default::default()
    }
}

/// 将字符串解析为 HWND（支持十六进制 0x12345 或十进制 12345）
fn parse_hwnd(s: &str) -> Result<isize, std::num::ParseIntError> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        isize::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<isize>()
    }
}

/// 获取窗口标题作为字符串
pub fn get_window_title(hwnd: HWND) -> Option<String> {
    unsafe {
        let mut buffer = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buffer);
        if len > 0 {
            Some(String::from_utf16_lossy(&buffer[..len as usize]))
        } else {
            None
        }
    }
}

/// 获取窗口类名作为字符串
pub fn get_window_class(hwnd: HWND) -> Option<String> {
    unsafe {
        let mut buffer = [0u16; 256];
        let len = GetClassNameW(hwnd, &mut buffer);
        if len > 0 {
            Some(String::from_utf16_lossy(&buffer[..len as usize]))
        } else {
            None
        }
    }
}

/// 从 HWND 获取进程名
pub fn get_process_name(hwnd: HWND) -> Option<String> {
    unsafe {
        // 从窗口获取进程 ID
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // 打开进程
        let process = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => handle,
            Err(_) => return None,
        };

        // 使用 QueryFullProcessImageNameW 获取可执行文件路径
        let mut exe_name = [0u16; 260];
        let mut size = exe_name.len() as u32;

        let result = windows::Win32::System::Threading::QueryFullProcessImageNameW(
            process,
            windows::Win32::System::Threading::PROCESS_NAME_FORMAT(0),
            windows::core::PWSTR(exe_name.as_mut_ptr()),
            &mut size,
        );

        if result.is_ok() && size > 0 {
            let path = String::from_utf16_lossy(&exe_name[..size as usize]);
            // 只提取文件名（basename）
            if let Some(name) = path.rsplit('\\').next() {
                return Some(name.to_lowercase());
            }
        }

        None
    }
}

/// 使用指定模式检查标题是否匹配
fn title_matches(window_title: &str, criteria: &str, mode: &TitleMatchMode) -> bool {
    match mode {
        TitleMatchMode::Prefix => window_title.starts_with(criteria),
        TitleMatchMode::Contains => window_title.contains(criteria),
    }
}

/// 检查窗口是否匹配搜索条件
fn window_matches(hwnd: HWND, search: &WindowSearch) -> bool {
    // 检查窗口是否存在且有效
    if !search.detect_hidden {
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() {
                return false;
            }
        }
    }

    // 检查 exclude_title
    if let Some(ref exclude) = search.exclude_title {
        if let Some(title) = get_window_title(hwnd) {
            if title_matches(&title, exclude, &TitleMatchMode::Contains) {
                return false;
            }
        }
    }

    // 直接通过 HWND 检查
    if let Some(target_hwnd) = search.hwnd {
        return HWND(target_hwnd as *mut std::ffi::c_void) == hwnd;
    }

    // 通过 PID 检查
    if let Some(target_pid) = search.pid {
        unsafe {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid != target_pid {
                return false;
            }
        }
    }

    // 通过进程名检查（exe:）
    if let Some(ref exe_name) = search.process_name {
        if let Some(process_name) = get_process_name(hwnd) {
            // exe_name 可能是 "notepad.exe" 或 "notepad"，处理两种情况
            let exe_lower = exe_name.to_lowercase();
            let target = if exe_lower.ends_with(".exe") {
                exe_lower
            } else {
                format!("{}.exe", exe_lower)
            };
            if process_name != target {
                return false;
            }
        } else {
            return false;
        }
    }

    // 通过类名检查
    if let Some(ref class) = search.class_name {
        if let Some(window_class) = get_window_class(hwnd) {
            if !window_class.eq_ignore_ascii_case(class) {
                return false;
            }
        } else {
            return false;
        }
    }

    // 通过标题检查
    if let Some(ref title) = search.title {
        if let Some(window_title) = get_window_title(hwnd) {
            return title_matches(&window_title, title, &search.match_mode);
        } else {
            return false;
        }
    }

    true
}

// 用于枚举的回调上下文
struct EnumContext {
    search: WindowSearch,
    result: isize,
}

impl EnumContext {
    fn new(search: WindowSearch) -> Self {
        Self { search, result: 0 }
    }
}

/// 查找第一个匹配搜索条件的窗口
pub fn find_window(search: &WindowSearch) -> Option<isize> {
    // 处理特殊情况

    // "A" 或空 - 返回 None，调用者应直接使用 GetForegroundWindow
    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.process_name.is_none()
    {
        return None;
    }

    // 直接 HWND 规格
    if let Some(hwnd_val) = search.hwnd {
        let hwnd = HWND(hwnd_val as *mut std::ffi::c_void);
        if window_matches(hwnd, search) {
            return Some(hwnd_val);
        }
        return None;
    }

    // 类名 - 使用 FindWindow 进行快速路径
    if let Some(ref class) = search.class_name {
        let wide_class: Vec<u16> = class.encode_utf16().chain(std::iter::once(0)).collect();
        unsafe {
            if let Ok(hwnd) = FindWindowW(None, windows::core::PCWSTR(wide_class.as_ptr())) {
                if window_matches(hwnd, search) {
                    return Some(hwnd.0 as isize);
                }
            }
        }
    }

    // Enumerate all windows to find match
    let mut ctx = EnumContext::new(search.clone());

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx_ptr = lparam.0 as *mut EnumContext;
        let ctx = &mut *ctx_ptr;

        if window_matches(hwnd, &ctx.search) {
            // 找到匹配 - 存储它并停止枚举
            ctx.result = hwnd.0 as isize;
            return BOOL(0); // 停止枚举
        }
        BOOL(1) // 继续枚举
    }

    unsafe {
        let lparam = LPARAM(&mut ctx as *mut EnumContext as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    if ctx.result == 0 {
        None
    } else {
        Some(ctx.result)
    }
}

// ============================================================================
// 专用窗口搜索函数 - 每种搜索类型都有自己明确的 API
// ============================================================================

/// 查找所有匹配标题的窗口（前缀匹配）
pub fn find_windows_by_title(title: &str) -> Vec<isize> {
    let search = WindowSearch {
        title: Some(title.to_string()),
        match_mode: TitleMatchMode::Prefix,
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// 查找所有匹配标题的窗口（包含匹配）
pub fn find_windows_by_title_contains(title: &str) -> Vec<isize> {
    let search = WindowSearch {
        title: Some(title.to_string()),
        match_mode: TitleMatchMode::Contains,
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// 通过精确类名查找窗口
pub fn find_window_by_class_name(class_name: &str) -> Option<isize> {
    let search = WindowSearch {
        class_name: Some(class_name.to_string()),
        ..Default::default()
    };
    find_window(&search)
}

/// 查找属于指定进程 ID 的所有窗口
pub fn find_windows_by_pid(pid: u32) -> Vec<isize> {
    let search = WindowSearch {
        pid: Some(pid),
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// 通过可执行文件名查找进程的所有窗口
pub fn find_windows_by_exe(process_name: &str) -> Vec<isize> {
    let process_name = if process_name.to_lowercase().ends_with(".exe") {
        process_name.to_string()
    } else {
        format!("{}.exe", process_name.to_lowercase())
    };
    let search = WindowSearch {
        process_name: Some(process_name),
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// 通过精确 HWND 查找窗口
pub fn find_window_by_hwnd(hwnd: isize) -> Option<isize> {
    let hwnd_check = HWND(hwnd as *mut std::ffi::c_void);
    // 验证 HWND 是否有效
    if hwnd_check.0.is_null() {
        return None;
    }
    // 通过检查是否有标题来验证是否是有效窗口
    get_window_title(hwnd_check)?;
    Some(hwnd)
}

// 用于查找所有匹配窗口的上下文
struct FindAllContext {
    search: WindowSearch,
    results: Vec<isize>,
}

/// 内部辅助函数，查找所有匹配搜索条件的窗口
fn find_all_matching_windows(search: &WindowSearch) -> Vec<isize> {
    let mut ctx = FindAllContext {
        search: search.clone(),
        results: Vec::new(),
    };

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let ctx_ptr = lparam.0 as *mut FindAllContext;
        let ctx = &mut *ctx_ptr;

        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        if window_matches(hwnd, &ctx.search) {
            ctx.results.push(hwnd.0 as isize);
        }

        BOOL(1) // 继续枚举
    }

    unsafe {
        let lparam = LPARAM(&mut ctx as *mut FindAllContext as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    ctx.results
}

/// 获取窗口的详细信息
pub fn get_window_info(hwnd: isize) -> Option<WindowInfo> {
    let hwnd = HWND(hwnd as *mut std::ffi::c_void);

    unsafe {
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let title = get_window_title(hwnd).unwrap_or_default();
        let class_name = get_window_class(hwnd).unwrap_or_default();
        let process_name = get_process_name(hwnd).unwrap_or_default();
        let is_visible = IsWindowVisible(hwnd).as_bool();

        Some(WindowInfo {
            hwnd: hwnd.0 as i64,
            title,
            class_name,
            process_name,
            pid,
            is_visible,
        })
    }
}

/// 列出所有顶级窗口
pub fn list_windows() -> Vec<WindowInfo> {
    let mut windows = Vec::new();

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let windows_ptr = lparam.0 as *mut Vec<WindowInfo>;
        let windows = &mut *windows_ptr;

        // 跳过不可见窗口
        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        // 跳过没有标题的窗口（通常是隐藏或系统窗口）
        let title = get_window_title(hwnd);
        if title.is_none() || title.as_ref().map(|t| t.is_empty()).unwrap_or(true) {
            return BOOL(1);
        }

        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        let class_name = get_window_class(hwnd).unwrap_or_default();
        let process_name = get_process_name(hwnd).unwrap_or_default();
        let is_visible = IsWindowVisible(hwnd).as_bool();

        windows.push(WindowInfo {
            hwnd: hwnd.0 as i64,
            title: title.unwrap_or_default(),
            class_name,
            process_name,
            pid,
            is_visible,
        });

        BOOL(1) // 继续枚举
    }

    unsafe {
        let lparam = LPARAM(&mut windows as *mut Vec<WindowInfo> as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    // 按标题排序以便浏览
    windows.sort_by(|a: &WindowInfo, b: &WindowInfo| a.title.to_lowercase().cmp(&b.title.to_lowercase()));
    windows
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_window_spec_empty() {
        let search = parse_window_spec("");
        assert!(search.title.is_none());
        assert!(search.class_name.is_none());
        assert!(search.hwnd.is_none());
    }

    #[test]
    fn test_parse_window_spec_a() {
        let search = parse_window_spec("A");
        assert!(search.title.is_none());
    }

    #[test]
    fn test_parse_window_spec_id() {
        let search = parse_window_spec("id:0x12345");
        assert_eq!(search.hwnd, Some(0x12345));
    }

    #[test]
    fn test_parse_window_spec_class() {
        let search = parse_window_spec("class:Notepad");
        assert_eq!(search.class_name, Some("Notepad".to_string()));
    }

    #[test]
    fn test_parse_window_spec_title() {
        let search = parse_window_spec("Untitled - Notepad");
        assert_eq!(search.title, Some("Untitled - Notepad".to_string()));
    }
}
