use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_register_invalid_role() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!({
        "username": "testuser_invalidrole",
        "password": "testpass",
        "email": "test_invalidrole@example.com",
        "role": "invalidrole"
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_register_valid_and_login() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let username = "testuser_auth";
    let password = "testpass_auth";
    let email = "test_auth@example.com";
    let body = serde_json::json!({
        "username": username,
        "password": password,
        "email": email,
        "role": "student"
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 201);
    // Now login
    let token = login_and_get_jwt(&server, username, password).await;
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_register_duplicate_username() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let username = "testuser_dup";
    let password = "testpass_dup";
    let email = "test_dup@example.com";
    let _ = create_test_user(&pool, username, password, email, "student").await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!({
        "username": username,
        "password": password,
        "email": email,
        "role": "student"
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 409);
}

#[tokio::test]
async fn test_login_invalid_user() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!({
        "username": "nonexistent",
        "password": "wrongpass"
    });
    let response = server.post("/login").json(&body).await;
    assert_eq!(response.status_code(), 401);
}