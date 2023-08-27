use windows::core::{ComInterface, IInspectable, Interface, Result, HSTRING};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::{
    Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem},
    DirectX::{Direct3D11::IDirect3DDevice, DirectXPixelFormat},
    Imaging::{BitmapAlphaMode, BitmapEncoder, BitmapPixelFormat},
    SizeInt32,
};
use windows::Storage::{CreationCollisionOption, FileAccessMode, StorageFolder};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Dwm::DwmGetWindowAttribute;

use windows::Win32::Graphics::Direct3D11::{
    ID3D11Resource, ID3D11Texture2D, D3D11_BIND_FLAG, D3D11_BOX, D3D11_CPU_ACCESS_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_RESOURCE_MISC_FLAG, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Gdi::{
    MonitorFromWindow, ScreenToClient, HMONITOR, MONITOR_DEFAULTTOPRIMARY,
};
use windows::Win32::Graphics::{
    Direct3D::{D3D_DRIVER_TYPE, D3D_DRIVER_TYPE_HARDWARE, D3D_DRIVER_TYPE_WARP},
    Direct3D11::{
        D3D11CreateDevice, ID3D11Device, D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        D3D11_CREATE_DEVICE_FLAG, D3D11_SDK_VERSION,
    },
    Dxgi::{IDXGIDevice, DXGI_ERROR_UNSUPPORTED},
};

use windows::Win32::System::WinRT::Direct3D11::{
    CreateDirect3D11DeviceFromDXGIDevice, IDirect3DDxgiInterfaceAccess,
};
use windows::Win32::System::WinRT::{
    Graphics::Capture::IGraphicsCaptureItemInterop, RoInitialize, RO_INIT_MULTITHREADED,
};
use windows_sys::Win32::UI::*;

use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::null_mut;
use std::sync::mpsc::channel;
use windows::Win32::Graphics::Dwm::DWMWA_EXTENDED_FRAME_BOUNDS;
use windows::Win32::UI::WindowsAndMessaging::GetDesktopWindow;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

pub fn get_d3d_interface_from_object<S: Interface + ComInterface, R: Interface + ComInterface>(
    object: &S,
) -> Result<R> {
    let access: IDirect3DDxgiInterfaceAccess = object.cast()?;
    let object = unsafe { access.GetInterface::<R>()? };
    Ok(object)
}

pub fn create_direct3d_device(d3d_device: &ID3D11Device) -> Result<IDirect3DDevice> {
    let dxgi_device: IDXGIDevice = d3d_device.cast()?;
    let inspectable = unsafe { CreateDirect3D11DeviceFromDXGIDevice(&dxgi_device)? };
    inspectable.cast()
}

enum Handle {
    HWND(HWND),
    HMONITOR(HMONITOR),
}

// The target of the capture, chosen with the picker control.
fn create_capture_item(handle: Handle) -> Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    match handle {
        Handle::HWND(window_handle) => unsafe { interop.CreateForWindow(window_handle) },
        Handle::HMONITOR(monitor_handle) => unsafe { interop.CreateForMonitor(monitor_handle) },
    }
}

fn create_d3d_device_with_type(
    driver_type: D3D_DRIVER_TYPE,
    flags: D3D11_CREATE_DEVICE_FLAG,
    device: *mut Option<ID3D11Device>,
) -> Result<()> {
    unsafe {
        D3D11CreateDevice(
            None,
            driver_type,
            None,
            flags,
            None,
            D3D11_SDK_VERSION as u32,
            Some(device),
            None,
            None,
        )
    }
}

pub fn create_d3d_device() -> Result<ID3D11Device> {
    let mut device = None;

    let mut result = create_d3d_device_with_type(
        D3D_DRIVER_TYPE_HARDWARE,
        D3D11_CREATE_DEVICE_BGRA_SUPPORT,
        &mut device,
    );

    if let Err(error) = &result {
        // Fallback to WARP if initialization failed with hardware driver
        // allows for rendering when d3d hard is not available
        if error.code() == DXGI_ERROR_UNSUPPORTED {
            result = create_d3d_device_with_type(
                D3D_DRIVER_TYPE_WARP,
                D3D11_CREATE_DEVICE_BGRA_SUPPORT,
                &mut device,
            );
        }
    }

    result?;
    Ok(device.unwrap())
}

fn save_as_image(bits: Vec<u8>, item_size: SizeInt32) -> Result<()> {
    // Create a file in the current directory
    let path = std::env::current_dir()
        .unwrap()
        .to_string_lossy()
        .to_string();

    let folder = StorageFolder::GetFolderFromPathAsync(&HSTRING::from(&path))?.get()?;

    let file = folder
        .CreateFileAsync(
            &HSTRING::from("screenshot.png"),
            CreationCollisionOption::ReplaceExisting,
        )?
        .get()?;

    // Open the file for writing and encode the image data into the file stream
    let stream = file.OpenAsync(FileAccessMode::ReadWrite)?.get()?;
    let encoder = BitmapEncoder::CreateAsync(BitmapEncoder::PngEncoderId()?, &stream)?.get()?;

    encoder.SetPixelData(
        BitmapPixelFormat::Bgra8,
        BitmapAlphaMode::Premultiplied,
        item_size.Width as u32,
        item_size.Height as u32,
        1.0,
        1.0,
        &bits,
    )?;

    encoder.FlushAsync()?.get()?;

    Ok(())
}

fn init() {
    unsafe {
        // Init windows runtime with multi-threaded concurrency model
        match RoInitialize(RO_INIT_MULTITHREADED) {
            Ok(_) => (),
            Err(error) => println!("Failed to initialize windows runtime: {:?}", error),
        }
    }
}

pub struct WindowRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

