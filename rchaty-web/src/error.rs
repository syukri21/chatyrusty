use askama::Template;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub error: &'a str,
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}
