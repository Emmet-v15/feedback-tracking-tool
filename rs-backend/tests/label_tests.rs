use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_labels_empty() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/label").await;
    assert_eq!(response.status_code(), 200);
    let _json = response.json::<serde_json::Value>();
}

#[tokio::test]
async fn test_create_label_valid_and_invalid() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    // Create a valid user and project
    let user_id = create_test_user(&pool, "label_owner", "labelpass", "label_owner@example.com", "student").await;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Label Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Valid label
    let body = serde_json::json!({
        "name": "Test Label",
        "color": "#FFFFFF",
        "project_id": project_id
    });
    let response = server.post("/label").json(&body).await;
    assert_eq!(response.status_code(), 201);
    // Invalid project_id
    let body = serde_json::json!({
        "name": "Test Label 2",
        "color": "#000000",
        "project_id": 99999999
    });
    let response = server.post("/label").json(&body).await;
    assert!(response.status_code() == 500);
}

#[tokio::test]
async fn test_get_label_by_id_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/label/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}

#[tokio::test]
async fn test_delete_label_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.delete("/label/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}