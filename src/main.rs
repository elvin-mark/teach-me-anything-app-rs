mod auth;
mod config;
mod core;
mod doc;
mod dto;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod services;

use repositories::user_repository::UserRepository;
use rocket::catchers;
use routes::{health_routes, user_routes};
use services::user_service::UserService;
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use crate::{
    auth::JwtSecret,
    config::database::{init_db_pool, run_migrations},
    core::{
        agents::{LessonAgent, planner_agent::PlannerAgent},
        llm::{self, new_llm},
    },
    routes::{auth_routes, lesson_routes, planner_routes},
    services::{
        auth_service::AuthService, lesson_service::LessonService, planner_service::PlannerService,
    },
};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    env_logger::init();

    // Initialize database pool (creates file if it doesn't exist)
    let db_pool = init_db_pool().await;

    // Run migrations
    run_migrations(&db_pool).await;

    // Initialize JWT secret
    let jwt_secret_str = std::env::var("JWT_SECRET")
        .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
    let jwt_secret = JwtSecret::new(&jwt_secret_str);

    // Initialize LLM
    let llm_config = llm::types::LlmConfig {
        llm_type: std::env::var("LLM_TYPE").unwrap_or_else(|_| "llamacpp".to_string()),
        base_url: std::env::var("LLM_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_string()),
        model: std::env::var("LLM_MODEL").unwrap_or_else(|_| "default".to_string()),
    };
    let llm = new_llm(llm_config);

    // Initialize agents
    let lesson_agent = LessonAgent::new(llm.clone());
    let planner_agent = PlannerAgent::new(llm.clone());

    // Initialize repositories
    let user_repo = UserRepository::new(db_pool.clone());

    // Initialize services
    let user_service = UserService::new(user_repo.clone());
    let lesson_service = LessonService::new(lesson_agent.clone());
    let planner_service = PlannerService::new(planner_agent.clone());
    let auth_service = AuthService::new(user_repo.clone(), jwt_secret.clone());

    rocket::build()
        .manage(db_pool)
        .manage(jwt_secret)
        .manage(user_service)
        .manage(lesson_service)
        .manage(planner_service)
        .manage(auth_service)
        .mount("/api/auth", auth_routes::routes())
        .mount("/api/health", health_routes::routes())
        .mount("/api/users", user_routes::routes())
        .mount("/api/lessons", lesson_routes::routes())
        .mount("/api/planner", planner_routes::routes())
        .mount(
            "/",
            Scalar::with_url("/docs", doc::api_doc::ApiDoc::openapi()),
        )
        .register(
            "/",
            catchers![
                errors::error_handler::not_found,
                errors::error_handler::unauthorized,
                errors::error_handler::default_catcher
            ],
        )
        .launch()
        .await?;

    Ok(())
}
