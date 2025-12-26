use rocket::serde::json::Json;
use rocket::{Request, catch};
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub code: u16,
}

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: format!("The path '{}' was not found.", req.uri()),
        code: 404,
    })
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: "Authentication required".to_string(),
        code: 401,
    })
}

// A "default" catcher handles any error code that doesn't have a specific catcher
#[catch(default)]
pub fn default_catcher(status: rocket::http::Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        message: "An unexpected error occurred".to_string(),
        code: status.code,
    })
}
