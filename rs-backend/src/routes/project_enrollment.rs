use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route(
        "/",
        get(get_enrollments)
            .post(create_enrollment)
            .delete(delete_enrollment),
    )
}

#[utoipa::path(
    get,
    path = "/project/{project_id}/enrollment/",
    params(("project_id", Path, description = "Project ID")),
    responses(
        (status = 200, description = "OK", body = [i32]),
        (status = 404, description = "Not Found", body = String)
    )
)]
pub async fn get_enrollments(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> Json<Vec<i32>> {
    let enrollments = sqlx::query!(
        "SELECT user_id FROM project_enrollments WHERE project_id = $1",
        project_id
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_else(|_| vec![]);
    Json(enrollments.into_iter().map(|e| e.user_id).collect())
}

#[utoipa::path(
    post,
    path = "/project/{project_id}/enrollment/",
    params(("project_id", Path, description = "Project ID")),
    request_body = i32,
    responses(
        (status = 201, description = "Created"),
        (status = 400, description = "Bad Request", body = String),
        (status = 404, description = "Not Found", body = String)
    )
)]
pub async fn create_enrollment(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(user_id): Json<i32>,
) -> Json<()> {
    sqlx::query!(
        "INSERT INTO project_enrollments (project_id, user_id) VALUES ($1, $2)",
        project_id,
        user_id
    )
    .execute(&pool)
    .await
    .unwrap();
    Json(())
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/enrollment/",
    params(("project_id", Path, description = "Project ID")),
    request_body = i32,
    responses(
        (status = 200, description = "OK"),
        (status = 400, description = "Bad Request", body = String),
        (status = 404, description = "Not Found", body = String)
    )
)]
pub async fn delete_enrollment(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(user_id): Json<i32>,
) -> Json<()> {
    sqlx::query!(
        "DELETE FROM project_enrollments WHERE project_id = $1 AND user_id = $2",
        project_id,
        user_id
    )
    .execute(&pool)
    .await
    .unwrap();
    Json(())
}
