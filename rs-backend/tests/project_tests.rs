mod test_utils;
use test_utils::*;
use serde_json::{json, Value};

#[tokio::test]
async fn test_get_projects() {
    let (server, jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let response = server.get("/project/")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    let json = response.json::<Value>();
    assert!(json.is_array() && !json.as_array().unwrap().is_empty(), "Response: {:?}", json);
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_create_project() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    register_test_user(&server, "teacher", "password", "teach@gmail.com", "teacher").await;
    let jwt = login_and_get_jwt(&server, "teacher", "password").await;

    let body = json!({
        "name": "Test Project",
        "description": "A test project"
    });
    let response = server.post("/project/")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 201);
}

#[tokio::test]
async fn test_create_project_unauthorized() {
    let (server, jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let body = json!({
        "name": "Test Project 2",
        "description": "A test project 2"
    });
    let response = server.post("/project/")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_get_project_by_id_not_found() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    register_test_user(&server, "teacher", "password", "teach@gmail.com", "teacher").await;
    let jwt = login_and_get_jwt(&server, "teacher", "password").await;
    let response = server.get("/project/999999")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 404);
}

#[tokio::test]
async fn test_update_project_not_found() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    register_test_user(&server, "teacher", "password", "teach@gmail.com", "teacher").await;
    let jwt = login_and_get_jwt(&server, "teacher", "password").await;
    let body = json!({
        "name": "Updated Project",
        "description": "Updated description"
    });
    let response = server.put("/project/999999")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .json(&body)
        .await;
    assert_eq!(response.status_code(), 404, "Response: {:?}", response);
}

#[tokio::test]
async fn test_delete_project_not_found() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    register_test_user(&server, "teacher", "password", "teach@gmail.com", "teacher").await;
    let jwt = login_and_get_jwt(&server, "teacher", "password").await;
    let response = server.delete("/project/999999")
        .add_header("Authorization", &format!("Bearer {}", jwt))
        .await;
    assert_eq!(response.status_code(), 404, "Response: {:?}", response);
}