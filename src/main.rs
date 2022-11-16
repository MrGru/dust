use std::net::SocketAddr;

use tracing::info;

mod app;
mod errors;
mod routes;
mod settings;
mod logger;

use settings::get_settings;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let settings = get_settings();
    let port = settings.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));

    info!("Server listening on {}", &address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}
