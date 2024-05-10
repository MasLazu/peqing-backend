use crate::middleware::error::Error;
use crate::models::user::{Role, User};
use crate::repositories::user_repository;
use axum::extract::Request;
use axum::http::header;
use axum::middleware::Next;
use axum::response::Response;
use axum::{body::Body, extract::State};
use core::result;
use hmac::{Hmac, Mac};
use jwt::{Header, Token, VerifyWithKey};
use sha2::Sha384;
use sqlx::PgPool;
use std::collections::BTreeMap;

use crate::middleware::error::Result;

pub async fn auth_resolver(
    db: State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let auth_header = req.headers().get(header::AUTHORIZATION);

    let token_str = auth_header
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .unwrap_or_default();

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"sangat rahasia").unwrap();

    let token: result::Result<Token<Header, BTreeMap<String, String>, _>, _> =
        token_str.verify_with_key(&key);

    let mut user: Option<User> = None;

    if let Ok(token) = token {
        let claims = token.claims();
        let user_id = claims.get("sub").unwrap();

        user = user_repository::get_user_by_id(&db, user_id).await.ok();
    }

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}

pub async fn authenticated_only(
    user: Option<User>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    match user {
        Some(_) => Ok(next.run(req).await),
        None => Err(Error::Unauthorized),
    }
}

pub async fn admin_only(user: Option<User>, req: Request<Body>, next: Next) -> Result<Response> {
    match user {
        Some(user) => match user.role {
            Role::Admin => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}

pub async fn dosen_only(user: Option<User>, req: Request<Body>, next: Next) -> Result<Response> {
    match user {
        Some(user) => match user.role {
            Role::Dosen => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}
pub async fn mahasiswa_only(
    user: Option<User>,
    req: Request<Body>,
    next: Next,
) -> Result<Response> {
    match user {
        Some(user) => match user.role {
            Role::Mahasiswa => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}
