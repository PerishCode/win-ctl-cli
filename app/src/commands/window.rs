use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
}

impl WindowInfo {
    fn is_useful(&self) -> bool {
        !self.title.trim().is_empty()
    }
}

#[cfg(windows)]
pub fn list(json: bool) -> Result<(), String> {
    let windows = collect_windows()?
        .into_iter()
        .filter(WindowInfo::is_useful)
        .collect::<Vec<_>>();
    if json {
        print_json(&windows);
    } else {
        print_text(&windows);
    }
    Ok(())
}

#[cfg(not(windows))]
pub fn list(_json: bool) -> Result<(), String> {
    Err(String::from("window list is only supported on Windows"))
}

#[cfg(windows)]
pub fn active(json: bool) -> Result<(), String> {
    let window = active_window()?.ok_or_else(|| String::from("no active window available"))?;
    if !window.is_useful() {
        return Err(String::from("no active window available"));
    }

    if json {
        print_json(std::slice::from_ref(&window));
    } else {
        print_text(std::slice::from_ref(&window));
    }

    Ok(())
}

#[cfg(not(windows))]
pub fn active(_json: bool) -> Result<(), String> {
    Err(String::from("window active is only supported on Windows"))
}

#[cfg(windows)]
fn collect_windows() -> Result<Vec<WindowInfo>, String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        EnumWindows, GetClassNameW, GetWindowTextLengthW, GetWindowTextW, IsWindowVisible,
    };

    unsafe extern "system" fn enum_proc(hwnd: HWND, lparam: isize) -> i32 {
        let windows = unsafe { &mut *(lparam as *mut Vec<WindowInfo>) };
        if unsafe { IsWindowVisible(hwnd) } == 0 {
            return 1;
        }

        let title_len = unsafe { GetWindowTextLengthW(hwnd) };
        let mut title_buf = vec![0u16; (title_len.max(0) as usize) + 1];
        let title_len =
            unsafe { GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32) };
        let title = OsString::from_wide(&title_buf[..title_len.max(0) as usize])
            .to_string_lossy()
            .into_owned();

        let mut class_buf = vec![0u16; 256];
        let class_len =
            unsafe { GetClassNameW(hwnd, class_buf.as_mut_ptr(), class_buf.len() as i32) };
        let _class_name = OsString::from_wide(&class_buf[..class_len.max(0) as usize])
            .to_string_lossy()
            .into_owned();

        windows.push(WindowInfo {
            hwnd: hwnd as isize,
            title,
        });
        1
    }

    let mut windows = Vec::new();
    let ok = unsafe { EnumWindows(Some(enum_proc), &mut windows as *mut _ as isize) };
    if ok == 0 {
        return Err(String::from("failed to enumerate windows"));
    }

    Ok(windows)
}

#[cfg(windows)]
fn active_window() -> Result<Option<WindowInfo>, String> {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt;
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GetClassNameW, GetForegroundWindow, GetWindowTextLengthW, GetWindowTextW,
    };

    let hwnd = unsafe { GetForegroundWindow() };
    if hwnd.is_null() {
        return Ok(None);
    }

    let title_len = unsafe { GetWindowTextLengthW(hwnd) };
    let mut title_buf = vec![0u16; (title_len.max(0) as usize) + 1];
    let title_len = unsafe { GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32) };
    let title = OsString::from_wide(&title_buf[..title_len.max(0) as usize])
        .to_string_lossy()
        .into_owned();

    let mut class_buf = vec![0u16; 256];
    let class_len =
        unsafe { GetClassNameW(hwnd as HWND, class_buf.as_mut_ptr(), class_buf.len() as i32) };
    let _class_name = OsString::from_wide(&class_buf[..class_len.max(0) as usize])
        .to_string_lossy()
        .into_owned();

    Ok(Some(WindowInfo {
        hwnd: hwnd as isize,
        title,
    }))
}

fn print_text(windows: &[WindowInfo]) {
    for window in windows {
        println!("0x{:X}\t{}", window.hwnd, window.title);
    }
}

fn print_json(windows: &[WindowInfo]) {
    let mut out = String::from("[");
    for (index, window) in windows.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        let _ = write!(
            out,
            "{{\"hwnd\":{},\"title\":{}}}",
            window.hwnd,
            json_string(&window.title)
        );
    }
    out.push(']');
    println!("{out}");
}

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
