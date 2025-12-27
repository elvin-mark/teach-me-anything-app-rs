use utoipa::OpenApi;

use crate::core::agents::types::GeneratedRoadmap;
use crate::dto::auth_dto::{AuthResponse, LoginRequest, RegisterRequest};
use crate::dto::planner_dto::GenerateRoadmapRequest;
use crate::dto::user_dto::{CreateUserDto, UserResponseDto};
use crate::errors::types::ErrorResponse;
use crate::{core::agents::GeneratedLesson, dto::lesson_dto::GenerateLessonRequest};

use crate::routes::auth_routes::__path_login;
use crate::routes::auth_routes::__path_register;
use crate::routes::health_routes::__path_get_health;
use crate::routes::lesson_routes::__path_generate_lesson;
use crate::routes::planner_routes::__path_generate_roadmap;
use crate::routes::user_routes::__path_delete_user;
use crate::routes::user_routes::__path_get_user;
use crate::routes::user_routes::__path_update_user;

#[derive(OpenApi)]
#[openapi(
    paths(
        generate_lesson,
        get_health,
        login,
        register,
        delete_user,
        get_user,
        update_user,
    generate_roadmap),
    info(
        title = "Teach Me Anything API",
        description = "APIs used in the Teach Me Anything platform",
        version = "1.0.0"
    ),
    components(schemas(
        GeneratedRoadmap,
        GenerateRoadmapRequest,
        GenerateLessonRequest,
        GeneratedLesson,
        RegisterRequest,
        LoginRequest,
        AuthResponse,
        CreateUserDto,
        UserResponseDto,
        ErrorResponse
    )),
    tags(
        (name = "Planner", description = "Planner Endpoints"),
        (name = "Lessons", description = "AI Tutoring Endpoints"),
        (name = "Health", description = "Health Endpoints"),
        (name = "Auth", description = "Auth Endpoints"),
        (name = "Users", description = "Users Endpoints"),
    )
)]
pub struct ApiDoc;
