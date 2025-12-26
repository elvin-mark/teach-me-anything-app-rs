use rocket::{
    Request,
    http::Status,
    response::{self, Responder},
    serde::json::to_string,
};

use crate::errors::types::ErrorResponse;

#[derive(Debug, Clone)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let (status, message) = match self {
            AppError::NotFound(msg) => (Status::NotFound, msg),
            AppError::BadRequest(msg) => (Status::BadRequest, msg),
            AppError::InternalError(msg) => (Status::InternalServerError, msg),
        };

        let error_response = ErrorResponse { error: message };
        let json = to_string(&error_response).unwrap();

        response::Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json.len(), std::io::Cursor::new(json))
            .ok()
    }
}
