use chrono::TimeDelta;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use rand::rngs::OsRng;
use rsa::{RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};

use actix_web::{dev::ServiceRequest, HttpMessage as _};
use actix_web_httpauth::extractors::bearer::BearerAuth;

mod init;

pub use init::init;

pub struct JwtUtil {
    pub public_key: DecodingKey,
    pub private_key: EncodingKey,
    pub algorithm: Algorithm,
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

    // pub fn user(mut self, user: &User) -> Self {
    //     self.sub = user.uid.to_string();
    //     self.jti = user.uid.to_string();
    //     self.name = user.name.clone();
    //     self.admin = user.admin;
    //     self
    // }

    pub fn generate_jwt(&self, jwt_util: &JwtUtil) -> Result<String, jsonwebtoken::errors::Error> {
        jwt_util.generate_jwt(self)
    }
}

pub fn generate_key_pair(bits: usize) -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("Failed to generate a key");
    let public_key = RsaPublicKey::from(&private_key);
    (private_key, public_key)
}

impl JwtUtil {
    pub fn validate_jwt(
        &self,
        token: &str,
    ) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode::<Claims>(token, &self.public_key, &Validation::new(self.algorithm))
    }

    pub fn generate_jwt(&self, claims: &Claims) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(self.algorithm), claims, &self.private_key)
    }
}

/// 认证中间件
/// wrap in scope needs validation
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, actix_web::error::Error> {
    let jwt = req
        .app_data::<JwtUtil>()
        .expect("JwtUtil is not configured");
    match jwt.validate_jwt(credentials.token()) {
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
        .app_data::<JwtUtil>()
        .expect("JwtUtil is not configured");
    jwt.validate_jwt(credentials.token())
    .map_err(|e| actix_web::error::ErrorUnauthorized(e.to_string()))?;
    Ok(req)
}
