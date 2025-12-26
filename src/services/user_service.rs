use std::sync::Arc;

use crate::{
    dto::user_dto::{CreateUserDto, UserResponseDto},
    errors::app_error::AppError,
    repositories::user_repository::UserRepository,
};

pub struct UserService {
    user_repository: Arc<UserRepository>,
}

impl UserService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn create_user(&self, dto: CreateUserDto) -> Result<UserResponseDto, AppError> {
        // Business logic here
        let user = self.user_repository.create(dto).await?;
        Ok(UserResponseDto::from(user))
    }

    pub async fn get_user(&self, id: i64) -> Result<UserResponseDto, AppError> {
        let user = self.user_repository.find_by_id(id).await?;
        Ok(UserResponseDto::from(user))
    }

    pub async fn list_users(&self) -> Result<Vec<UserResponseDto>, AppError> {
        let users = self.user_repository.find_all().await?;
        Ok(users.into_iter().map(UserResponseDto::from).collect())
    }
}
