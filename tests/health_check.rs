// tests/health_check.rs

use actix_web::{http::StatusCode, test, web, App, HttpResponse};

#[actix_rt::test]
async fn health_check_works() {
    let mut app = test::init_service(
        App::new().service(web::resource("/health_check").to(|| async { HttpResponse::Ok() })),
    )
    .await;

    // Create request object
    let req = test::TestRequest::with_uri("/health_check").to_request();

    // Call application
    let resp = test::call_service(&mut app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}