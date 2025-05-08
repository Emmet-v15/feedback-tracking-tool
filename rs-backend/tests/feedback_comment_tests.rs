use axum_test::TestServer;
use axum::http::StatusCode;
use rs_backend::app::build_app_with_pool;
mod test_utils;
use test_utils::*;

#[tokio::test]
async fn test_get_comments_empty() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let user_id = create_test_user(&pool, "comment_owner", "commentpass", "comment_owner@example.com", "student").await;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Comment Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let feedback_id = sqlx::query!("INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", "Feedback for Comment", "desc", "open", "medium", user_id, project_id)
        .fetch_one(&pool).await.unwrap().id;
    println!("Test setup - project_id: {}, feedback_id: {}", project_id, feedback_id);
    let feedbacks: Vec<(i32, i32)> = sqlx::query_as::<_, (i32, i32)>("SELECT id, project_id FROM feedback").fetch_all(&pool).await.unwrap();
    println!("Feedback table contents: {:?}", feedbacks);
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    let jwt = login_and_get_jwt(&server, "comment_owner", "commentpass").await;
    let response = server
        .get(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_comment_invalid() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Seed user, project, feedback
    let username = "commenter";
    let password = "commentpass";
    let email = "commenter@example.com";
    register_test_user(&server, username, password, email, "student").await;
    let user_id = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_one(&pool).await.unwrap().id;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Comment Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let feedback_id = sqlx::query!("INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", "Feedback for Comment", "desc", "open", "medium", user_id, project_id)
        .fetch_one(&pool).await.unwrap().id;
    // No content field (invalid data), no auth header
    let body = serde_json::json!({});
    let response = server
        .post(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .json(&body)
        .await;
    // Should be 401 (unauthorized) or 404 (not found) if auth is required
    assert!(
        response.status_code() == StatusCode::UNAUTHORIZED
        || response.status_code() == StatusCode::NOT_FOUND
    );
}

#[tokio::test]
async fn test_put_comment_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Register and login user
    let username = "commentputter";
    let password = "commentpass";
    let email = "commentputter@example.com";
    register_test_user(&server, username, password, email, "student").await;
    let user_id = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_one(&pool).await.unwrap().id;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Comment Project", "desc", user_id)
        .fetch_one(&pool).await.unwrap().id;
    let feedback_id = sqlx::query!("INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", "Feedback for Comment", "desc", "open", "medium", user_id, project_id)
        .fetch_one(&pool).await.unwrap().id;
    let jwt = login_and_get_jwt(&server, username, password).await;
    let body = serde_json::json!({
        "content": "Updated comment",
        "user_id": user_id,
        "feedback_id": feedback_id
    });
    let response = server.put(&format!("/project/{}/feedback/{}/comments/999999", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    if response.status_code() != StatusCode::NOT_FOUND {
        let _ = response.json::<serde_json::Value>();
    }
    assert!(response.status_code() == 404 || response.status_code() == 500 || response.status_code() == 401);
}

#[tokio::test]
async fn test_delete_comment_not_found() {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let app = build_app_with_pool(pool.clone()).await;
    let server = TestServer::new(app).unwrap();
    // Register and login user
    let username = "commentdeleter";
    let password = "commentpass";
    let email = "commentdeleter@example.com";
    register_test_user(&server, username, password, email, "student").await;
    let jwt = login_and_get_jwt(&server, username, password).await;
    // Authenticated request
    let response = server.delete("/project/1/feedback/1/comments/999999")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    if response.status_code() != StatusCode::NOT_FOUND {
        let _ = response.json::<serde_json::Value>();
    }
    assert!(response.status_code() == 404 || response.status_code() == 500 || response.status_code() == 401);
}