use crate::repositories::user_repository::UserRepository;
use crate::routes::error::{Error, Result};
use axum::extract::State;
use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

pub fn routes(ur: UserRepository) -> Router {
    Router::new()
        .route("/users", get(get_all_users))
        .with_state(ur)
}

async fn get_all_users(State(ur): State<UserRepository>) -> Result<Json<Value>> {
    let users = ur
        .get_all_users()
        .await
        .map_err(|_| Error::InternalServerError)?;

    let body = Json(json!({
        "satatus": "success",
        "code": 200,
        "body": {
            "length": users.len(),
            "data": users
        }
    }));

    Ok(body)
}
