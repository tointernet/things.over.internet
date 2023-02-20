use crate::controllers::things;
use httpw::viewmodels::HttpErrorViewModel;
use utoipa::{OpenApi, openapi::{self, security::{SecurityScheme, Http, HttpAuthScheme}}, Modify};
use crate::viewmodels;

#[derive(OpenApi)]
#[openapi(
        paths(
            things::post,
        ),
        components(
            schemas(
                viewmodels::DelayedThingsRequest, 
                viewmodels::DelayedThingsResponse, 
                HttpErrorViewModel,
            )
        ),
        tags(
            (name = "things", description = "Things management endpoints.")
        ),
        modifiers(&SecurityAddon)
    )]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut openapi::OpenApi) {
      let components = openapi.components.as_mut().unwrap();
      components.add_security_scheme("auth0", SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)))
    }
}

