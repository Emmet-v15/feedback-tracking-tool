use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_projects_empty() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/project/").await;
    assert_eq!(response.status_code(), 200);
    let _json = response.json::<serde_json::Value>();
}

#[tokio::test]
async fn test_create_project_valid_and_invalid_owner() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    // Create a valid user
    let user_id = create_test_user(&pool, "proj_owner", "projpass", "proj_owner@example.com", "student").await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Valid owner
    let body = serde_json::json!({
        "name": "Test Project",
        "description": "A test project",
        "owner_id": user_id
    });
    let response = server.post("/project/").json(&body).await;
    assert_eq!(response.status_code(), 201);
    // Invalid owner
    let body = serde_json::json!({
        "name": "Test Project 2",
        "description": "A test project 2",
        "owner_id": 99999999
    });
    let response = server.post("/project/").json(&body).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_get_project_by_id_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/project/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}

#[tokio::test]
async fn test_update_project_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!({
        "name": "Updated Project",
        "description": "Updated description",
        "owner_id": 1,
        "updated_at": null
    });
    let response = server.put("/project/999999").json(&body).await;
    assert!(response.status_code() == 500 || response.status_code() == 404);
}

#[tokio::test]
async fn test_delete_project_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.delete("/project/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}