use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header, request::Parts, StatusCode},
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::env;
use std::sync::OnceLock;
use subtle::ConstantTimeEq;

const JWT_ISSUER: &str = "fatsus_api";
const JWT_AUDIENCE: &str = "fatsus_delphi";
const API_KEY_HEADER: &str = "X-API-Key";

struct AuthConfig {
    jwt_secret: Vec<u8>,
    api_key: String,
    operator_user: String,
    operator_password: String,
}

static AUTH_CONFIG: OnceLock<AuthConfig> = OnceLock::new();

fn required_env(name: &str) -> String {
    let value = env::var(name).unwrap_or_else(|_| panic!("A variável {name} precisa estar configurada"));
    let value = value.trim().to_string();
    if value.is_empty() {
        panic!("A variável {name} não pode estar vazia");
    }
    value
}

pub fn init() {
    AUTH_CONFIG.get_or_init(|| {
        let jwt_secret = required_env("JWT_SECRET");
        let api_key = required_env("FATSUS_API_KEY");
        if jwt_secret.len() < 32 {
            panic!("JWT_SECRET deve ter pelo menos 32 caracteres");
        }
        if api_key.len() < 32 {
            panic!("FATSUS_API_KEY deve ter pelo menos 32 caracteres");
        }

        AuthConfig {
            jwt_secret: jwt_secret.into_bytes(),
            api_key,
            operator_user: required_env("FATSUS_OPERATOR_USER"),
            operator_password: required_env("FATSUS_OPERATOR_PASSWORD"),
        }
    });
}

fn config() -> &'static AuthConfig {
    AUTH_CONFIG
        .get()
        .expect("auth::init() deve ser chamado na inicialização da API")
}

fn secrets_match(provided: &str, expected: &str) -> bool {
    let provided_hash = Sha256::digest(provided.as_bytes());
    let expected_hash = Sha256::digest(expected.as_bytes());
    provided_hash.ct_eq(&expected_hash).into()
}

fn api_key_from_headers(parts: &Parts) -> Result<String, (StatusCode, &'static str)> {
    let header_value = parts
        .headers
        .get(API_KEY_HEADER)
        .ok_or((StatusCode::UNAUTHORIZED, "API Key ausente"))?;

    header_value
        .to_str()
        .map(str::trim)
        .map(ToString::to_string)
        .map_err(|_| (StatusCode::FORBIDDEN, "API Key inválida"))
}

fn jwt_from_headers(parts: &Parts) -> Result<&str, (StatusCode, &'static str)> {
    let from_access_token = parts
        .headers
        .get("X-Access-Token")
        .and_then(|v| v.to_str().ok())
        .map(|value| value.strip_prefix("Bearer ").unwrap_or(value).trim())
        .filter(|value| !value.is_empty());

    if let Some(token) = from_access_token {
        return Ok(token);
    }

    parts
        .headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or((StatusCode::UNAUTHORIZED, "Token JWT ausente ou inválido"))
}

fn validate_api_key(parts: &Parts) -> Result<(), (StatusCode, &'static str)> {
    let provided = api_key_from_headers(parts)?;
    if !secrets_match(&provided, &config().api_key) {
        return Err((StatusCode::FORBIDDEN, "API Key inválida"));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
}

pub struct ApiKeyAuth;

pub struct AuthenticatedUser(#[allow(dead_code)] pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for ApiKeyAuth
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        validate_api_key(parts)?;
        Ok(ApiKeyAuth)
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        validate_api_key(parts)?;

        let token = jwt_from_headers(parts)?;

        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        validation.leeway = 30;
        validation.set_issuer(&[JWT_ISSUER]);
        validation.set_audience(&[JWT_AUDIENCE]);

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&config().jwt_secret),
            &validation,
        )
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Token JWT ausente ou inválido"))?;

        Ok(AuthenticatedUser(token_data.claims))
    }
}

pub fn credenciais_validas(usuario: &str, senha: &str) -> bool {
    let cfg = config();
    let user_ok = secrets_match(usuario, &cfg.operator_user);
    let senha_ok = secrets_match(senha, &cfg.operator_password);
    user_ok && senha_ok
}

pub fn gerar_token(usuario: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::hours(40)).timestamp() as usize;
    let claims = Claims {
        sub: usuario.to_string(),
        exp,
        iat,
        iss: JWT_ISSUER.to_string(),
        aud: JWT_AUDIENCE.to_string(),
    };

    encode(
        &Header::new(Algorithm::HS256),
        &claims,
        &EncodingKey::from_secret(&config().jwt_secret),
    )
}
