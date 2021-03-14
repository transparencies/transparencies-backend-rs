use pretty_assertions::assert_eq;
use transparencies_backend_rs::server::filters;
use warp::{
    http::StatusCode,
    test::request,
};

#[tokio::test]
async fn health_check_works() {
    let api = filters::health_check();

    let resp = request()
        .method("GET")
        .path("/health_check")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}
