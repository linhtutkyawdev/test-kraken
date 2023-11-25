use askama_axum::{IntoResponse, Response};
use axum::http::StatusCode;

pub async fn styles() -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../../styles/tailwind.css").to_owned())
        .unwrap()
}
