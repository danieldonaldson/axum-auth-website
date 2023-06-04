use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailTokenExpired,
    AuthFailTokenUnexpected,
    AuthFailUserNotFound,
    DBFailFieldNotFound(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("Error: {:?}", self);
        (StatusCode::INTERNAL_SERVER_ERROR, "Error").into_response()
    }
}
