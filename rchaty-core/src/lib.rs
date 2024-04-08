use async_trait::async_trait;

pub async fn run() -> &'static str {
    "Hello, World!"
}

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

#[derive(Copy, Clone)]
pub struct AuthImpl {}

#[derive(Debug)]
pub struct SignupParams {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

pub struct SignupResult {}

#[async_trait]
pub trait Auth {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError>;
}

impl AuthImpl {
    pub fn new() -> Self {
        AuthImpl {}
    }
}

#[async_trait]
impl Auth for AuthImpl {
    async fn signup(&self, params: SignupParams) -> Result<SignupResult, BaseError> {
        tracing::info!("signup params: {:?}", params);
        Ok(SignupResult {})
    }
}
