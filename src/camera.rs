use image::{ImageBuffer, Rgb};
use rscam::{Camera, Config};
use std::error::Error;

/// Captures an image from the camera, converts it from YUYV format to JPEG, and returns the JPEG data.
///
/// # Returns
/// * `Ok(Vec<u8>)` - A vector containing the JPEG data of the captured image.
/// * `Err(Box<dyn Error>)` - An error occurred during the process.
///
/// # Errors
/// This function can return an error if:
/// * The camera fails to open.
/// * The camera fails to start.
/// * The camera fails to capture a frame.
/// * The frame fails to convert to RGB format.
/// * The RGB image fails to encode to JPEG.
pub async fn capture_image() -> Result<Vec<u8>, Box<dyn Error>> {
    const FPS: u32 = 30;
    const RESOLUTION_WIDTH: u32 = 640;
    const RESOLUTION_HEIGHT: u32 = 480;

    println!("Attempting to open camera...");
    let mut camera: Camera = Camera::new("/dev/video0").map_err(|e: std::io::Error| {
        println!("Error: Failed to open camera: {}", e);
        e
    })?;

    println!("Camera opened successfully. Starting camera...");
    camera
        .start(&Config {
            interval: (1, FPS), // 30 fps
            resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
            format: b"YUYV",
            ..Default::default()
        })
        .map_err(|e: rscam::Error| {
            println!("Error: Failed to start camera: {}", e);
            e
        })?;

    println!("Camera opened successfully. Starting camera...");
    camera
        .start(&Config {
            interval: (1, FPS), // 30 fps
            resolution: (RESOLUTION_WIDTH, RESOLUTION_HEIGHT),
            format: b"YUYV",
            ..Default::default()
        })
        .map_err(|e: rscam::Error| {
            println!("Error: Failed to start camera: {}", e);
            e
        })?;

    println!("Camera started successfully. Capturing frame...");
    let frame = camera.capture().map_err(|e: std::io::Error| {
        println!("Error: Failed to capture frame: {}", e);
        e
    })?;

    println!("Frame captured successfully. Converting to JPEG...");

    // Convert YUYV to RGB
    let rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> =
        yuyv_to_rgb(&frame[..], RESOLUTION_WIDTH, RESOLUTION_HEIGHT)?;

    // Encode RGB image as JPEG
    let mut jpeg_data: Vec<u8> = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut jpeg_data)
        .encode_image(&rgb_image)
        .map_err(|e: image::ImageError| {
            println!("Error: Failed to encode JPEG: {}", e);
            e
        })?;

    println!("JPEG conversion successful.");
    Ok(jpeg_data)
}

/// Converts an image from YUYV format to RGB format.
///
/// # Parameters
/// * `yuyv` - A byte slice containing the YUYV image data.
/// * `width` - The width of the image.
/// * `height` - The height of the image.
///
/// # Returns
/// * `Ok(ImageBuffer<Rgb<u8>, Vec<u8>>)` - The converted RGB image.
/// * `Err(Box<dyn Error>)` - An error occurred during the conversion.
///
/// # Errors
/// This function can return an error if the conversion process fails.
fn yuyv_to_rgb(
    yuyv: &[u8],
    width: u32,
    height: u32,
) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut rgb_image: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    for (i, chunk) in yuyv.chunks(4).enumerate() {
        let y0: f32 = chunk[0] as f32;
        let u: f32 = chunk[1] as f32 - 128.0;
        let y1: f32 = chunk[2] as f32;
        let v: f32 = chunk[3] as f32 - 128.0;

        let r0: f32 = y0 + 1.402 * v;
        let g0: f32 = y0 - 0.344136 * u - 0.714136 * v;
        let b0: f32 = y0 + 1.772 * u;

        let r1: f32 = y1 + 1.402 * v;
        let g1: f32 = y1 - 0.344136 * u - 0.714136 * v;
        let b1: f32 = y1 + 1.772 * u;

        let x: u32 = (i as u32 % width) * 2;
        let y: u32 = i as u32 / width;

        rgb_image.put_pixel(x, y, Rgb([r0 as u8, g0 as u8, b0 as u8]));
        if x + 1 < width {
            rgb_image.put_pixel(x + 1, y, Rgb([r1 as u8, g1 as u8, b1 as u8]));
        }
    }
    Ok(rgb_image)
}
