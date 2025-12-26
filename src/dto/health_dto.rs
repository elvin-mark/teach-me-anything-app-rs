use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponseDto {
    pub status: &'static str,
}
