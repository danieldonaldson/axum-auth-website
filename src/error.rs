use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde_json::json;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    AuthFailNoAuthTokenCookie,
    AuthFailTokenWrongFormat,
    AuthFailTokenExpired,
    AuthFailUserNotFound,
    AuthFailIncorrectPassword,
    AuthFailUserAlreadyExists,
    QueryFailNoUsername,
    QueryFailNoPassword,
    DBFailFieldNotFound(String),
    DBFailFieldEmpty(String),
    DBFailedToCreateUser,
    DBConnectionFail,
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        eprintln!("Error: {:?}", self);
        let (status, err_msg) = match self {
            Self::AuthFailNoAuthTokenCookie => {
                (StatusCode::UNAUTHORIZED, "No token")
            }
            Self::AuthFailTokenWrongFormat => {
                (StatusCode::UNAUTHORIZED, "Wrong token")
            }
            Self::AuthFailTokenExpired => {
                (StatusCode::UNAUTHORIZED, "Expired token")
            }
            Self::AuthFailUserNotFound => {
                (StatusCode::UNAUTHORIZED, "User not found")
            }
            Self::AuthFailIncorrectPassword => {
                (StatusCode::UNAUTHORIZED, "Incorrect password")
            }
            Self::QueryFailNoUsername => {
                (StatusCode::BAD_REQUEST, "Missing username in request")
            }
            Self::QueryFailNoPassword => {
                (StatusCode::BAD_REQUEST, "Missing password in request")
            }
            Self::DBFailFieldNotFound(s) => {
                eprintln!("Database field not found: {:?}", s);
                (StatusCode::INTERNAL_SERVER_ERROR, ("Database error"))
            }
            Self::DBFailFieldEmpty(s) => {
                eprintln!("Database field not expected to be empty: {:?}", s);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database error")
            }
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Error"),
        };
        (status, Json(json!({ "error": err_msg }))).into_response()
    }
}
