extern crate image;
use error::WindowsCaptureError;
use image::{DynamicImage, ImageBuffer, Pixel, Rgba, io::Reader as ImageReader};
use std::sync::mpsc::channel;
use windows::core::ComInterface;
use windows::core::{IInspectable, Result, HSTRING};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::{
    Capture::{Direct3D11CaptureFramePool, GraphicsCaptureItem},
    DirectX::DirectXPixelFormat,
    Imaging::{BitmapAlphaMode, BitmapEncoder, BitmapPixelFormat},
    SizeInt32,
};
use windows::Storage::{CreationCollisionOption, FileAccessMode, StorageFolder};
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Direct3D11::{
    ID3D11Resource, ID3D11Texture2D, D3D11_BIND_FLAG, D3D11_BOX, D3D11_CPU_ACCESS_READ,
    D3D11_MAPPED_SUBRESOURCE, D3D11_MAP_READ, D3D11_RESOURCE_MISC_FLAG, D3D11_TEXTURE2D_DESC,
    D3D11_USAGE_STAGING,
};
use windows::Win32::Graphics::Gdi::HMONITOR;
use windows::Win32::System::WinRT::{
    Graphics::Capture::IGraphicsCaptureItemInterop, RoInitialize, RO_INIT_MULTITHREADED,
};
use windows::Win32::UI::WindowsAndMessaging::{
    GetSystemMetrics, SM_CXPADDEDBORDER, SM_CYCAPTION, SM_CYFRAME,
};
use std::env;

pub mod devices;
pub mod error;
pub mod monitor;
pub mod window;

enum Handle {
    HWND(HWND),
    HMONITOR(HMONITOR),
}

pub enum ImageMode {
    Save,
    NoSave
}

pub struct WindowRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ResourceSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug)]
pub struct ImageResource {
    pub bits: Vec<u8>,
    pub size: ResourceSize,
}

pub fn create_dynamic_image(
    image_scr: &ImageResource,
) -> error::Result<DynamicImage> {
    let mut img = ImageBuffer::new(image_scr.size.width, image_scr.size.height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let offset = (y * image_scr.size.width + x) as usize * 4; // 4 channels (R, G, B, A)
        if offset + 3 < image_scr.bits.len() {
            let rgba = Rgba([
                image_scr.bits[offset],     // Red
                image_scr.bits[offset + 1], // Green
                image_scr.bits[offset + 2], // Blue
                image_scr.bits[offset + 3], // Alpha
            ]);
            *pixel = rgba;
        }
    }

    // Convert the ImageBuffer to a DynamicImage
    let dynamic_image = DynamicImage::ImageRgba8(img);

    Ok(dynamic_image)
}

// The target of the capture, chosen with the picker control.
fn create_capture_item(handle: Handle) -> Result<GraphicsCaptureItem> {
    let interop = windows::core::factory::<GraphicsCaptureItem, IGraphicsCaptureItemInterop>()?;
    match handle {
        Handle::HWND(window_handle) => 
            unsafe { interop.CreateForWindow(window_handle) },
        Handle::HMONITOR(monitor_handle) => 
            unsafe { interop.CreateForMonitor(monitor_handle) },
    }
}

pub fn save_as_image(img: &ImageResource) -> error::Result<DynamicImage> {
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
        img.size.width,
        img.size.height,
        1.0,
        1.0,
        &img.bits,
    )?;

    encoder.FlushAsync()?.get()?;

    let saved_to_path = format!("{}/{}", path, "screenshot.png");

    match ImageReader::open(saved_to_path) {
        Ok(image) => Ok(image.decode().unwrap()), 
        Err(e) => Err(WindowsCaptureError::ImageSaveFailedErr(e))
    }
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

fn take_sc(
    item: &GraphicsCaptureItem,
    rect: &RECT
) -> error::Result<ImageResource> {
    // The size of the target of the capture.
    let item_size = item.Size()?;

    // Create a D3D11 device
    let d3d_device = devices::create_d3d_device()?;
    let d3d_context = unsafe { d3d_device.GetImmediateContext()? };

    let device = devices::create_direct3d_device(&d3d_device)?;
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
        let source_texture: ID3D11Texture2D =
            devices::get_d3d_interface_from_object(&frame.Surface()?)?;

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

    let subresource_size = ResourceSize {
        width: (rect.right - rect.left) as u32,
        height: (rect.bottom - rect.top) as u32,
    };

    println!("{:?}", subresource_size);

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

        let mut title_bar_height = {
            GetSystemMetrics(SM_CYCAPTION)
                + GetSystemMetrics(SM_CYFRAME)
                + GetSystemMetrics(SM_CXPADDEDBORDER)
        } as u32;

        if (rect.bottom == item_size.Height as i32 && rect.right == item_size.Width as i32) {
            title_bar_height = 0;
        }

        let bytes_per_pixel = 4;
        let mut bits = vec![0u8; (subresource_size.width * desc.Height * bytes_per_pixel) as usize];
        for row in title_bar_height..subresource_size.height {
            let data_begin = (row * (subresource_size.width * bytes_per_pixel)) as usize;
            let data_end = ((row + 1) * (subresource_size.width * bytes_per_pixel)) as usize;

            let slice_begin = (row * mapped.RowPitch) as usize;
            let slice_end = slice_begin + (subresource_size.width * bytes_per_pixel) as usize;

            bits[data_begin..data_end].copy_from_slice(&slice[slice_begin..slice_end]);
        }

        d3d_context.Unmap(Some(&resource), 0);

        bits
    };


    // match mode {
    //     ImageMode::Save => save_as_image(&ImageResource {
    //         bits: bits,
    //         size: subresource_size,
    //     }),
    //     ImageMode::NoSave => create_dynamic_image(&ImageResource {
    //         bits: bits,
    //         size: subresource_size,
    //     })
    // }

    Ok(ImageResource { bits: bits, size: subresource_size })
}
