use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Feedback {
    #[schema(example = 1, read_only)]
    pub id: Option<i32>,
    #[schema(example = "Feedback title")]
    pub title: Option<String>,
    #[schema(example = "Feedback description")]
    pub description: Option<String>,
    #[schema(example = "open")]
    pub status: Option<String>,
    #[schema(example = "medium")]
    pub priority: Option<String>,
    #[schema(example = 1)]
    pub creator_id: i32,
    #[schema(example = 1)]
    pub project_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
