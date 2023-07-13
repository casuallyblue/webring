use axum::{response::Redirect, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() {
    let static_files = ServeDir::new(".").not_found_service(ServeFile::new("static/404.html"));

    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("/static/index.html") }),
        )
        .nest_service("/static", static_files);

    let port = std::env::args().skip(1).next().unwrap();

    let ip_port = format!("0.0.0.0:{}", port);
    axum::Server::bind(&ip_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
