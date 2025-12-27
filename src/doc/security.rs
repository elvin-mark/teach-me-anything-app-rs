use utoipa::Modify;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme};

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearerAuth", // The arbitrary name for your security scheme
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        )
    }
}
