use actix_web::error::BlockingError;
use actix_web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use serde_derive::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(diesel::result::Error),
    OperationCanceled,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This recored violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exists"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The running operation was canceled"),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(e),
        }
    }
}

impl From<BlockingError> for AppError {
    fn from(e: BlockingError) -> Self {
        match e {
            _ => AppError::OperationCanceled,
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    err: String,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let err = format!("{}", self);
        let mut builder = match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest(),
            AppError::RecordNotFound => HttpResponse::NotFound(),
            _ => HttpResponse::InternalServerError(),
        };
        builder.json(ErrorResponse { err })
    }

    // fn render_response(&self) -> HttpResponse {
    //     self.error_response()
    // }
}
