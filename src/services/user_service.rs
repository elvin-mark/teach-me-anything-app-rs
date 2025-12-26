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

    pub async fn get_user(&self, id: i64) -> Result<UserResponseDto, AppError> {
        let user = self.user_repository.find_by_id(id).await?;
        Ok(UserResponseDto::from(user))
    }

    pub async fn update_user(
        &self,
        id: i64,
        dto: CreateUserDto,
    ) -> Result<UserResponseDto, AppError> {
        let user = self.user_repository.update(id, dto).await?;
        Ok(UserResponseDto::from(user))
    }

    pub async fn delete_user(&self, id: i64) -> Result<String, AppError> {
        self.user_repository.delete(id).await?;
        Ok(String::from("Success"))
    }
}
