mod test_utils;
use rs_backend::models::label::LabelPayload;
use test_utils::*;

#[tokio::test]
async fn test_add_label_to_feedback() {
    let (server, jwt, _user, project_id, feedback_id) = setup_test_environment().await;
    let response = server
        .post(&format!("/project/{}/feedback/{}/labels/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&LabelPayload { color: "red".to_string(), name: "Test Label".to_string() })
        .await;
    let id = response.json::<serde_json::Value>();
    let label_id = id["id"].as_i64().unwrap_or(0);
    assert!(label_id > 0, "Label ID should be greater than 0");
    assert_eq!(response.status_code(), 201);
}

#[tokio::test]
async fn test_remove_label_from_feedback() {
    let (server, jwt, _user, project_id, feedback_id) = setup_test_environment().await;

    let response = server
        .post(&format!("/project/{}/feedback/{}/labels/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&LabelPayload { color: "red".to_string(), name: "Test Label".to_string() })
        .await;
    let id = response.json::<serde_json::Value>();
    let label_id = id["id"].as_i64().unwrap_or(0);
    assert!(label_id > 0, "Label ID should be greater than 0");
    assert_eq!(response.status_code(), 201);

    // Remove label
    let response = server
        .delete(&format!("/project/{}/feedback/{}/labels/{}", project_id, feedback_id, label_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_labels_for_feedback() {
    let (server, jwt, _user, project_id, feedback_id) = setup_test_environment().await;
    let response = server
        .get(&format!("/project/{}/feedback/{}/labels/", project_id, feedback_id))
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 200);
    let data = response.json::<serde_json::Value>();
    assert!(data.is_array() && data.as_array().unwrap().is_empty(), "Response: {:?}", data);
}