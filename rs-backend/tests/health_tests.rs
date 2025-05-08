use axum_test::TestServer;
use rs_backend::app::build_app;


#[tokio::test]
async fn test_health() {
    let app = build_app().await.unwrap();
    let server = TestServer::new(app).unwrap();
    let response = server.get("/health").await;
    assert_eq!(response.status_code(), 200);
    let json = response.json::<serde_json::Value>();
    assert_eq!(json["message"], "API is healthy");
}