use std::sync::Arc;

use crate::auth::{JwtSecret, encode_token};
use crate::dto::auth_dto::{AuthResponse, LoginRequest, RegisterRequest, UserInfo};
use crate::errors::app_error::AppError;
use crate::repositories::user_repository::UserRepository;
use bcrypt::{DEFAULT_COST, hash, verify};

#[derive(Clone)]
pub struct AuthService {
    user_repository: Arc<UserRepository>,
    jwt_secret: JwtSecret,
}

impl AuthService {
    pub fn new(user_repository: Arc<UserRepository>, jwt_secret: JwtSecret) -> Self {
        Self {
            user_repository,
            jwt_secret,
        }
    }

    pub async fn register(&self, dto: RegisterRequest) -> Result<AuthResponse, AppError> {
        // Hash password
        let password_hash = hash(&dto.password, DEFAULT_COST)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?;

        // Create user
        let user = self
            .user_repository
            .create_with_password(&dto.username, &dto.email, &password_hash)
            .await?;

        // Generate token
        let token = encode_token(user.id, &user.username, &self.jwt_secret)
            .map_err(|e| AppError::InternalError(format!("Failed to create token: {}", e)))?;

        Ok(AuthResponse {
            token,
            user: UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }

    pub async fn login(&self, dto: LoginRequest) -> Result<AuthResponse, AppError> {
        // Find user by username
        let user = self
            .user_repository
            .find_by_username(&dto.username)
            .await
            .map_err(|_| AppError::BadRequest("Invalid credentials".to_string()))?;

        // Verify password
        let is_valid = verify(&dto.password, &user.password)
            .map_err(|e| AppError::InternalError(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Err(AppError::BadRequest("Invalid credentials".to_string()));
        }

        // Generate token
        let token = encode_token(user.id, &user.username, &self.jwt_secret)
            .map_err(|e| AppError::InternalError(format!("Failed to create token: {}", e)))?;

        Ok(AuthResponse {
            token,
            user: UserInfo {
                id: user.id,
                username: user.username,
                email: user.email,
            },
        })
    }
}
