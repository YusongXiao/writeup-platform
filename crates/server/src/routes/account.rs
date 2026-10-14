use axum::{
    Router,
    extract::State,
    http::{HeaderMap, HeaderValue},
    response::IntoResponse,
    routing::post,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    error::{AppError, Result},
    middleware::auth::{Token, encode_jwt},
    state::AppState,
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/login", post(code_login))
}

#[derive(Deserialize)]
struct CodeLoginRequest {
    code: u64,
}

#[derive(Deserialize)]
struct R2sUser {
    id: i64,
    account: String,
    nickname: String,
    permissions: serde_json::Value,
}

#[derive(Serialize, Deserialize)]
struct JwtClaims {
    id: i64,
    account: String,
    nickname: String,
    permissions: Vec<i32>,
    exp: i64,
}

async fn code_login(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CodeLoginRequest>,
) -> Result<impl IntoResponse> {
    let url = format!(
        "{}/api/account/query?code={}",
        state.config.r2s_api.base_url, body.code
    );

    let resp = state
        .http
        .get(&url)
        .send()
        .await
        .map_err(anyhow::Error::from)?;

    if resp.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(AppError::Forbidden("验证码无效或已过期".into()));
    }
    if !resp.status().is_success() {
        return Err(AppError::Forbidden("验证码验证失败".into()));
    }

    let user: R2sUser = resp.json().await.map_err(anyhow::Error::from)?;

    let permissions: Vec<i32> = user.permissions
        .as_array()
        .map(|arr| arr.iter().filter_map(|v| v.as_i64().map(|n| n as i32)).collect())
        .unwrap_or_default();

    let exp = chrono::Utc::now().timestamp() + 7 * 24 * 3600;
    let claims = Token {
        id: user.id,
        account: user.account,
        nickname: user.nickname,
        permissions,
        exp,
    };

    let token = encode_jwt(&claims, &state.config.auth.signing_key);

    let mut headers = HeaderMap::new();
    if let Ok(v) = HeaderValue::from_str(&token) {
        headers.insert("Set-Token", v);
    }

    Ok((headers, Json(serde_json::json!(null))))
}
