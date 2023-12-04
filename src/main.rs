use std::error::Error;

use axum::{routing::get, Router, extract::{Query, State}, response::Redirect};
use clap::Parser;
use tower_http::services::ServeDir;
use maud::*;

use std::sync::Arc;

use serde::Deserialize;

use normalize_url_rs::{normalize_url, OptionsBuilder};

mod homepage;
mod join;
mod members;
mod page;

use crate::{homepage::*, page::Page};

#[derive(Parser)]
struct Options {
    port: u16,
    config_file: String
}

#[derive(Deserialize, Debug)]
struct RedirectParams {
    #[serde(rename = "from")]
    url: String,

    #[serde(alias = "dir")]
    direction: Option<String>
}

#[derive(Deserialize, Debug, Clone)]
pub struct WebringSite {
    pub name: String,
    pub url: String,
    pub author: String,
    pub feed: Option<String>,
}

#[derive(Debug)]
pub struct AppState {
    pub sites: Vec<WebringSite>
}


async fn redirect(State(state): State<Arc<AppState>>, Query(redirect_params): Query<RedirectParams>) -> Redirect {
    let options = OptionsBuilder::default().build().unwrap();
    let origin_site_normalized_url = normalize_url(redirect_params.url.as_str(), &options).unwrap();

    let direction = redirect_params.direction.unwrap_or("next".to_string());

    let next_site = if let Some((index, site)) = state.sites.iter().enumerate().find(|(_, site)| {
        let normalized_url = normalize_url(site.url.as_str(), &options);
        if let Ok(normalized_url) = normalized_url {
            normalized_url == origin_site_normalized_url
        } else {
            false
        }
    }) {
        match direction.as_str() {
           "prev"  => {
                if index == 0 {
                    state.sites[state.sites.len() - 1].clone()
                } else {
                    state.sites[index-1].clone()
                }
            },
            "next" => {
                if index == state.sites.len() - 1 {
                    state.sites[0].clone()
                } else {
                    state.sites[index+1].clone()
                }
            },
            _ => state.sites[0].clone()
        }
    } else {
        state.sites[0].clone()
    };

    Redirect::to(next_site.url.as_str())
    
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let options = Options::parse();

    let sites: Vec<WebringSite>= serde_dhall::from_file(options.config_file.as_str()).parse()?;
    let shared_state = std::sync::Arc::new(AppState{sites});


    let js_dir = ServeDir::new("js");
    let css_dir = ServeDir::new("css");

    let images_dir = ServeDir::new("images");

    let app = Router::new()
        .nest_service("/js", js_dir)
        .nest_service("/css", css_dir)
        .nest_service("/images", images_dir)

        .route("/", get(|State(state): State<Arc<AppState>> | async { HomePage {state}.render() }))
        .route("/join", get(|State(state): State<Arc<AppState>>| async {join::JoinPage{state}.render()}))
        .route("/members", get(|State(state): State<Arc<AppState>>| async {members::MembersPage{state}.render()}))
        .route("/redirect", get(redirect))

        .with_state(shared_state);

    let ip_port = format!("0.0.0.0:{}", options.port);
    axum::Server::bind(&ip_port.parse()?)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
