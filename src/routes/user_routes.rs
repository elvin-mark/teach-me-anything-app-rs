use crate::{
    dto::user_dto::{CreateUserDto, UserResponseDto},
    errors::app_error::AppError,
    services::user_service::UserService,
};
use rocket::{Route, State, get, post, routes, serde::json::Json};

#[post("/", data = "<create_user>")]
async fn create_user(
    create_user: Json<CreateUserDto>,
    user_service: &State<UserService>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = user_service.create_user(create_user.into_inner()).await?;
    Ok(Json(user))
}

#[get("/<id>")]
async fn get_user(
    id: i64,
    user_service: &State<UserService>,
) -> Result<Json<UserResponseDto>, AppError> {
    let user = user_service.get_user(id).await?;
    Ok(Json(user))
}

#[get("/")]
async fn list_users(
    user_service: &State<UserService>,
) -> Result<Json<Vec<UserResponseDto>>, AppError> {
    let users = user_service.list_users().await?;
    Ok(Json(users))
}

pub fn routes() -> Vec<Route> {
    routes![create_user, get_user, list_users]
}
