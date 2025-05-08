use crate::models::user::User;
use axum::http::StatusCode;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/user", get(get_self_user))
        .route("/users", get(get_users))
        .route("/user/{user_id}", get(get_user_by_id))
}

#[utoipa::path(
    get,
    path = "/users",
    responses(
        (status = 200, description = "OK", body = [User]),
        (status = 403, description = "Forbidden")
    )
)]

pub async fn get_users(
    State(pool): State<PgPool>,
    auth_ctx: crate::middleware::auth::AuthContext,
) -> Result<Json<Vec<User>>, StatusCode> {
    if auth_ctx.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Ok(Json(users))
}

#[utoipa::path(
    get,
    path = "/user/{user_id}",
    params(
        ("user_id", Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "OK", body = Option<User>),
        (status = 404, description = "Not Found")
    )
)]
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    Json(user)
}


#[utoipa::path(
    get,
    path = "/user",
    responses(
        (status = 200, description = "OK", body = User),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn get_self_user(
    State(pool): State<PgPool>,
    auth_ctx: crate::middleware::auth::AuthContext,
) -> Result<Json<User>, StatusCode> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", auth_ctx.user_id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(user))
}