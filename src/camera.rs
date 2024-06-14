use rppal::video::{Video, VideoOptions};
use image::{ImageBuffer, Rgb};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Represents a camera that captures frames and stores the latest frame.
pub struct Camera {
    /// A shared reference to the latest captured frame, protected by a mutex for thread safety.
    pub frame: Arc<Mutex<Option<ImageBuffer<Rgb<u8>, Vec<u8>>>>>,
}

impl Camera {
    /// Creates a new `Camera` instance and starts a background thread to capture frames.
    ///
    /// # Returns
    ///
    /// * `Camera` - A new `Camera` instance with a background thread capturing frames.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Camera;
    ///
    /// let camera = Camera::new();
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the video capture device cannot be initialized or if frame capture fails.
    pub fn new() -> Self {
        let frame = Arc::new(Mutex::new(None));
        let frame_clone = Arc::clone(&frame);
        thread::spawn(move || {
            let mut video = Video::new(VideoOptions::default()).unwrap();
            loop {
                let buffer = video.capture_frame().unwrap();
                let image = ImageBuffer::from_raw(video.get_resolution().0, video.get_resolution().1, buffer).unwrap();
                let mut frame = frame_clone.lock().unwrap();
                *frame = Some(image);
                thread::sleep(Duration::from_millis(100));
            }
        });
        Camera { frame }
    }

    /// Retrieves the latest captured frame.
    ///
    /// # Returns
    ///
    /// * `Option<ImageBuffer<Rgb<u8>, Vec<u8>>>` - The latest captured frame if available, or `None` if no frame has been captured yet.
    ///
    /// # Examples
    ///
    /// ```
    /// use my_crate::Camera;
    ///
    /// let camera = Camera::new();
    /// thread::sleep(std::time::Duration::from_secs(1)); // Wait for at least one frame to be captured
    /// if let Some(frame) = camera.get_frame() {
    ///     println!("Captured a frame with dimensions: {}x{}", frame.width(), frame.height());
    /// } else {
    ///     println!("No frame captured yet.");
    /// }
    /// ```
    ///
    /// # Panics
    ///
    /// This function will panic if the mutex is poisoned.
    pub fn get_frame(&self) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
        let frame = self.frame.lock().unwrap();
        frame.clone()
    }
}