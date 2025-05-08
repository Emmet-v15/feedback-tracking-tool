mod test_utils;
use test_utils::*;
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_comments_empty() {
    let (server, jwt, _user_id, project_id, feedback_id) = setup_test_environment().await;
    let response = server
        .get(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_comment_unauthorized() {
    let (server, _jwt, _user_id, project_id, feedback_id) = setup_test_environment().await;
    let body = json!({ "content": "Test comment" });
    let response = server
        .post(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_edit_comment() {
    let (server, jwt, _user_id, project_id, feedback_id) = setup_test_environment().await;
    let body = json!({ "content": "Initial comment" });
    let response = server
        .post(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 201);

    let id = response.json::<Value>();
    let comment_id = id["id"].as_i64().unwrap_or(0);
    assert!(comment_id > 0, "Comment ID should be greater than 0");

    let body = json!({ "content": "Updated comment" });
    let response2 = server
        .put(&format!("/project/{}/feedback/{}/comments/{}", project_id, feedback_id, comment_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response2.status_code(), 200);
}

#[tokio::test]
async fn test_edit_comment_unauthorized() {
    let (server, jwt1, _user_id, project_id, feedback_id) = setup_test_environment().await;
    register_test_user(&server, "user2", "password", "user2@email.com", "student").await;
    let jwt2 = login_and_get_jwt(&server, "user2", "password").await;

    let body = json!({ "content": "Initial comment" });
    let response = server
        .post(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt1))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 201);
    let id = response.json::<Value>();
    let comment_id = id["id"].as_i64().unwrap_or(0);
    assert!(comment_id > 0, "Comment ID should be greater than 0");

    let body = json!({ "content": "Updated comment" });
    let response2 = server
        .put(&format!("/project/{}/feedback/{}/comments/{}", project_id, feedback_id, comment_id))
        .add_header("Authorization", &format!("Bearer {}", jwt2))
        .json(&body)
        .await;
    assert!(response2.status_code() == 401 || response2.status_code() == 404);
}

#[tokio::test]
async fn test_create_comment() {
    let (server, jwt, _user_id, project_id, feedback_id) = setup_test_environment().await;
    let body = json!({ "content": "Test comment" });
    let response = server
        .post(&format!("/project/{}/feedback/{}/comments/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    if response.status_code() != 404 {
        let id = response.json::<Value>();
        let comment_id = id["id"].as_i64().unwrap_or(0);
        assert!(comment_id > 0, "Comment ID should be greater than 0");
    }
    assert_eq!(response.status_code(), 201);
}

#[tokio::test]
async fn test_put_comment_not_found() {
    let (server, jwt, _user_id, project_id, feedback_id) = setup_test_environment().await;
    let body = json!({ "content": "Updated comment" });
    let response = server
        .put(&format!("/project/{}/feedback/{}/comments/999999", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    if response.status_code() != 404 {
        let _ = response.json::<Value>();
    }
    assert!(response.status_code() == 404 || response.status_code() == 500 || response.status_code() == 401);
}

#[tokio::test]
async fn test_delete_comment_not_found() {
    let (server, jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let response = server
        .delete("/project/1/feedback/1/comments/999999")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    if response.status_code() != 404 {
        let _ = response.json::<Value>();
    }
    assert!(response.status_code() == 404 || response.status_code() == 500 || response.status_code() == 401);
}