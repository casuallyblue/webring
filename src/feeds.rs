
pub fn feeds_opml(state: std::sync::Arc<crate::AppState>) -> impl axum::response::IntoResponse {
    let sites = state.sites.iter().filter(|site| site.feed.is_some()).map(|site| {
        format!("<outline text=\"{}\" title=\"{}\" type=\"rss\" xmlUrl=\"{}\" htmlUrl=\"{}\"/>",
            site.name,
            site.name,
            site.feed.clone().unwrap(),
            site.url
        )
    }).collect::<Vec<String>>().join("\n");
    axum::response::Response::builder()
        .status(axum::http::StatusCode::OK)
        .header("content-type", "application/xml")
        .body(axum::body::Body::from(
            format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                <opml version=\"1.0\">
                <head>
                    <title>Webring</title>
                </head>
                <body>
                    {sites}
                </body>
                </opml>
            ")
        )).unwrap()
    
}
