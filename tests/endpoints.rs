use transparencies_backend_rs::server::filters;
use warp::{
    http::StatusCode,
    test::request,
};

#[tokio::test]
async fn health_check_is_reachable() {
    let api = filters::health_check();

    let resp = request()
        .method("GET")
        .path("/health_check")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn matchinfo_is_reachable() {
    let api = filters::matchinfo();

    let resp = request()
        .method("GET")
        .path("/matchinfo?id_type=profile_id&id_number=459658")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
}
