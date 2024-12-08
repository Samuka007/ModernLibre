use actix_web::web;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::environment::{JWT_ALGORITHM, JWT_PRIVATE_KEY_PATH, JWT_PUBLIC_KEY_PATH};

use super::JwtUtil;

pub fn init(cfg: &mut web::ServiceConfig) {
    let jwt_util = env_jwt_util();
    cfg.app_data(web::Data::new(jwt_util)); // todo...
}

fn env_jwt_util() -> JwtUtil {
    let algorithm = env_jwt_algorithm();
    let pubkey_path =
        std::env::var(JWT_PUBLIC_KEY_PATH).expect(&format!("{JWT_PUBLIC_KEY_PATH} must be set"));
    let privkey_path =
        std::env::var(JWT_PRIVATE_KEY_PATH).expect(&format!("{JWT_PRIVATE_KEY_PATH} must be set"));

    let pub_file = std::fs::read(pubkey_path).expect("Failed to read public key file");
    let priv_file = std::fs::read(privkey_path).expect("Failed to read private key file");

    let public_key = DecodingKey::from_rsa_pem(&pub_file).expect("Failed to parse public key file");
    let private_key = EncodingKey::from_rsa_pem(&priv_file).expect("Failed to parse private key file");

    JwtUtil {
        public_key,
        private_key,
        algorithm,
    }
}

fn env_jwt_algorithm() -> jsonwebtoken::Algorithm {
    let alg = std::env::var(JWT_ALGORITHM)
        .unwrap_or_else(|_| "RS256".to_string())
        .parse()
        .expect("Unknown algorithm, must be one of RS256, RS384, RS512");

    match alg {
        jsonwebtoken::Algorithm::RS256
        | jsonwebtoken::Algorithm::RS384
        | jsonwebtoken::Algorithm::RS512 => alg,
        _ => panic!("JWT_ALGORITHM must be an RSA algorithm (RS256, RS384, RS512)"),
    }
}
