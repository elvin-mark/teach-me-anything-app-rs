use std::sync::Arc;

use crate::{
    config::database::DbPool, dto::user_dto::CreateUserDto, errors::app_error::AppError,
    models::user::User,
};

pub struct UserRepository {
    // DB pool or connection
    pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Arc<Self> {
        Arc::new(Self { pool })
    }

    pub async fn create(&self, dto: CreateUserDto) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password, email, created_at) VALUES (?, ?, ?, datetime('now')) RETURNING *"
        )
        .bind(&dto.username)
        .bind(&dto.password)
        .bind(&dto.email)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    pub async fn create_with_password(
        &self,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO users (username, email, password, created_at) 
             VALUES (?, ?, ?, datetime('now')) RETURNING *",
        )
        .bind(username)
        .bind(email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(user)
    }

    pub async fn find_by_username(&self, username: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
            .bind(username)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => AppError::NotFound("User not found".to_string()),
                _ => AppError::InternalError(format!("Database error: {}", e)),
            })?;

        Ok(user)
    }

    pub async fn find_by_id(&self, id: i64) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    AppError::NotFound(format!("User with id {} not found", id))
                }
                _ => AppError::InternalError(format!("Database error: {}", e)),
            })?;

        Ok(user)
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        Ok(users)
    }

    pub async fn update(&self, id: i64, dto: CreateUserDto) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE users SET username = ?, email = ? WHERE id = ? RETURNING *",
        )
        .bind(&dto.username)
        .bind(&dto.email)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("User with id {} not found", id))
            }
            _ => AppError::InternalError(format!("Database error: {}", e)),
        })?;

        Ok(user)
    }

    pub async fn delete(&self, id: i64) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::InternalError(format!("Database error: {}", e)))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("User with id {} not found", id)));
        }

        Ok(())
    }
}
