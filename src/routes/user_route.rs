use super::response_ok;
use crate::models::user::UserForCreate;
use crate::repositories::user_repository::UserRepository;
use crate::routes::error::{Error, Result};
use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes(ur: UserRepository) -> Router {
    Router::new()
        .route(
            "/users",
            get(get_all_users).post(create_user).put(update_user),
        )
        .route("/users/:id", get(get_user_by_id).delete(delete_user))
        .with_state(ur)
}

async fn get_all_users(State(ur): State<UserRepository>) -> Result<Json<Value>> {
    let users = ur.get_all_users().await.map_err(|_| Error::DatabaseError)?;

    response_ok(users)
}

async fn get_user_by_id(
    State(ur): State<UserRepository>,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    let user = ur
        .get_user_by_id(&id)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn create_user(
    State(ur): State<UserRepository>,
    Json(payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    let user = ur
        .insert_user(payload.into())
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn update_user(
    State(ur): State<UserRepository>,
    Json(payload): Json<UserForCreate>,
) -> Result<Json<Value>> {
    let user = ur
        .update_user(payload.into())
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(user)
}

async fn delete_user(
    State(ur): State<UserRepository>,
    Path(id): Path<String>,
) -> Result<Json<Value>> {
    ur.delete_user(&id)
        .await
        .map_err(|_| Error::DatabaseError)?;

    response_ok(json!({}))
}
