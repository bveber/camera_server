mod camera;

use warp::Filter;
use tokio::sync::Mutex;
use std::sync::Arc;
use warp::reject::Reject;
use futures_util::TryFutureExt;

#[derive(Debug)]
struct CaptureError;

impl Reject for CaptureError {}

#[tokio::main]
async fn main() {
    // Shared state for the captured image
    let image_data = Arc::new(Mutex::new(None));

    // Route for capturing an image
    let capture_route = warp::path("capture")
        .and(warp::get())
        .and(with_image_data(image_data.clone()))
        .and_then(capture_handler);

    // Route for serving the captured image
    let image_route = warp::path("image")
        .and(warp::get())
        .and(with_image_data(image_data.clone()))
        .and_then(image_handler);

    let routes = capture_route.or(image_route);

    println!("Server running on http://localhost:3030");
    warp::serve(routes).run(([0, 0, 0, 0], 3030)).await;
}

fn with_image_data(
    image_data: Arc<Mutex<Option<Vec<u8>>>>
) -> impl Filter<Extract = (Arc<Mutex<Option<Vec<u8>>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || image_data.clone())
}

async fn capture_handler(
    image_data: Arc<Mutex<Option<Vec<u8>>>>
) -> Result<impl warp::Reply, warp::Rejection> {
    // Spawn blocking task and wait for it to complete
    let image_result = tokio::task::spawn_blocking(move || camera::capture_image())
        .await
        .map_err(|_| warp::reject::custom(CaptureError))?;

    // Handle the inner result
    let image = image_result.map_err(|_| warp::reject::custom(CaptureError)).await?;

    // Store the image in the shared state
    let mut data = image_data.lock().await;
    *data = Some(image.clone());

    println!("Image captured successfully, size: {} bytes", image.len());

    // Return HTML response with the image
    Ok(warp::reply::html("<html><body><img src=\"/image\"></body></html>"))
}

async fn image_handler(
    image_data: Arc<Mutex<Option<Vec<u8>>>>
) -> Result<impl warp::Reply, warp::Rejection> {
    // Lock and access the shared state
    let data = image_data.lock().await;

    if let Some(image) = &*data {
        // Return the image as the response
        Ok(warp::http::Response::builder()
            .header("Content-Type", "image/jpeg")
            .body(image.clone()))
    } else {
        // Return a 404 error if no image is available
        Err(warp::reject::not_found())
    }
}
