use askama::Template;

#[derive(Template)]
#[template(path = "login_clicked.html")]
pub struct LoginClicked {}

#[derive(Template)]
#[template(path = "alert.html")]
pub struct Alert {
    pub message: String,
}
