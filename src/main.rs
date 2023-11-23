mod kraken;
use axum::{routing::get, Router};
use kraken::{index, styles};
#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(index))
        .route("/styles/tailwind.css", get(styles));

    Ok(router.into())
}
