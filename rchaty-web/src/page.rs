use askama::Template;

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "signup.html")]
pub struct SignupTemplate {}
