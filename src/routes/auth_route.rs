use crate::routes::error::Result;
use axum::{routing::post, Json, Router};
use serde_json::Value;
use sqlx::PgPool;

pub fn routes(db: PgPool) -> Router {
    Router::new().route("/auth/login", post(login))
}

async fn login() -> Result<Json<Value>> {
    return Ok(Json(serde_json::json!({
        "status": "success",
        "code": 200,
        "data": "login"
    })));
}
