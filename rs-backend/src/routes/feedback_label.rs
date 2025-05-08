use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new().route(
        "/{label_id}",
        get(get_labels)
            .post(create_label)
            .delete(delete_label),
    )
}

#[utoipa::path(
    get,
    path = "/project/{project_id}/feedback/{feedback_id}/labels",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    responses((status = 200, description = "OK", body = [i32]), (status = 404, description = "Not Found"))
)]
pub async fn get_labels(
    State(pool): State<PgPool>,
    Path(feedback_id): Path<i32>,
) -> Json<Vec<i32>> {
    let labels = sqlx::query!(
        "SELECT label_id FROM feedback_labels WHERE feedback_id = $1",
        feedback_id
    )
    .fetch_all(&pool)
    .await
    .unwrap_or_else(|_| vec![]);
    Json(labels.into_iter().map(|l| l.label_id).collect())
}

#[utoipa::path(
    post,
    path = "/project/{project_id}/feedback/{feedback_id}/labels",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    request_body = i32,
    responses((status = 201, description = "Created"), (status = 404, description = "Not Found"))
)]
pub async fn create_label(
    State(pool): State<PgPool>,
    Path(feedback_id): Path<i32>,
    Json(label_id): Json<i32>,
) -> Json<()> {
    sqlx::query!(
        "INSERT INTO feedback_labels (feedback_id, label_id) VALUES ($1, $2)",
        feedback_id,
        label_id
    )
    .execute(&pool)
    .await
    .unwrap();
    Json(())
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/feedback/{feedback_id}/labels",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    request_body = i32,
    responses((status = 200, description = "OK"), (status = 404, description = "Not Found"))
)]
pub async fn delete_label(
    State(pool): State<PgPool>,
    Path(feedback_id): Path<i32>,
    Json(label_id): Json<i32>,
) -> Json<()> {
    sqlx::query!(
        "DELETE FROM feedback_labels WHERE feedback_id = $1 AND label_id = $2",
        feedback_id,
        label_id
    )
    .execute(&pool)
    .await
    .unwrap();
    Json(())
}
