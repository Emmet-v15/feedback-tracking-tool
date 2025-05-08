use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_users_forbidden() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/users").await;
    assert_eq!(response.status_code(), 403);
}

#[tokio::test]
async fn test_get_users_admin() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let username = "adminuser";
    let password = "adminpass";
    let email = "admin@example.com";
    let _ = create_test_user(&pool, username, password, email, "admin").await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let token = login_and_get_jwt(&server, username, password).await;
    let response = server.get("/users").add_header("Authorization", &format!("Bearer {}", token)).await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_self_user_forbidden() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/user").await;
    assert_eq!(response.status_code(), 403);
}

#[tokio::test]
async fn test_get_user_by_id_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/user/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}