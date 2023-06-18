use axum::Router;
use utoipa::{
    openapi::{
        self,
        security::{ApiKey, ApiKeyValue, SecurityScheme},
    },
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

mod meta;
mod post;

pub fn router() -> Router {
    #[derive(OpenApi)]
    #[openapi(
        paths(
            meta::root::handler,
            meta::healthz::handler,
            post::get_posts::handler,
            post::create_post::handler,
        ),
        components(schemas(
            meta::root::AppMetaPayload,
            post::PostPayload,
            post::create_post::CreatePostInput,
        )),
        modifiers(&SecurityAddon)
    )]
    struct ApiDoc;

    struct SecurityAddon;
    impl Modify for SecurityAddon {
        fn modify(&self, openapi: &mut openapi::OpenApi) {
            if let Some(components) = openapi.components.as_mut() {
                components.add_security_scheme(
                    "api_key",
                    SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
                )
            }
        }
    }

    Router::new()
        // swagger api /docs and static json
        .merge(SwaggerUi::new("/docs").url("/docs/openapi.json", ApiDoc::openapi()))
        // merge api routes
        .merge(meta::router())
        .merge(post::router())
}