pub fn window_handle(window_title: &str) -> HWND {
    init();
    let window_name: String = String::from(window_title) + "\0";
    return unsafe {
        match HWND(WindowsAndMessaging::FindWindowA(
            null_mut(),
            window_name.as_ptr(),
        )) {
            HWND(0) => panic!("Failed to find window"),
            handle => handle,
        }
    };
} 

pub fn get_window_rect(window_handle: HWND) -> RECT {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    unsafe {
        match DwmGetWindowAttribute(
            window_handle,
            DWMWA_EXTENDED_FRAME_BOUNDS,
            &mut rect as *mut RECT as *mut c_void,
            size_of::<RECT>() as u32,
        ) {
            Ok(_) => (),
            Err(error) => println!("Failed to get window rect: {:?}", error),
        }

        println!("rect: {:?}", rect);
    }

    rect
}

pub fn window_sc(window_title: &str, rect: Option<&WindowRect>) {
    let window_handle = window_handle(window_title);

    let capture_rect = match rect {
        Some(window_rect) => RECT {
            left: window_rect.left,
            top: window_rect.top,
            right: window_rect.right,
            bottom: window_rect.bottom,
        },
        None => {
            let rect = get_window_rect(window_handle);
            rect
        }
    };

    let window_capture_item = create_capture_item(Handle::HWND(window_handle)).unwrap();

    match take_sc(&window_capture_item, &capture_rect) {
        Ok(_) => println!("Screenshot taken"),
        Err(error) => println!("Failed to take screenshot: {:?}", error),
    };
}

pub fn monitor_sc(rect: Option<&RECT>) {
    init();
    
    let main_monitor_handle =
    unsafe { MonitorFromWindow(GetDesktopWindow(), MONITOR_DEFAULTTOPRIMARY) };
    
    let monitor_capture_item = create_capture_item(Handle::HMONITOR(main_monitor_handle)).unwrap();

    let (width, height) = match monitor_capture_item.Size() {
        Ok(size) => (size.Width, size.Height),
        Err(error) => {
            println!("Failed to get capture item size: {:?}", error);
            return;
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
        }
    };

    match take_sc(&monitor_capture_item, &capture_rect) {
        Ok(_) => println!("Screenshot taken"),
        Err(error) => println!("Failed to take screenshot: {:?}", error),
    };
}

fn take_sc(item: &GraphicsCaptureItem, rect: &RECT) -> Result<()> {
    // The size of the target of the capture.
    let item_size = item.Size()?;
    println!("item_size: {:?}", item_size);

    // Create a D3D11 device
    let d3d_device = create_d3d_device()?;
    let d3d_context = unsafe { d3d_device.GetImmediateContext()? };

    let device = create_direct3d_device(&d3d_device)?;
    let frame_pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
        &device,
        DirectXPixelFormat::B8G8R8A8UIntNormalized,
        1,
        item_size,
    )?;

    let session = frame_pool.CreateCaptureSession(item)?;
    let (sender, receiver) = channel();

    frame_pool.FrameArrived(
        &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new({
            move |frame_pool, _| {
                let frame_pool = frame_pool.as_ref().unwrap();
                let frame = frame_pool.TryGetNextFrame()?;
                sender.send(frame).unwrap();
                Ok(())
            }
        }),
    )?;

    session.StartCapture()?;

    let frame = receiver.recv().unwrap();

    let texture = unsafe {
        let source_texture: ID3D11Texture2D = get_d3d_interface_from_object(&frame.Surface()?)?;

        let mut desc = D3D11_TEXTURE2D_DESC::default();
        source_texture.GetDesc(&mut desc);
        desc.BindFlags = D3D11_BIND_FLAG(0);
        desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG(0);
        desc.Usage = D3D11_USAGE_STAGING;
        desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;

        let copy_texture = {
            let mut texture = None;
            d3d_device.CreateTexture2D(&desc, None, Some(&mut texture))?;
            texture.unwrap()
        };

        // d3d_context.CopyResource(Some(&copy_texture.cast()?), Some(&source_texture.cast()?));

        d3d_context.CopySubresourceRegion(
            Some(&copy_texture.cast()?),
            0,
            0,
            0,
            0,
            Some(&source_texture.cast()?),
            0,
            Some(&D3D11_BOX {
                left: rect.left as u32,
                top: rect.top as u32,
                right: rect.right as u32,
                bottom: rect.bottom as u32,
                front: 0,
                back: 1,
            }),
        );

        session.Close()?;
        frame_pool.Close()?;

        copy_texture
    };

    let bits = unsafe {
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        texture.GetDesc(&mut desc as *mut _);

        let resource: ID3D11Resource = texture.cast()?;
        let mut mapped = D3D11_MAPPED_SUBRESOURCE::default();
        d3d_context.Map(
            Some(&resource.clone()),
            0,
            D3D11_MAP_READ,
            0,
            Some(&mut mapped),
        )?;

        // Get a slice of bytes
        let slice: &[u8] = {
            std::slice::from_raw_parts(
                mapped.pData as *const _,
                (desc.Height * mapped.RowPitch) as usize,
            )
        };

        let bytes_per_pixel = 4;
        let mut bits = vec![0u8; (desc.Width * desc.Height * bytes_per_pixel) as usize];
        for row in 0..desc.Height {
            let data_begin = (row * (desc.Width * bytes_per_pixel)) as usize;
            let data_end = ((row + 1) * (desc.Width * bytes_per_pixel)) as usize;

            let slice_begin = (row * mapped.RowPitch) as usize;
            let slice_end = slice_begin + (desc.Width * bytes_per_pixel) as usize;

            bits[data_begin..data_end].copy_from_slice(&slice[slice_begin..slice_end]);
        }

        d3d_context.Unmap(Some(&resource), 0);

        bits
    };

    save_as_image(bits, item_size)?;

    Ok(())
}
