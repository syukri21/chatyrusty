use askama::Template;

#[derive(Template)]
#[template(path = "pages/error.html")]
pub struct ErrorTemplate<'a> {
    pub error: &'a str,
}

#[derive(Template)]
#[template(path = "pages/404.html")]
pub struct Page404Template {}
