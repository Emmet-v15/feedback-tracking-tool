use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Comment {
    #[schema(example = 1, read_only)]
    pub id: Option<i32>,
    #[schema(example = "A comment")]
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[schema(example = 1)]
    pub user_id: i32,
    #[schema(example = 1)]
    pub feedback_id: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentWithUser {
    pub id: Option<i32>,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub user_id: i32,
    pub feedback_id: i32,
    pub username: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CommentPayload {
    pub content: String,
}