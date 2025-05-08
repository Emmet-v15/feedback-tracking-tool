use serde::{Deserialize, Serialize};
use sqlx::types::chrono::{DateTime, Utc};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    #[schema(example = 1, read_only)]
    pub id: Option<i32>,
    #[schema(example = "johndoe")]
    pub username: String,
    #[serde(skip_serializing)]
    #[schema(example = "hashedpassword")]
    pub password_hash: String,
    #[schema(example = "john@example.com")]
    pub email: String,
    #[schema(example = "student")]
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub email: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Claims {
    pub sub: i32, // user id
    pub exp: usize,
    pub username: String,
    pub role: String,
}