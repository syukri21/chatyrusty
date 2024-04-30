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
    pub fn htmx(message: String) -> String {
        Alert { message }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "htmx/redirect.html")]
pub struct RedirectHtmx<'a> {
    pub url: &'a str,
}

impl<'a> RedirectHtmx<'a> {
    pub fn htmx(url: &'a str) -> String {
        RedirectHtmx { url }.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "htmx/verified_email_checker.html")]
pub struct VerifiedEmailChecker {
    pub user_id: String,
}

impl VerifiedEmailChecker {
    pub fn htmx(user_id: String) -> String {
        let template = VerifiedEmailChecker { user_id };
        template.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "htmx/verified_email_success.html")]
pub struct VerifiedEmailSuccess {}

impl VerifiedEmailSuccess {
    pub fn htmx() -> String {
        let template = VerifiedEmailSuccess {};
        template.render().unwrap()
    }
}
