use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use transformer::models::transform::TransformInput;
use transformer::services::transform::transform_p_case;

pub fn routes() -> Router {
    Router::new().route("/transform", post(post_transform))
}

async fn post_transform(Json(body): Json<TransformInput>) -> Result<String, (StatusCode, String)> {
    let transformed = transform_p_case(body.html, body.transform);
    Ok(transformed)
}
