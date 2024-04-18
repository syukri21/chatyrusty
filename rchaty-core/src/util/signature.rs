use crate::BaseError;

pub trait Signature {
    fn sign(&self, data: &str) -> Result<String, BaseError>;
    fn verify(&self, data: &str, signature: &str) -> Result<(), BaseError>;
}
