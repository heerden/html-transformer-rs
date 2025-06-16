use axum::Router;
use tower_http::compression::CompressionLayer;
use tower_http::compression::predicate::SizeAbove;
use tower_http::cors::CorsLayer;

mod transform;

pub fn routes() -> Router {
    let compress_limit = SizeAbove::new(128);
    let compression = CompressionLayer::new()
        .gzip(true)
        .deflate(true)
        .br(true)
        .compress_when(compress_limit);
    Router::new()
        .nest("/v1", transform::routes())
        .layer(compression)
        .layer(CorsLayer::permissive())
}
