use askama::Template;
use axum::{extract::Query, response::Html};
use rchaty_web::ErrorTemplate;
use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct ErrorPageQueryParams {
    pub msg :String
}


pub async fn error_page(Query(params): Query<ErrorPageQueryParams>) -> axum::response::Html<String> {
    let template = ErrorTemplate { error: &params.msg};
    let html = template.render().unwrap();
    Html(html)
}
