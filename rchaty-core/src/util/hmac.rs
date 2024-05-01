use crate::{util::signature::Signature, BaseError};
use base64::prelude::*;
use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone, Debug)]
pub struct HmacSignatureImpl {
    key: String,
}

impl HmacSignatureImpl {
    pub fn new(key: String) -> Self {
        Self { key }
    }
}

impl Signature for HmacSignatureImpl {
    fn sign(&self, data: &str) -> Result<String, BaseError> {
        let new_from_slice = HmacSha256::new_from_slice(self.key.as_bytes());
        let mut mac = match new_from_slice {
            Ok(mac) => mac,
            Err(e) => {
                return Err(BaseError {
                    code: 500,
                    messages: e.to_string(),
                })
            }
        };
        mac.update(data.as_bytes());
        let result = mac.finalize().into_bytes();
        let code = BASE64_URL_SAFE.encode(result);
        Ok(code)
    }

    fn verify(&self, data: &str, signature: &str) -> Result<(), BaseError> {
        let key = HmacSha256::new_from_slice(self.key.as_bytes());
        let mut mac = match key {
            Ok(mac) => mac,
            Err(e) => {
                return Err(BaseError {
                    code: 500,
                    messages: e.to_string(),
                })
            }
        };
        mac.update(data.as_bytes());
        let signature = BASE64_URL_SAFE.decode(signature)?;
        mac.verify_slice(&signature)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hmac() {
        let hmac = HmacSignatureImpl::new("admin".to_string());
        let data = "239f1055-0a6b-4d01-9202-c0fff0a50a26";
        let res = hmac.sign(data).unwrap();
        println!("{}", res);
        assert!("q52fkjXJGu6tuIgEMBdfsJkdhHThxxYrgU7J_Zx0cY0=" == res);
    }

    #[test]
    fn test_hmac_verify() {
        let hmac = HmacSignatureImpl::new("admin".to_string());
        let data = "239f1055-0a6b-4d01-9202-c0fff0a50a26";

        let signature = "q52fkjXJGu6tuIgEMBdfsJkdhHThxxYrgU7J_Zx0cY0=";
        // let signature = "q52fkjXJGu6tuIgEMBdfsJkdhHThxxYrgU7J/Zx0cY0=";
        let res = hmac.verify(data, signature);
        assert!(res.is_ok());
    }
}
