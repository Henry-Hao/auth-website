use std::fmt;

use actix_web::{HttpResponse, ResponseError, http::header};
use mongodb::error::Error as MongoError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum ErrorKind {
    AuthenticationError,
    MongoDBError,
    RedisError,
    ParseError,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub label: String,
}

impl fmt::Display for Error
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error:{}", self.label)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let mut resp = HttpResponse::new(self.status_code());
        let mut buf = actix_web::web::BytesMut::new();
        buf.extend_from_slice(format!("{}", self).as_bytes());
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("text/plain; charset=utf-8"),
        );
        resp.set_body(actix_web::dev::Body::from(buf))
    }

}

impl From<MongoError> for Error {
    fn from(err: MongoError) -> Self {
        Self {
            kind: ErrorKind::MongoDBError,
            label: err.labels().join(","),
        }
    }
}
