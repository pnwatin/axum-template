mod system;

use axum::{Json, Router, routing::get};

use crate::{context::ServerContext, openapi::BaseOpenApi};

pub fn build_axum_router(cx: ServerContext) -> Router {
    let (router, openapi) = BaseOpenApi::router().split_for_parts();

    router.merge(system::router()).route(
        "/api/openapi.json",
        get(move || {
            let spec = serde_json::to_value(&openapi).expect("OpenAPI document serialises");

            async move { Json(spec) }
        })
        .with_state(cx),
    )
}
