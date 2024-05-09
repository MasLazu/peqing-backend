use super::response_ok;
use crate::models::user::UserForCreate;
use crate::repositories::user_repository;
use crate::routes::error::{Error, Result};
use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use bcrypt::{hash, DEFAULT_COST};
use serde_json::{json, Value};
use sqlx::PgPool;

pub fn routes(db: PgPool) -> Router {
    Router::new()
        .route(
            "/users",
            get(get_all_users).post(create_user).put(update_user),
        )
        .route("/users/:id", get(get_user_by_id).delete(delete_user))
        .with_state(db)
}

async fn get_all_users(State(db): State<PgPool>) -> Result<Json<Value>> {
    let users = user_repository::get_all_users(&db)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(users)
}

async fn get_user_by_id(State(db): State<PgPool>, Path(id): Path<String>) -> Result<Json<Value>> {
    let user = user_repository::get_user_by_id(&db, &id)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn create_user(
    State(db): State<PgPool>,
    Json(mut payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    payload.password =
        hash(payload.password, DEFAULT_COST).map_err(|_| Error::InternalServerError)?;

    let user = user_repository::insert_user(&db, payload.into())
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn update_user(
    State(db): State<PgPool>,
    Json(mut payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    payload.password =
        hash(payload.password, DEFAULT_COST).map_err(|_| Error::InternalServerError)?;

    let user = user_repository::update_user(&db, payload.into())
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn delete_user(State(db): State<PgPool>, Path(id): Path<String>) -> Result<Json<Value>> {
    user_repository::delete_user(&db, &id)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(json!({}))
}
