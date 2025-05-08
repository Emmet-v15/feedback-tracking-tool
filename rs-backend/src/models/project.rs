use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    #[schema(example = 1, read_only)]
    pub id: Option<i32>,
    #[schema(example = "Project Name")]
    pub name: String,
    #[schema(example = "A project description")]
    pub description: Option<String>,
    #[schema(example = 1)]
    pub owner_id: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
