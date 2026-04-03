use crate::core::RuntimeConfig;
use std::fmt::Write as _;

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub hwnd: isize,
    pub title: String,
    pub class_name: String,
    pub pid: u32,
}

impl WindowInfo {
    fn is_useful(&self) -> bool {
        !self.title.trim().is_empty()
    }
}

pub struct WindowBounds {
    pub hwnd: isize,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, serde::Serialize)]
pub struct ActiveWindowBounds {
    pub hwnd: isize,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
}

pub fn list(_runtime: &RuntimeConfig, json: bool) -> Result<(), String> {
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

pub fn active(
    _runtime: &RuntimeConfig,
    json: bool,
    bounds: bool,
    class: bool,
    pid: bool,
) -> Result<(), String> {
    let window = active_window()?.ok_or_else(|| String::from("no active window available"))?;
    if !window.is_useful() {
        return Err(String::from("no active window available"));
    }

    if class {
        println!("{}", window.class_name);
        return Ok(());
    }

    if pid {
        println!("{}", window.pid);
        return Ok(());
    }

    if bounds {
        let bounds = window_bounds(window.hwnd)?;
        if json {
            print_bounds_json(&bounds);
        } else {
            print_bounds_text(std::slice::from_ref(&bounds));
        }
    } else {
        if json {
            print_json(std::slice::from_ref(&window));
        } else {
            print_text(std::slice::from_ref(&window));
        }
    }

    Ok(())
}

pub fn bounds(_runtime: &RuntimeConfig, hwnd: isize, json: bool) -> Result<(), String> {
    let bounds = window_bounds(hwnd)?;
    if json {
        print_bounds_json(&bounds);
    } else {
        print_bounds_text(std::slice::from_ref(&bounds));
    }
    Ok(())
}

pub fn window_bounds_for_compose(
    _runtime: &RuntimeConfig,
    hwnd: isize,
) -> Result<ActiveWindowBounds, String> {
    let bounds = window_bounds(hwnd)?;
    Ok(ActiveWindowBounds {
        hwnd: bounds.hwnd,
        left: bounds.left,
        top: bounds.top,
        width: bounds.width,
        height: bounds.height,
    })
}

pub fn active_bounds(_runtime: &RuntimeConfig) -> Result<ActiveWindowBounds, String> {
    let window = active_window()?.ok_or_else(|| String::from("no active window available"))?;
    if !window.is_useful() {
        return Err(String::from("no active window available"));
    }

    let bounds = window_bounds(window.hwnd)?;
    Ok(ActiveWindowBounds {
        hwnd: bounds.hwnd,
        left: bounds.left,
        top: bounds.top,
        width: bounds.width,
        height: bounds.height,
    })
}

pub fn active_class(_runtime: &RuntimeConfig) -> Result<String, String> {
    let window = active_window()?.ok_or_else(|| String::from("no active window available"))?;
    if !window.is_useful() {
        return Err(String::from("no active window available"));
    }

    Ok(window.class_name)
}

pub fn active_pid(_runtime: &RuntimeConfig) -> Result<u32, String> {
    let window = active_window()?.ok_or_else(|| String::from("no active window available"))?;
    if !window.is_useful() {
        return Err(String::from("no active window available"));
    }

    Ok(window.pid)
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
        let class_name = OsString::from_wide(&class_buf[..class_len.max(0) as usize])
            .to_string_lossy()
            .into_owned();

        let mut pid = 0u32;
        unsafe {
            windows_sys::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId(hwnd, &mut pid)
        };

        windows.push(WindowInfo {
            hwnd: hwnd as isize,
            title,
            class_name,
            pid,
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

#[cfg(not(windows))]
fn collect_windows() -> Result<Vec<WindowInfo>, String> {
    Err(String::from(
        "window commands are only supported on Windows",
    ))
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
    let class_name = OsString::from_wide(&class_buf[..class_len.max(0) as usize])
        .to_string_lossy()
        .into_owned();

    let mut pid = 0u32;
    unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::GetWindowThreadProcessId(hwnd, &mut pid)
    };

    Ok(Some(WindowInfo {
        hwnd: hwnd as isize,
        title,
        class_name,
        pid,
    }))
}

#[cfg(not(windows))]
fn active_window() -> Result<Option<WindowInfo>, String> {
    Err(String::from(
        "window commands are only supported on Windows",
    ))
}

#[cfg(windows)]
fn window_bounds(hwnd: isize) -> Result<WindowBounds, String> {
    use windows_sys::Win32::Foundation::RECT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetWindowRect;

    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    if unsafe { GetWindowRect(hwnd as _, &mut rect) } == 0 {
        return Err(String::from("failed to read window bounds"));
    }

    Ok(WindowBounds {
        hwnd,
        left: rect.left,
        top: rect.top,
        width: rect.right - rect.left,
        height: rect.bottom - rect.top,
    })
}

#[cfg(not(windows))]
fn window_bounds(_hwnd: isize) -> Result<WindowBounds, String> {
    Err(String::from(
        "window commands are only supported on Windows",
    ))
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

fn print_bounds_text(bounds: &[WindowBounds]) {
    for value in bounds {
        println!(
            "{} {} {} {}",
            value.left, value.top, value.width, value.height
        );
    }
}

fn print_bounds_json(bounds: &WindowBounds) {
    println!(
        "{{\"hwnd\":{},\"left\":{},\"top\":{},\"width\":{},\"height\":{}}}",
        bounds.hwnd, bounds.left, bounds.top, bounds.width, bounds.height
    );
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
