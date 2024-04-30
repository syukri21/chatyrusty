use askama::Template;

#[derive(Template)]
#[template(path = "pages/login.html")]
pub struct LoginTemplate {}

#[derive(Template)]
#[template(path = "pages/signup.html")]
pub struct SignupTemplate {}

#[derive(Template)]
#[template(path = "pages/home.html")]
pub struct HomeTemplate {}
