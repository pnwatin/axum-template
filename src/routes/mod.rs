mod health;

use axum::{Json, Router, routing::get};
use utoipa_axum::router::OpenApiRouter;

use crate::{context::ServerContext, openapi::BaseOpenApi};

pub type ApiRouter = OpenApiRouter<ServerContext>;

pub fn build_axum_router(cx: ServerContext) -> Router {
    let (router, openapi) = BaseOpenApi::router::<ServerContext>()
        .merge(health::router())
        .split_for_parts();

    router
        .route(
            "/openapi.json",
            get(move || {
                let spec = serde_json::to_value(&openapi).expect("OpenAPI document serialises");

                async move { Json(spec) }
            }),
        )
        .with_state(cx)
}
