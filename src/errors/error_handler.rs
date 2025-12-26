use rocket::serde::json::Json;
use rocket::{Request, catch};

use crate::errors::types::ErrorResponse;

#[catch(404)]
pub fn not_found(req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: format!("The path '{}' was not found.", req.uri()),
    })
}

#[catch(401)]
pub fn unauthorized() -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "Authentication required".to_string(),
    })
}

// A "default" catcher handles any error code that doesn't have a specific catcher
#[catch(default)]
pub fn default_catcher(_: rocket::http::Status, _req: &Request) -> Json<ErrorResponse> {
    Json(ErrorResponse {
        error: "An unexpected error occurred".to_string(),
    })
}
