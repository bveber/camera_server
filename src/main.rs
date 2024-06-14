mod camera;
mod server;

use std::sync::Arc;

#[tokio::main]
async fn main() {
    let camera = Arc::new(camera::Camera::new());
    server::start_server(camera).await;
}