use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, HeaderMap},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::sync::Arc;

use crate::{error::AppError, state::AppState};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: i64,
    pub account: String,
    pub nickname: String,
    pub permissions: Vec<i32>,
    pub exp: i64,
}

impl Token {
    pub fn has_permission(&self, perm: i32) -> bool {
        self.permissions.contains(&perm)
    }

    pub fn is_basic(&self) -> bool {
        self.has_permission(0)
    }

    pub fn is_game_admin(&self, admins: &[i64]) -> bool {
        self.has_permission(5) && admins.contains(&self.id)
    }
}

pub fn sign_hs256(header_b64: &str, payload_b64: &str, secret: &[u8]) -> String {
    let msg = format!("{}.{}", header_b64, payload_b64);
    let mut mac = HmacSha256::new_from_slice(secret).expect("HMAC accepts any key size");
    mac.update(msg.as_bytes());
    URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
}

pub fn encode_jwt(claims: &Token, secret: &str) -> String {
    let header = URL_SAFE_NO_PAD.encode(r#"{"alg":"HS256","typ":"JWT"}"#);
    let payload = URL_SAFE_NO_PAD.encode(serde_json::to_string(claims).unwrap());
    let sig = sign_hs256(&header, &payload, secret.as_bytes());
    format!("{}.{}.{}", header, payload, sig)
}

fn decode_jwt(token_str: &str, secret: &str) -> Option<Token> {
    let parts: Vec<&str> = token_str.splitn(3, '.').collect();
    if parts.len() != 3 {
        return None;
    }
    let expected_sig = sign_hs256(parts[0], parts[1], secret.as_bytes());
    // constant-time comparison
    if !constant_time_eq(expected_sig.as_bytes(), parts[2].as_bytes()) {
        return None;
    }
    let payload = URL_SAFE_NO_PAD.decode(parts[1]).ok()?;
    let claims: Token = serde_json::from_slice(&payload).ok()?;
    if claims.exp < chrono::Utc::now().timestamp() {
        return None;
    }
    Some(claims)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.iter().zip(b.iter()).fold(0u8, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct OptionalToken(pub Option<Token>);

#[derive(Debug, Clone)]
pub struct RequiredToken(pub Token);

fn extract_bearer(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
}

impl<S> FromRequestParts<S> for OptionalToken
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let app = Arc::<AppState>::from_ref(state);
        let token = extract_bearer(&parts.headers)
            .and_then(|t| decode_jwt(t, &app.config.auth.signing_key));
        Ok(OptionalToken(token))
    }
}

impl<S> FromRequestParts<S> for RequiredToken
where
    S: Send + Sync,
    Arc<AppState>: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> std::result::Result<Self, Self::Rejection> {
        let app = Arc::<AppState>::from_ref(state);
        let token = extract_bearer(&parts.headers)
            .and_then(|t| decode_jwt(t, &app.config.auth.signing_key))
            .ok_or(AppError::Unauthorized)?;
        if !token.is_basic() {
            return Err(AppError::Forbidden("account not activated".into()));
        }
        Ok(RequiredToken(token))
    }
}
