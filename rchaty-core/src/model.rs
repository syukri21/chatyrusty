#[derive(Debug)]
pub struct BaseError {
    code: usize,
    messages: String,
}

impl std::fmt::Display for BaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.messages)
    }
}

pub struct SignupResult {}

#[derive(Debug)]
pub struct SignupParams {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}
