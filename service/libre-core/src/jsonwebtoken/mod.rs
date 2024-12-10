use chrono::TimeDelta;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use actix_web::{dev::ServiceRequest, HttpMessage as _};
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub mod init;

pub use init::{init_decoder, init_encoder};

pub struct TokenEncoder {
    pub private_key: EncodingKey,
    pub algorithm: Algorithm,
}

impl TokenEncoder {
    pub fn generate_jwt(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(self.algorithm), claims, &self.private_key)
    }
}

pub struct TokenDecoder {
    pub public_key: DecodingKey,
    pub algorithm: Algorithm,
}

impl TokenDecoder {
    pub fn validate(&self, token: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.public_key, &Validation::new(self.algorithm))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// Issuer
    pub iss: String,
    /// Expiration time
    pub exp: u64,
    /// Subject, also user ID
    pub sub: String,
    /// Audience
    pub aud: String,
    /// Issued at
    pub iat: u64,
    /// JWT ID
    pub jti: String,
    /// Login
    pub login: String,
    /// User name
    pub name: String,
    /// Permissions
    pub admin: bool,
    // ...other fields...
}

impl Claims {
    pub fn expiration(mut self, duration: TimeDelta) -> Self {
        self.exp = self.iat + duration.num_seconds() as u64;
        self
    }

    pub fn generate_jwt(
        &self,
        jwt_util: &TokenEncoder,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        jwt_util.generate_jwt(self)
    }
}

pub fn generate_key_pair(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

/// 认证中间件
/// wrap in scope needs validation
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::error::Error> {
    let jwt = req
        .app_data::<TokenDecoder>()
        .expect("JwtUtil is not configured");
    match jwt.validate(credentials.token()) {
        Ok(user) => {
            req.extensions_mut().insert(user.claims);
            Ok(req)
        }
        Err(e) => Err(actix_web::error::ErrorUnauthorized(e.to_string())),
    }
}

pub async fn validator_no_data(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::error::Error> {
    let jwt = req
        .app_data::<TokenDecoder>()
        .expect("JwtUtil is not configured");
    jwt.validate(credentials.token())
        .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;
    Ok(req)
}
