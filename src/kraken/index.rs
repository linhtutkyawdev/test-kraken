use askama::Template;
use askama_axum::IntoResponse;
#[derive(Template)]
#[template(path = "index.html")]
struct TheTemplate<'a> {
    title: &'a str,
}
pub async fn main() -> impl IntoResponse {
    TheTemplate {
        title: "Kraken - Index",
    }
}
