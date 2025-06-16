use actix_web::{App, HttpResponse, HttpServer, Responder, post, web};
use transformer::models::transform::TransformInput;
use transformer::services::transform::transform_p_case;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // A minimal Actix server!
    HttpServer::new(|| App::new().service(transform))
        .bind(("127.0.0.1", 8081))?
        .run()
        .await
}

#[post("/v1/transform")]
async fn transform(body: web::Json<TransformInput>) -> impl Responder {
    let payload = body.into_inner();
    let transformed = transform_p_case(payload.html, payload.transform);
    HttpResponse::Ok().body(transformed)
}
