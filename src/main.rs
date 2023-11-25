mod kraken;
use axum::{routing::get, Router};
use kraken::{index, tailwindcss};
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index::main))
        .route("/styles/tailwind.css", get(tailwindcss::styles));

    Ok(router.into())
}
