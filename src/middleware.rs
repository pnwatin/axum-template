use std::time::Duration;

use axum::{Router, http::StatusCode};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::{RequestBodyTimeoutLayer, TimeoutLayer},
    trace::TraceLayer,
};

use crate::context::ServerContext;

pub fn apply_middlewares(router: Router, _cx: ServerContext) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http())
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(CatchPanicLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
        .layer(CompressionLayer::new().quality(tower_http::CompressionLevel::Fastest));

    router.layer(middleware)
}
