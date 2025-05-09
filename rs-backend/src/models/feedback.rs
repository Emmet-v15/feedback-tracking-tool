use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::label::Label;

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

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackResponse {
    pub feedback: Feedback,
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackPayload {
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
}