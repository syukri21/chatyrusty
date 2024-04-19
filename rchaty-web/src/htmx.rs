use askama::Template;

#[derive(Template)]
#[template(path = "login_clicked.html")]
pub struct LoginClicked {}
