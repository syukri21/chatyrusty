use askama::Template;
use axum::{extract::Query, response::Html};
use rchaty_web::{error::LoginTemplate, htmx::LoginClicked, ErrorTemplate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct ErrorPageQueryParams {
    pub msg: String,
}

pub async fn error_page(
    Query(params): Query<ErrorPageQueryParams>,
) -> axum::response::Html<String> {
    let template = ErrorTemplate { error: &params.msg };
    let html = template.render().unwrap();
    Html(html)
}

pub async fn login_page() -> axum::response::Html<String> {
    let template = LoginTemplate {};
    let html = template.render().unwrap();
    Html(html)
}

pub async fn htmx_login_cliked() -> axum::response::Html<String> {
    let template = LoginClicked {};
    let html = template.render().unwrap();
    Html(html)
}
