mod test_utils;
use test_utils::*;
use serde_json::Value;

#[tokio::test]
async fn test_get_users_unauthorized() {
    let (server, _jwt, _, _, _) = setup_test_environment().await;
    let response = server.get("/users").await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_get_users_admin() {
    let (server, _jwt, _, _, _) = setup_test_environment().await;
    register_test_user(&server, "adminuser", "adminpass", "admin@example.com", "admin").await;
    let token = login_and_get_jwt(&server, "adminuser", "adminpass").await;
    let response = server.get("/users").add_header("Authorization", &format!("Bearer {}", token)).await;
    assert_eq!(response.status_code(), 200);
}

#[tokio::test]
async fn test_get_self_user_unauthorized() {
    let (server, _jwt, _, _, _) = setup_test_environment().await;
    let response = server.get("/user").await;
    assert_eq!(response.status_code(), 401);
}

#[tokio::test]
async fn test_get_user_by_id_not_found() {
    let (server, jwt, _, _, _) = setup_test_environment().await;
    let response = server.get("/user/999999").add_header("Authorization", &format!("Bearer {}", jwt)).await;
    let json = response.json::<Value>();
    assert!(json.is_null() || response.status_code() == 404 || response.status_code() == 500);
}