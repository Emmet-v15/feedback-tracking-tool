mod test_utils;
use test_utils::*;
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_feedback() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let response = server.get(&format!("/project/{}/feedback/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
    let data = response.json::<Value>();
    assert!(data.is_array() && !data.as_array().unwrap().is_empty(), "Response: {:?}", data);
}

#[tokio::test]
async fn test_create_feedback() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    // Valid feedback
    let body = json!({
        "title": "Test Feedback",
        "description": "A test feedback",
        "status": "open",
        "priority": "medium",
    });
    let response = server.post(&format!("/project/{}/feedback/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 201);
}

#[tokio::test]
async fn test_create_feedback_unauthorized() {
    let (server, _jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let body = json!({
        "title": "Test Feedback 2",
        "description": "A test feedback 2",
        "status": "open",
        "priority": "medium",
        "creator_id": 1,
        "project_id": project_id
    });
    let response = server.post(&format!("/project/{}/feedback/", project_id))
        .json(&body)
        .await;
    assert!(response.status_code() == 401);
}

#[tokio::test]
async fn test_get_feedback_by_id_not_found() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let response = server.get(&format!("/project/{}/feedback/999999", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert!(response.status_code() == 404, "Response: {:?}", response);
}

#[tokio::test]
async fn test_delete_feedback_not_found() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let response = server.delete(&format!("/project/{}/feedback/999999", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 404, "Response: {:?}", response);
}