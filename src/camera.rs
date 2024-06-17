use rscam::{Camera, Config};

pub async fn capture_image() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut camera = Camera::new("/dev/video0")?;
    
    camera.start(&Config {
        interval: (1, 30), // 30 fps.
        resolution: (640, 480),
        format: b"JPEG",
        ..Default::default()
    })?;

    let frame = camera.capture()?;

    Ok(frame.to_vec())
}
