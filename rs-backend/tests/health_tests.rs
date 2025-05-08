mod test_utils;
use test_utils::*;
use serde_json::Value;

#[tokio::test]
async fn test_health() {
    let (server, jwt, _user_id, _project_id, _feedback_id) = setup_test_environment().await;
    let response = server.get("/health").add_header("Authorization", &format!("Bearer {}", jwt)).await;
    assert_eq!(response.status_code(), 200);
    let json = response.json::<Value>();
    assert_eq!(json["message"], "API is healthy");
}