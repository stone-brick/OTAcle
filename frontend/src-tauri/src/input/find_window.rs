//! Window finding functionality
//!
//! Supports the following window specification formats:
//! - `(empty)` or `"A"` - Current foreground window
//! - `"id:<hwnd>"` - Direct HWND specification
//! - `"class:<classname>"` - Window class name
//! - `"pid:<pid>"` - Process ID
//! - `"exe:<process>"` - Process name (e.g., "notepad.exe")
//! - `"<title>"` - Window title (with TitleMatchMode)

use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, FindWindowW, GetClassNameW, GetWindowTextW, GetWindowThreadProcessId,
    IsWindowVisible,
};

/// Window information for frontend display
#[derive(Debug, Clone, serde::Serialize)]
pub struct WindowInfo {
    pub hwnd: i64,
    pub title: String,
    pub class_name: String,
    pub process_name: String,
    pub pid: u32,
    pub is_visible: bool,
}

/// Title match mode for window title search
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TitleMatchMode {
    Prefix = 1,  // Title starts with the specified text (default)
    Contains = 2, // Title contains the specified text
}

impl Default for TitleMatchMode {
    fn default() -> Self {
        TitleMatchMode::Prefix
    }
}

/// Window search criteria
#[derive(Debug, Clone)]
pub struct WindowSearch {
    /// Window title to match
    pub title: Option<String>,
    /// Window class name to match (class:)
    pub class_name: Option<String>,
    /// Direct HWND to validate (id:)
    pub hwnd: Option<isize>,
    /// Process ID to match (pid:)
    pub pid: Option<u32>,
    /// Process name to match (exe:), e.g., "notepad.exe"
    pub exe_name: Option<String>,
    /// Window title to exclude
    pub exclude_title: Option<String>,
    /// Title match mode
    pub match_mode: TitleMatchMode,
    /// Whether to detect hidden windows
    pub detect_hidden: bool,
}

impl Default for WindowSearch {
    fn default() -> Self {
        Self {
            title: None,
            class_name: None,
            hwnd: None,
            pid: None,
            exe_name: None,
            exclude_title: None,
            match_mode: TitleMatchMode::Prefix,
            detect_hidden: true,
        }
    }
}

/// Parse a window specification string into WindowSearch
///
/// Supported formats:
/// - `"A"` - Current foreground window
/// - `"id:0x12345"` or `"id:12345"` - Direct HWND
/// - `"class:Notepad"` - Window class name
/// - `"pid:1234"` - Process ID
/// - `"exe:notepad.exe"` - Process name
/// - `"Untitled - Notepad"` - Window title
pub fn parse_window_spec(spec: &str) -> WindowSearch {
    let spec = spec.trim();

    // Handle "A" or empty - means foreground window
    if spec.is_empty() || spec.eq_ignore_ascii_case("A") {
        return WindowSearch::default();
    }

    // Check for id:
    if spec.to_lowercase().starts_with("id:") {
        let value = spec[3..].trim();
        if let Ok(hwnd) = parse_hwnd(value) {
            return WindowSearch {
                hwnd: Some(hwnd),
                ..Default::default()
            };
        }
    }

    // Check for plain HWND (pure number, e.g., "395542" or "0x60916")
    if let Ok(hwnd) = parse_hwnd(spec) {
        return WindowSearch {
            hwnd: Some(hwnd),
            ..Default::default()
        };
    }

    // Check for class:
    if spec.to_lowercase().starts_with("class:") {
        let class_name = spec[6..].trim().to_string();
        return WindowSearch {
            class_name: Some(class_name),
            ..Default::default()
        };
    }

    // Check for pid:
    if spec.to_lowercase().starts_with("pid:") {
        let value = spec[4..].trim();
        if let Ok(pid) = value.parse::<u32>() {
            return WindowSearch {
                pid: Some(pid),
                ..Default::default()
            };
        }
    }

    // Check for exe:
    if spec.to_lowercase().starts_with("exe:") {
        let exe_name = spec[4..].trim().to_string();
        return WindowSearch {
            exe_name: Some(exe_name),
            ..Default::default()
        };
    }

    // Otherwise, treat as window title
    WindowSearch {
        title: Some(spec.to_string()),
        ..Default::default()
    }
}

/// Parse a string to HWND (supports hex 0x12345 or decimal 12345)
fn parse_hwnd(s: &str) -> Result<isize, std::num::ParseIntError> {
    let s = s.trim();
    if s.starts_with("0x") || s.starts_with("0X") {
        isize::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<isize>()
    }
}

/// Get window title as a String
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

/// Get window class name as a String
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

/// Get process name from HWND
pub fn get_process_name(hwnd: HWND) -> Option<String> {
    unsafe {
        // Get process ID from window
        let mut pid: u32 = 0;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));

        // Open the process
        let process = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => handle,
            Err(_) => return None,
        };

        // Get the executable path using QueryFullProcessImageNameW
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
            // Extract just the filename (basename)
            if let Some(name) = path.rsplit('\\').next() {
                return Some(name.to_lowercase());
            }
        }

        None
    }
}

/// Check if title matches using the specified mode
fn title_matches(window_title: &str, criteria: &str, mode: &TitleMatchMode) -> bool {
    match mode {
        TitleMatchMode::Prefix => window_title.starts_with(criteria),
        TitleMatchMode::Contains => window_title.contains(criteria),
    }
}

