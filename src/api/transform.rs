use crate::models::transform::TransformInput;
use crate::services::transform::transform_p_case;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};

pub fn routes() -> Router {
    Router::new().route("/transform", post(post_transform))
}

async fn post_transform(Json(body): Json<TransformInput>) -> Result<String, (StatusCode, String)> {
    let transformed = transform_p_case(body.html, body.transform);
    Ok(transformed)
}
