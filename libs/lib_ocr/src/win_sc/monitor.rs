use image::DynamicImage;
use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;

use crate::win_sc::{create_capture_item, init, take_sc, Handle};

pub fn monitor_sc(rect: Option<&RECT>) -> DynamicImage {
    init();

    let main_monitor_handle =
        unsafe { MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY) };

    let monitor_capture_item = create_capture_item(Handle::HMONITOR(main_monitor_handle)).unwrap();

    let (width, height) = match monitor_capture_item.Size() {
        Ok(size) => (size.Width, size.Height),
        Err(error) => {
            panic!("Failed to get capture item size: {:?}", error);
        }
    };

    // Either capture subregion or entire screen
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

    match take_sc(&monitor_capture_item, &capture_rect) {
        Ok(dynamic_image) => dynamic_image,
        Err(error) => panic!("Failed to take screenshot: {:?}", error)
    }
}
