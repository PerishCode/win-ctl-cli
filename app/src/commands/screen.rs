use std::path::Path;

#[cfg(windows)]
pub fn capture(output_path: &Path) -> Result<(), String> {
    capture_windows(output_path, None)
}

#[cfg(windows)]
pub fn capture_with_format(output_path: &Path, format: &str) -> Result<(), String> {
    capture_windows(output_path, Some(format))
}

#[cfg(not(windows))]
pub fn capture(_output_path: &Path) -> Result<(), String> {
    Err(String::from("screen capture is only supported on Windows"))
}

#[cfg(not(windows))]
pub fn capture_with_format(_output_path: &Path, _format: &str) -> Result<(), String> {
    Err(String::from("screen capture is only supported on Windows"))
}

#[cfg(windows)]
fn capture_windows(output_path: &Path, format_override: Option<&str>) -> Result<(), String> {
    use image::{ImageBuffer, ImageFormat, Rgba};
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Foundation::HWND;
    use windows_sys::Win32::Graphics::Gdi::{
        BI_RGB, BITMAPINFO, BITMAPINFOHEADER, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC,
        DIB_RGB_COLORS, DeleteDC, DeleteObject, GetDC, GetDIBits, GetDeviceCaps, HBITMAP, HGDIOBJ,
        HORZRES, ReleaseDC, SRCCOPY, SelectObject, VERTRES,
    };

    let output_wide: Vec<u16> = output_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    if output_wide.len() <= 1 {
        return Err(String::from("output path is required"));
    }

    let format = match format_override {
        Some(format) => parse_format(format)?,
        None => format_from_path(output_path)?,
    };

    let screen_dc = unsafe { GetDC(0 as HWND) };
    if screen_dc.is_null() {
        return Err(String::from("failed to access screen"));
    }

    let width = unsafe { GetDeviceCaps(screen_dc, HORZRES as i32) };
    let height = unsafe { GetDeviceCaps(screen_dc, VERTRES as i32) };
    if width <= 0 || height <= 0 {
        unsafe { ReleaseDC(0 as HWND, screen_dc) };
        return Err(String::from("failed to read screen size"));
    }

    let mem_dc = unsafe { CreateCompatibleDC(screen_dc) };
    if mem_dc.is_null() {
        unsafe { ReleaseDC(0 as HWND, screen_dc) };
        return Err(String::from("failed to create memory device context"));
    }

    let bitmap = unsafe { CreateCompatibleBitmap(screen_dc, width, height) };
    if bitmap.is_null() {
        unsafe {
            DeleteDC(mem_dc);
            ReleaseDC(0 as HWND, screen_dc);
        }
        return Err(String::from("failed to create bitmap"));
    }

    let old_object = unsafe { SelectObject(mem_dc, bitmap as HGDIOBJ) };
    if old_object.is_null() {
        unsafe {
            DeleteObject(bitmap as HGDIOBJ);
            DeleteDC(mem_dc);
            ReleaseDC(0 as HWND, screen_dc);
        }
        return Err(String::from("failed to select bitmap"));
    }

    let blt_ok = unsafe { BitBlt(mem_dc, 0, 0, width, height, screen_dc, 0, 0, SRCCOPY) };
    if blt_ok == 0 {
        unsafe {
            SelectObject(mem_dc, old_object);
            DeleteObject(bitmap as HGDIOBJ);
            DeleteDC(mem_dc);
            ReleaseDC(0 as HWND, screen_dc);
        }
        return Err(String::from("failed to copy screen pixels"));
    }

    let mut info = BITMAPINFO {
        bmiHeader: BITMAPINFOHEADER {
            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: [windows_sys::Win32::Graphics::Gdi::RGBQUAD {
            rgbBlue: 0,
            rgbGreen: 0,
            rgbRed: 0,
            rgbReserved: 0,
        }],
    };

    let image_size = (width as usize) * (height as usize) * 4;
    let mut pixels = vec![0u8; image_size];
    let got_bits = unsafe {
        GetDIBits(
            mem_dc,
            bitmap as HBITMAP,
            0,
            height as u32,
            pixels.as_mut_ptr() as *mut _,
            &mut info,
            DIB_RGB_COLORS,
        )
    };
    if got_bits == 0 {
        unsafe {
            SelectObject(mem_dc, old_object);
            DeleteObject(bitmap as HGDIOBJ);
            DeleteDC(mem_dc);
            ReleaseDC(0 as HWND, screen_dc);
        }
        return Err(String::from("failed to read bitmap data"));
    }

    for pixel in pixels.chunks_exact_mut(4) {
        pixel.swap(0, 2);
        pixel[3] = 255;
    }

    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(width as u32, height as u32, pixels)
        .ok_or_else(|| String::from("failed to build image buffer"))?;
    let write_result = match format {
        ImageFormat::Png => image.save_with_format(output_path, ImageFormat::Png),
        ImageFormat::Bmp => image.save_with_format(output_path, ImageFormat::Bmp),
        _ => unreachable!(),
    };

    unsafe {
        SelectObject(mem_dc, old_object);
        DeleteObject(bitmap as HGDIOBJ);
        DeleteDC(mem_dc);
        ReleaseDC(0 as HWND, screen_dc);
    }

    write_result.map_err(|err| err.to_string())?;
    Ok(())
}

#[cfg(windows)]
fn parse_format(value: &str) -> Result<image::ImageFormat, String> {
    match value.to_ascii_lowercase().as_str() {
        "png" => Ok(image::ImageFormat::Png),
        "bmp" => Ok(image::ImageFormat::Bmp),
        _ => Err(String::from("unsupported format: use png or bmp")),
    }
}

#[cfg(windows)]
fn format_from_path(path: &Path) -> Result<image::ImageFormat, String> {
    let ext = path
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| String::from("unsupported output extension: use .png or .bmp"))?;
    match ext.to_ascii_lowercase().as_str() {
        "png" => Ok(image::ImageFormat::Png),
        "bmp" => Ok(image::ImageFormat::Bmp),
        _ => Err(String::from(
            "unsupported output extension: use .png or .bmp",
        )),
    }
}
