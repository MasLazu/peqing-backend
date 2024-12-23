use crate::ctx::Ctx;
use crate::models::user::Role;
use crate::repositories::user_repository;
use crate::routes::error::{Error, Result};
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

pub async fn auth_resolver(
    db: State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response> {
    let mut ctx = Ctx::new();
    ctx.push_trace("auth_resolver").await;

    let auth_header = req.headers().get(header::AUTHORIZATION);

    let token_str = auth_header
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .unwrap_or_default();

    let key: Hmac<Sha384> = Hmac::new_from_slice(b"sangat rahasia").unwrap();

    let token: result::Result<Token<Header, BTreeMap<String, String>, _>, _> =
        token_str.verify_with_key(&key);

    if let Ok(token) = token {
        let claims = token.claims();
        let user_id = claims.get("sub").cloned().unwrap_or_default();

        ctx.user = user_repository::get_user_by_id(&db, user_id.as_str())
            .await
            .ok();
    }

    req.extensions_mut().insert(ctx);

    Ok(next.run(req).await)
}

pub async fn authenticated_only(mut ctx: Ctx, req: Request<Body>, next: Next) -> Result<Response> {
    ctx.push_trace(" -> authenticated_only").await;
    match ctx.user {
        Some(_) => Ok(next.run(req).await),
        None => Err(Error::Unauthenticated),
    }
}

pub async fn admin_only(mut ctx: Ctx, req: Request<Body>, next: Next) -> Result<Response> {
    ctx.push_trace(" -> admin_only").await;
    if let Some(user) = ctx.user.clone() {
        print!("{:?}", user);
    }
    match ctx.user {
        Some(user) => match user.role {
            Role::Admin => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}

pub async fn dosen_only(mut ctx: Ctx, req: Request<Body>, next: Next) -> Result<Response> {
    ctx.push_trace(" -> dosen_only").await;
    match ctx.user {
        Some(user) => match user.role {
            Role::Dosen => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}

pub async fn mahasiswa_only(mut ctx: Ctx, req: Request<Body>, next: Next) -> Result<Response> {
    ctx.push_trace(" -> mahasiswa_only").await;
    match ctx.user {
        Some(user) => match user.role {
            Role::Mahasiswa => Ok(next.run(req).await),
            _ => Err(Error::Unauthorized),
        },
        _ => Err(Error::Unauthorized),
    }
}
