mod test_utils;
use test_utils::*;
use serde_json::json;

#[tokio::test]
async fn test_register_invalid_role() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let body = json!({
        "username": "testuser_invalidrole",
        "password": "testpass",
        "email": "test_invalidrole@example.com",
        "role": "invalidrole"
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 400);
}

#[tokio::test]
async fn test_register_and_login() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let username = "testuser_auth";
    let password = "testpass_auth";
    let email = "test_auth@example.com";
    let body = json!({
        "username": username,
        "password": password,
        "email": email,
        "role": "student"
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 201);
    let token = login_and_get_jwt(&server, username, password).await;
    assert!(!token.is_empty());
}

#[tokio::test]
async fn test_register_duplicate_username() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let username = "testuser_dup";
    let password = "testpass_dup";
    let email = "test_dup@example.com";
    let body = json!({
        "username": username,
        "password": password,
        "email": email,
        "role": "student"
    });
    // first registration should succeed
    let response1 = server.post("/register").json(&body).await;
    assert_eq!(response1.status_code(), 201);
    // duplicate registration should fail
    let response2 = server.post("/register").json(&body).await;
    assert_eq!(response2.status_code(), 409);
}

#[tokio::test]
async fn test_login_invalid_user() {
    let (server, _jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let body = json!({
        "username": "nonexistent",
        "password": "wrongpass"
    });
    let response = server.post("/login").json(&body).await;
    assert_eq!(response.status_code(), 401);
}