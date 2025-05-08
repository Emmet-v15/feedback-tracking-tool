mod test_utils;
use test_utils::*;
use serde_json::json;

#[tokio::test]
async fn test_get_enrollments_empty() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let response = server
        .get(&format!("/project/{}/enrollment/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_enrollment_invalid() {
    let (server, jwt, _user_id, project_id, _feedback_id) = setup_test_environment().await;
    let body = json!(999999);
    let response = server
        .post(&format!("/project/{}/enrollment/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert!(response.status_code() == 400);
}

#[tokio::test]
async fn test_create_enrollment() {
    let (server, jwt, user_id, project_id, _feedback_id) = setup_test_environment().await;
    let body = json!(user_id);
    let response = server
        .post(&format!("/project/{}/enrollment/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert!(response.status_code() == 201);
}

#[tokio::test]
async fn test_delete_enrollment() {
    let (server, jwt, user_id, project_id, _feedback_id) = setup_test_environment().await;

    let body = json!(user_id);
    let response = server
        .post(&format!("/project/{}/enrollment/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert!(response.status_code() == 201);

    let response = server
        .delete(&format!("/project/{}/enrollment/", project_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert!(response.status_code() == 200);
}