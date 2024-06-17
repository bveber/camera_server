use rscam::{Camera, Config};
use std::error::Error;
use image::{ImageBuffer, Rgb};

pub async fn capture_image() -> Result<Vec<u8>, Box<dyn Error>> {
    let fps = 30;
    let resolution_width = 640;
    let resolution_height = 480;
    println!("Attempting to open camera...");
    let mut camera = Camera::new("/dev/video0").map_err(|e| {
        println!("Error: Failed to open camera: {}", e);
        e
    })?;

    println!("Camera opened successfully. Starting camera...");
    camera.start(&Config {
        interval: (1, fps), // 30 fps.
        resolution: (resolution_width, resolution_height),
        format: b"YUYV",
        ..Default::default()
    }).map_err(|e| {
        println!("Error: Failed to start camera: {}", e);
        e
    })?;

    println!("Camera started successfully. Capturing frame...");
    let frame = camera.capture().map_err(|e| {
        println!("Error: Failed to capture frame: {}", e);
        e
    })?;

    println!("Frame captured successfully. Converting to JPEG...");

    // Convert YUYV to RGB
    let rgb_image = yuyv_to_rgb(&frame[..], resolution_width, resolution_height)?;

    // Encode RGB image as JPEG
    let mut jpeg_data = Vec::new();
    image::codecs::jpeg::JpegEncoder::new(&mut jpeg_data)
        .encode_image(&rgb_image)
        .map_err(|e| {
            println!("Error: Failed to encode JPEG: {}", e);
            e
        })?;

    println!("JPEG conversion successful.");
    Ok(jpeg_data)
}

fn yuyv_to_rgb(yuyv: &[u8], width: u32, height: u32) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
    let mut rgb_image = ImageBuffer::new(width, height);
    for (i, chunk) in yuyv.chunks(4).enumerate() {
        let y0 = chunk[0] as f32;
        let u = chunk[1] as f32 - 128.0;
        let y1 = chunk[2] as f32;
        let v = chunk[3] as f32 - 128.0;

        let r0 = y0 + 1.402 * v;
        let g0 = y0 - 0.344136 * u - 0.714136 * v;
        let b0 = y0 + 1.772 * u;

        let r1 = y1 + 1.402 * v;
        let g1 = y1 - 0.344136 * u - 0.714136 * v;
        let b1 = y1 + 1.772 * u;

        let x = (i as u32 % width) * 2;
        let y = i as u32 / width;

        rgb_image.put_pixel(x, y, Rgb([r0 as u8, g0 as u8, b0 as u8]));
        if x + 1 < width {
            rgb_image.put_pixel(x + 1, y, Rgb([r1 as u8, g1 as u8, b1 as u8]));
        }
    }
    Ok(rgb_image)
}