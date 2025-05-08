use crate::models::feedback::Feedback;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::PgPool;

use super::{feedback_comment, feedback_label};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_feedback).post(create_feedback))
        .route(
            "/{feedback_id}",
            get(get_feedback_by_id).delete(delete_feedback),
        )
        .nest("/{feedback_id}/labels", feedback_label::routes())
        .nest(
            "/{feedback_id}/comments/",
            feedback_comment::routes(),
        )
}

// Example: retrieve a list of feedback
#[utoipa::path(
    get,
    path = "/project/{project_id}/feedback/",
    params(("project_id", Path, description = "Project ID")),
    responses((status = 200, description = "OK", body = [Feedback]))
)]
pub async fn get_feedback(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
) -> impl IntoResponse {
    println!("Fetching feedback.");
    match sqlx::query_as!(Feedback, "SELECT * FROM feedback WHERE project_id = $1", project_id)
        .fetch_all(&pool)
        .await {
        Ok(feedbacks) => Json(feedbacks).into_response(),
        Err(e) => {
            eprintln!("DB error in get_feedback: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch feedback").into_response()
        }
    }
}

// Example: create feedback
#[utoipa::path(
    post,
    path = "/project/{project_id}/feedback/",
    params(("project_id", Path, description = "Project ID")),
    request_body = Feedback,
    responses((status = 201, description = "Created", body = Feedback), (status = 500, description = "Internal Server Error"))
)]
pub async fn create_feedback(
    State(pool): State<PgPool>,
    Path(project_id): Path<i32>,
    Json(payload): Json<Feedback>,
) -> impl axum::response::IntoResponse {
    let result = sqlx::query_as!(Feedback, "INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", 
        payload.title, payload.description, payload.status, payload.priority, payload.creator_id, project_id)
        .fetch_one(&pool)
        .await;
    match result {
        Ok(feedback) => (StatusCode::CREATED, Json(feedback)).into_response(),
        Err(e) => {
            eprintln!("DB error creating feedback: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create feedback").into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/project/{project_id}/feedback/{feedback_id}",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    responses((status = 200, description = "OK", body = Feedback), (status = 404, description = "Not Found"))
)]
pub async fn get_feedback_by_id(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> Json<Option<Feedback>> {
    let feedback = sqlx::query_as!(Feedback, "SELECT * FROM feedback WHERE id = $1 AND project_id = $2", feedback_id, project_id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    Json(feedback)
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/feedback/{feedback_id}",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    responses((status = 200, description = "OK", body = Feedback), (status = 404, description = "Not Found"))
)]
pub async fn delete_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> Json<Option<Feedback>> {
    let feedback = sqlx::query_as!(
        Feedback,
        "DELETE FROM feedback WHERE id = $1 AND project_id = $2 RETURNING *",
        feedback_id, project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();
    Json(feedback)
}
