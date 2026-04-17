use crate::input;
use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;

#[tauri::command]
pub fn window_find_by_title(title: String) -> Result<Option<i64>, String> {
    let search = input::parse_window_spec(&title);

    if search.title.is_none()
        && search.class_name.is_none()
        && search.hwnd.is_none()
        && search.pid.is_none()
        && search.process_name.is_none()
    {
        let hwnd = unsafe { GetForegroundWindow() };
        if hwnd.0.is_null() {
            return Ok(None);
        }
        return Ok(Some(hwnd.0 as i64));
    }

    Ok(input::find_window(&search).map(|h| h as i64))
}

#[tauri::command]
pub fn window_find_one(title: String) -> Result<Option<i64>, String> {
    let results = input::find_windows_by_title(&title);
    Ok(results.into_iter().next().map(|h| h as i64))
}

#[tauri::command]
pub fn window_find_many(title: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_title(&title).into_iter().map(|h| h as i64).collect())
}

#[tauri::command]
pub fn window_find_one_contains(title: String) -> Result<Option<i64>, String> {
    let results = input::find_windows_by_title_contains(&title);
    Ok(results.into_iter().next().map(|h| h as i64))
}

#[tauri::command]
pub fn window_find_many_contains(title: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_title_contains(&title).into_iter().map(|h| h as i64).collect())
}

#[tauri::command]
pub fn window_find_by_class(class_name: String) -> Result<Option<i64>, String> {
    Ok(input::find_window_by_class_name(&class_name).map(|h| h as i64))
}

#[tauri::command]
pub fn window_find_by_pid(pid: u32) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_pid(pid).into_iter().map(|h| h as i64).collect())
}

#[tauri::command]
pub fn window_find_by_exe(process_name: String) -> Result<Vec<i64>, String> {
    Ok(input::find_windows_by_exe(&process_name).into_iter().map(|h| h as i64).collect())
}

#[tauri::command]
pub fn window_find_by_hwnd(hwnd: i64) -> Result<Option<i64>, String> {
    Ok(input::find_window_by_hwnd(hwnd as isize).map(|h| h as i64))
}
