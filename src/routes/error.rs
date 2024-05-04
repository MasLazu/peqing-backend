use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use derive_more::From;
use serde::Serialize;
use serde_json::json;
use serde_with::serde_as;

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From, strum_macros::AsRefStr, Clone)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    DatabaseError,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        println!("->> {:<12} - {self:?}", "INTO_RES");
        let mut body = json!({
            "status": "error",
        });

        if let Some(map) = body.as_object_mut() {
            match self {
                _ => {
                    map.insert("message".to_string(), json!("Internal Server Error"));
                    map.insert(
                        "code".to_string(),
                        json!(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
                    );
                }
            }
        }

        Response::builder()
            .status(body["code"].as_u64().unwrap() as u16)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_vec(&body).unwrap()))
            .unwrap()
    }
}
