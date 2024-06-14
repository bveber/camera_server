use warp::Filter;
use std::sync::Arc;
use image::ImageBuffer;
use rppal::video::{Video, VideoOptions};

/// Starts an HTTP server that streams video frames captured by a camera.
///
/// # Arguments
///
/// * `camera` - An `Arc` reference to a `camera::Camera` instance that provides the latest captured frame.
///
/// # Examples
///
/// ```
/// use my_crate::{start_server, camera::Camera};
/// use std::sync::Arc;
///
/// let camera = Arc::new(Camera::new());
/// start_server(camera);
/// ```
///
/// # Details
///
/// This function sets up a Warp web server that serves video frames on the `/video` endpoint. 
/// It clones the provided `camera` reference for use in each request handler. The captured frame is 
/// encoded as a JPEG image and sent as the HTTP response.
///
/// The server listens on all available network interfaces on port 3030.
///
/// # Panics
///
/// This function will panic if the frame cannot be retrieved or encoded as a JPEG image.
pub fn start_server(camera: Arc<camera::Camera>) {
    let camera_filter = warp::any().map(move || Arc::clone(&camera));
    let video_route = warp::path("video")
        .and(camera_filter)
        .map(|camera: Arc<camera::Camera>| {
            let frame = camera.get_frame().unwrap();
            let mut buf = Vec::new();
            image::codecs::jpeg::JpegEncoder::new(&mut buf)
                .encode(&frame, frame.width(), frame.height(), image::ColorType::Rgb8)
                .unwrap();
            warp::http::Response::builder()
                .header("Content-Type", "image/jpeg")
                .body(buf)
        });

    let routes = video_route;

    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030));
}
