use askama::Template;

#[derive(Template)]
#[template(path = "login_clicked.html")]
pub struct LoginClicked {}

#[derive(Template)]
#[template(path = "htmx/alert.html")]
pub struct Alert {
    pub message: String,
}

impl Alert {
    pub fn new(message: String) -> Self {
        Self { message }
    }
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

#[derive(Template)]
#[template(path = "htmx/verified_email_checker.html")]
pub struct VerifiedEmailChecker {}

impl VerifiedEmailChecker {
    pub fn htmx() -> String {
        let template = VerifiedEmailChecker {};
        template.render().unwrap()
    }
}
