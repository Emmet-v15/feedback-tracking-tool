use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_feedback_empty() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    // Create a user and project
    let user_id = create_test_user(&pool, "fb_owner", "fbpass", "fb_owner@example.com", "student").await;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Feedback Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get(&format!("/project/{}/feedback/", project_id)).await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_feedback_valid_and_invalid() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let user_id = create_test_user(&pool, "fb_creator", "fbpass2", "fb_creator@example.com", "student").await;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Feedback Project 2", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Valid feedback
    let body = serde_json::json!({
        "title": "Test Feedback",
        "description": "A test feedback",
        "status": "open",
        "priority": "medium",
        "creator_id": user_id,
        "project_id": project_id
    });
    let response = server.post(&format!("/project/{}/feedback/", project_id)).json(&body).await;
    assert_eq!(response.status_code(), 201);
    // Invalid creator_id
    let body = serde_json::json!({
        "title": "Test Feedback 2",
        "description": "A test feedback 2",
        "status": "open",
        "priority": "medium",
        "creator_id": 99999999,
        "project_id": project_id
    });
    let response = server.post(&format!("/project/{}/feedback/", project_id)).json(&body).await;
    assert!(response.status_code() == 500);
}

#[tokio::test]
async fn test_get_feedback_by_id_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get("/project/1/feedback/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}

#[tokio::test]
async fn test_delete_feedback_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.delete("/project/1/feedback/999999").await;
    let json = response.json::<serde_json::Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}