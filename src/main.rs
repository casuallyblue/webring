use std::error::Error;

use axum::{routing::get, Router};
use clap::Parser;
use tower_http::services::ServeDir;

mod homepage;
mod page;

use crate::{homepage::*, page::Page};

#[derive(Parser)]
struct Options {
    port: u16,
}

struct SRV {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    let js_dir = ServeDir::new("js");
    let css_dir = ServeDir::new("css");
    let static_dir = ServeDir::new("static");

    let images_dir = ServeDir::new("images");

    let app = Router::new()
        .nest_service("/js", js_dir)
        .nest_service("/css", css_dir)
        .nest_service("/static", static_dir)
        .nest_service("/images", images_dir)
        .route("/", get(|| async { HomePage {}.render() }));

    let ip_port = format!("0.0.0.0:{}", options.port);
    axum::Server::bind(&ip_port.parse()?)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
