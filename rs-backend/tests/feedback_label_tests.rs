use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_feedback_labels_empty() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let user_id = create_test_user(&pool, "label_fb_owner", "labelfbpass", "label_fb_owner@example.com", "student").await;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "LabelFB Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let feedback_id = sqlx::query!("INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", "Feedback for Label", "desc", "open", "medium", user_id, project_id)
        .fetch_one(&pool).await.unwrap().id;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let response = server.get(&format!("/project/{}/feedback/{}/labels", project_id, feedback_id)).await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_feedback_label_invalid() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!(999999); // label_id
    let response = server.post("/project/1/feedback/1/labels").json(&body).await;
    assert!(response.status_code() == 500);
}

#[tokio::test]
async fn test_delete_feedback_label_invalid() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let body = serde_json::json!(999999); // label_id
    let response = server.delete("/project/1/feedback/1/labels").json(&body).await;
    assert!(response.status_code() == 500);
}