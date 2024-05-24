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

#[derive(Template)]
#[template(path = "htmx/store_auth_token.html")]
pub struct StoreAuthToken<'a> {
    pub need_redirect: bool,
    pub redirect_url: &'a str,
    pub token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

impl<'a> StoreAuthToken<'a> {
    pub fn htmx(
        need_redirect: bool,
        redirect_url: &str,
        token: String,
        refresh_token: String,
        expires_in: i64,
    ) -> String {
        let template = StoreAuthToken {
            need_redirect,
            redirect_url,
            token,
            refresh_token,
            expires_in,
        };
        template.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "htmx/chat_incoming.html")]
pub struct ChatIncomming<'a> {
    pub content: &'a str,
    pub date: &'a str,
}

impl<'a> ChatIncomming<'a> {
    pub fn htmx(content: &'a str, date: &'a str) -> String {
        let template = ChatIncomming { content, date };
        template.render().unwrap()
    }
}

#[derive(Template)]
#[template(path = "htmx/contact_list.html")]
pub struct ContactListHtmx<'a> {
    pub contacts: &'a Vec<ContactItemHtmx<'a>>,
}

#[derive(Debug, Clone)]
pub struct ContactItemHtmx<'a> {
    pub user_id: &'a str,
    pub name: &'a str,
}

impl<'a> ContactItemHtmx<'a> {
    pub fn new(user_id: &'a str, name: &'a str) -> Self {
        ContactItemHtmx { user_id, name }
    }
}

impl<'a> ContactListHtmx<'a> {
    pub fn htmx(contacts: &'a Vec<ContactItemHtmx<'a>>) -> String {
        let template = ContactListHtmx { contacts };
        template.render().unwrap()
    }
}
