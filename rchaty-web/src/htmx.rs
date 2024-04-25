use askama::Template;

#[derive(Template)]
#[template(path = "login_clicked.html")]
pub struct LoginClicked {}

#[derive(Template)]
#[template(path = "htmx/alert.html")]
pub struct Alert {
    pub message: String,
}

#[derive(Template)]
#[template(path = "htmx/redirect.html")]
pub struct RedirectHtmx<'a> {
    pub url: &'a str,
}

impl<'a> RedirectHtmx<'a> {
    pub fn new(url: &'a str) -> Self {
        Self { url }
    }
}
