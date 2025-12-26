use crate::auth::{AuthUser, JwtSecret, decode_token};
use crate::errors::app_error::AppError;
use rocket::State;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};

pub struct AuthGuard {
    pub user: AuthUser,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = AppError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Get JWT secret from Rocket state
        let secret = match request.guard::<&State<JwtSecret>>().await {
            Outcome::Success(s) => s,
            _ => {
                return Outcome::Error((
                    Status::InternalServerError,
                    AppError::InternalError("JWT secret not configured".to_string()),
                ));
            }
        };

        // Extract token from Authorization header
        let token = match request.headers().get_one("Authorization") {
            Some(header) => {
                if let Some(token) = header.strip_prefix("Bearer ") {
                    token
                } else {
                    return Outcome::Error((
                        Status::Unauthorized,
                        AppError::BadRequest("Invalid authorization header format".to_string()),
                    ));
                }
            }
            None => {
                return Outcome::Error((
                    Status::Unauthorized,
                    AppError::BadRequest("Missing authorization header".to_string()),
                ));
            }
        };

        // Decode and validate token
        match decode_token(token, secret) {
            Ok(claims) => {
                let user = AuthUser {
                    id: claims.sub.parse().unwrap_or(0),
                    // username: claims.username,
                };
                Outcome::Success(AuthGuard { user })
            }
            Err(_) => Outcome::Error((
                Status::Unauthorized,
                AppError::BadRequest("Invalid or expired token".to_string()),
            )),
        }
    }
}
