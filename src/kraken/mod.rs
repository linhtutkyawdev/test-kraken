use askama::Template;
use askama_axum::{IntoResponse, Response};
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    title: &'a str,
}

pub async fn index() -> impl IntoResponse {
    IndexTemplate {
        title: "Kraken - Index",
    }
}

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../../styles/tailwind.css").to_owned())
        .unwrap()
}
