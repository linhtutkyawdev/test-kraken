use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};

#[derive(Template)]
#[template(path = "test.html")]
struct TestTemplate;

async fn hello_world() -> impl IntoResponse {
    TestTemplate
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(hello_world));

    Ok(router.into())
}
