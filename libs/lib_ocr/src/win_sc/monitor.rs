use image::DynamicImage;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

use crate::win_sc::{create_capture_item, init, take_sc, Handle, error, ImageMode};

use super::ImageResource;

pub fn monitor_sc(rect: Option<&RECT>) -> error::Result<ImageResource> {
    init();

    let main_monitor_handle =
        unsafe { MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY) };

    let monitor_capture_item = create_capture_item(Handle::HMONITOR(main_monitor_handle))?;

    let (width, height) = match monitor_capture_item.Size() {
        Ok(size) => (size.Width, size.Height),
        Err(error) => {
            return Err(error::WindowsCaptureError::DimensionNotFoundErr(error))
        }
    };

    let capture_rect = match rect {
        Some(window_rect) => RECT {
            left: window_rect.left,
            top: window_rect.top,
            right: window_rect.right,
            bottom: window_rect.bottom,
        },
        None => RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        },
    };

    take_sc(&monitor_capture_item, &capture_rect) 
}
