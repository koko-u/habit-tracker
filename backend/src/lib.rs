use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::get;
use actix_web::web;
use serde::Serialize;

pub mod config;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse { status: "ok" })
}

pub fn configure(config: &mut web::ServiceConfig) {
    config.service(health);
}

#[cfg(test)]
mod tests {
    use actix_web::App;
    use actix_web::http::StatusCode;
    use actix_web::test;
    use serde_json::json;

    use super::configure;

    #[actix_web::test]
    async fn health_check_returns_ok() {
        let app = test::init_service(App::new().configure(configure)).await;
        let request = test::TestRequest::get().uri("/health").to_request();
        let response = test::call_service(&app, request).await;

        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(body, json!({ "status": "ok" }));
    }
}