/// Check if a window matches the search criteria
fn window_matches(hwnd: HWND, search: &WindowSearch) -> bool {
    // Check if window exists and is valid
    if !search.detect_hidden {
        unsafe {
            if !IsWindowVisible(hwnd).as_bool() {
                return false;
            }
        }
    }

    // Check exclude_title
    if let Some(ref exclude) = search.exclude_title {
        if let Some(title) = get_window_title(hwnd) {
            if title_matches(&title, exclude, &TitleMatchMode::Contains) {
                return false;
            }
        }
    }

    // Check by HWND directly
    if let Some(target_hwnd) = search.hwnd {
        return HWND(target_hwnd as *mut std::ffi::c_void) == hwnd;
    }

    // Check by PID
    if let Some(target_pid) = search.pid {
        unsafe {
            let mut pid: u32 = 0;
            GetWindowThreadProcessId(hwnd, Some(&mut pid));
            if pid != target_pid {
                return false;
            }
        }
    }

    // Check by process name (exe:)
    if let Some(ref exe_name) = search.exe_name {
        if let Some(process_name) = get_process_name(hwnd) {
            // exe_name might be "notepad.exe" or "notepad", handle both
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

    // Check by class name
    if let Some(ref class) = search.class_name {
        if let Some(window_class) = get_window_class(hwnd) {
            if !window_class.eq_ignore_ascii_case(class) {
                return false;
            }
        } else {
            return false;
        }
    }

    // Check by title
    if let Some(ref title) = search.title {
        if let Some(window_title) = get_window_title(hwnd) {
            return title_matches(&window_title, title, &search.match_mode);
        } else {
            return false;
        }
    }

    true
}

// Callback context for enumeration
struct EnumContext {
    search: WindowSearch,
    result: isize,
}

impl EnumContext {
    fn new(search: WindowSearch) -> Self {
        Self { search, result: 0 }
    }
}

/// Find the first window matching the search criteria
pub fn find_window(search: &WindowSearch) -> Option<isize> {
    // Handle special cases

    // "A" or empty - return None, callers should use GetForegroundWindow directly
    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.exe_name.is_none()
    {
        return None;
    }

    // Direct HWND specification
    if let Some(hwnd_val) = search.hwnd {
        let hwnd = HWND(hwnd_val as *mut std::ffi::c_void);
        if window_matches(hwnd, search) {
            return Some(hwnd_val);
        }
        return None;
    }

    // Class name - use FindWindow for fast path
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
            // Found a match - store it and stop enumeration
            ctx.result = hwnd.0 as isize;
            return BOOL(0); // Stop enumeration
        }
        BOOL(1) // Continue enumeration
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
// Dedicated window search functions - each search type has its own explicit API
// ============================================================================

/// Find all windows matching a title (prefix match)
pub fn find_windows_by_title(title: &str) -> Vec<isize> {
    let search = WindowSearch {
        title: Some(title.to_string()),
        match_mode: TitleMatchMode::Prefix,
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// Find all windows matching a title (contains match)
pub fn find_windows_by_title_contains(title: &str) -> Vec<isize> {
    let search = WindowSearch {
        title: Some(title.to_string()),
        match_mode: TitleMatchMode::Contains,
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// Find window by exact class name
pub fn find_window_by_class_name(class_name: &str) -> Option<isize> {
    let search = WindowSearch {
        class_name: Some(class_name.to_string()),
        ..Default::default()
    };
    find_window(&search)
}

/// Find all windows belonging to a process ID
pub fn find_windows_by_pid(pid: u32) -> Vec<isize> {
    let search = WindowSearch {
        pid: Some(pid),
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// Find all windows for a process by executable name
pub fn find_windows_by_exe(exe_name: &str) -> Vec<isize> {
    let exe_name = if exe_name.to_lowercase().ends_with(".exe") {
        exe_name.to_string()
    } else {
        format!("{}.exe", exe_name.to_lowercase())
    };
    let search = WindowSearch {
        exe_name: Some(exe_name),
        ..Default::default()
    };
    find_all_matching_windows(&search)
}

/// Find window by exact HWND
pub fn find_window_by_hwnd(hwnd: isize) -> Option<isize> {
    let hwnd_check = HWND(hwnd as *mut std::ffi::c_void);
    // Verify the HWND is valid
    if hwnd_check.0.is_null() {
        return None;
    }
    // Verify it's a valid window by checking if it has a title
    if get_window_title(hwnd_check).is_none() {
        return None;
    }
    Some(hwnd)
}

// Context for finding all matching windows
struct FindAllContext {
    search: WindowSearch,
    results: Vec<isize>,
}

/// Internal helper to find all windows matching a search criteria
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

        BOOL(1) // Continue enumeration
    }

    unsafe {
        let lparam = LPARAM(&mut ctx as *mut FindAllContext as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    ctx.results
}

/// Get detailed information about a window
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

/// List all top-level windows
pub fn list_windows() -> Vec<WindowInfo> {
    let mut windows = Vec::new();

    unsafe extern "system" fn enum_callback(hwnd: HWND, lparam: LPARAM) -> BOOL {
        let windows_ptr = lparam.0 as *mut Vec<WindowInfo>;
        let windows = &mut *windows_ptr;

        // Skip invisible windows
        if !IsWindowVisible(hwnd).as_bool() {
            return BOOL(1);
        }

        // Skip windows without titles (typically hidden or system windows)
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

        BOOL(1) // Continue enumeration
    }

    unsafe {
        let lparam = LPARAM(&mut windows as *mut Vec<WindowInfo> as isize);
        let _ = EnumWindows(Some(enum_callback), lparam);
    }

    // Sort by title for easier browsing
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
