use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct Label {
    pub id: Option<i32>,
    pub name: String,
    pub color: String,
    pub project_id: i32,
}
