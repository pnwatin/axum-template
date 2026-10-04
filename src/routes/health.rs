use std::time::Duration;

use axum::{Json, http::StatusCode};
use serde::Serialize;
use tokio::time::timeout;
use utoipa::ToSchema;
use utoipa_axum::routes;

use super::ApiRouter;
use crate::context::ServerContext;

const READINESS_TIMEOUT: Duration = Duration::from_secs(3);

pub(super) fn router() -> ApiRouter {
    ApiRouter::new()
        .routes(routes!(live))
        .routes(routes!(ready))
}

#[derive(Serialize, ToSchema)]
struct LivenessResponse {
    status: HealthStatus,
}

#[utoipa::path(
    get,
    path = "/health/live",
    tag = "health",
    responses((status = 200, body = LivenessResponse))
)]
async fn live() -> Json<LivenessResponse> {
    Json(LivenessResponse {
        status: HealthStatus::Ok,
    })
}

#[derive(Serialize, ToSchema)]
struct ReadinessResponse {
    status: HealthStatus,
    checks: ReadinessChecks,
}

#[derive(Serialize, ToSchema)]
struct ReadinessChecks {
    database: HealthStatus,
}

impl ReadinessChecks {
    fn status(&self) -> HealthStatus {
        let required_checks = [self.database];

        if required_checks
            .into_iter()
            .all(|check| matches!(check, HealthStatus::Ok))
        {
            HealthStatus::Ok
        } else {
            HealthStatus::Unavailable
        }
    }
}

#[utoipa::path(
    get,
    path = "/health/ready",
    tag = "health",
    responses(
        (status = 200, body = ReadinessResponse),
        (status = 503, body = ReadinessResponse),
    )
)]
async fn ready(cx: ServerContext) -> (StatusCode, Json<ReadinessResponse>) {
    let database = check_database(&cx).await;

    let checks = ReadinessChecks { database };

    let status = checks.status();

    (
        StatusCode::from(status),
        Json(ReadinessResponse { status, checks }),
    )
}

async fn check_database(cx: &ServerContext) -> HealthStatus {
    match timeout(
        READINESS_TIMEOUT,
        sqlx::query_scalar::<_, i32>("SELECT 1").fetch_one(&cx.database),
    )
    .await
    {
        Ok(Ok(_)) => HealthStatus::Ok,
        _ => HealthStatus::Unavailable,
    }
}

#[derive(Clone, Copy, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
enum HealthStatus {
    Ok,
    Unavailable,
}

impl From<HealthStatus> for StatusCode {
    fn from(status: HealthStatus) -> Self {
        match status {
            HealthStatus::Ok => Self::OK,
            HealthStatus::Unavailable => Self::SERVICE_UNAVAILABLE,
        }
    }
}
