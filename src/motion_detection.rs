use opencv::prelude::*;
use opencv::videoio::{VideoCapture, CAP_ANY};
use opencv::imgproc::{cvt_color, COLOR_BGR2GRAY};
use opencv::core::{Mat, Size, CV_8UC1};
use std::error::Error;

pub fn detect_motion() -> Result<bool, Box<dyn Error>> {
    let mut cam = VideoCapture::new(0, CAP_ANY)?; // 0 is the default camera
    let mut frame = Mat::default()?;
    cam.read(&mut frame)?;

    let mut gray_frame = Mat::default()?;
    cvt_color(&frame, &mut gray_frame, COLOR_BGR2GRAY, 0)?;

    // Add your motion detection logic here
    // For simplicity, we'll just check if the frame is not empty
    Ok(!gray_frame.empty()?)
}