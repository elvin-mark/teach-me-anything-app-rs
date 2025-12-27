use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct GenerateLessonRequest {
    pub topic: String,
}
