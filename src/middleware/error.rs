use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use derive_more::From;
use serde::Serialize;
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, From, Clone)]
pub enum Error {
    Unauthorized,
}

//masih belum selesai (place holder)
impl IntoResponse for Error {
    fn into_response(self) -> Response<Body> {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();
        response.extensions_mut().insert(self);

        response
    }
}
