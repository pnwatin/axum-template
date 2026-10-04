use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn router() -> OpenApiRouter {
    OpenApiRouter::new().routes(routes!(health))
}

#[derive(Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "system",
    responses((status = 200, body = HealthResponse))
)]
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
    })
}
