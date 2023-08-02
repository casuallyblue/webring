use axum::{response::Redirect, routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use maud::{html, Markup};
use reqwest::StatusCode;

mod homepage;

use homepage::*;

async fn get_keys(client: &reqwest::Client) -> Result<Markup, Box<dyn std::error::Error>> {
    let response = client.get("https://github.com/casually-blue.keys").send().await?;
    match response.status() {
        StatusCode::OK => {
            Ok(html! { div ."key-container" ."keys-loaded"  {
                h3 {"SSH Pubkeys"}
                @for key in response.text().await?.split('\n') {
                    p { (key) }
                }
            }})
        }
        _ => Err("Could not get keys".to_string().into())
    }
}

#[tokio::main]
async fn main() {
    let js_dir = ServeDir::new("js");
    let css_dir = ServeDir::new("css");


    let app = Router::new()
        .nest_service("/js", js_dir)
        .nest_service("/css", css_dir)
        .route("/", get(|| async {HomePage{}.page()}))
        .route(
            "/keys",
            get(|| async { 
                let client = reqwest::Client::new();
                match get_keys(&client).await {
                    Ok(html) => html,
                    _ => {
                        html! { 
                            p { "no keys found"}
                        } 
                    }
                }
            })
        )
    ;

    let port = std::env::args().skip(1).next().unwrap();

    let ip_port = format!("0.0.0.0:{}", port);
    axum::Server::bind(&ip_port.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
