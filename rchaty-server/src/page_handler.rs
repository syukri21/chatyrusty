use askama::Template;
use axum::{extract::Query, response::Html};
use rchaty_web::{
    error::Page404Template,
    htmx::LoginClicked,
    page::{HomeTemplate, LoginTemplate, SignupTemplate},
    ErrorTemplate,
};
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

pub async fn page_404() -> axum::response::Html<String> {
    let template = Page404Template {};
    let html = template.render().unwrap();
    Html(html)
}

pub async fn signup_page() -> axum::response::Html<String> {
    let template = SignupTemplate {};
    let html = template.render().unwrap();
    Html(html)
}

pub async fn home_page() -> axum::response::Html<String> {
    let template = HomeTemplate {};
    let html = template.render().unwrap();
    Html(html)
}
