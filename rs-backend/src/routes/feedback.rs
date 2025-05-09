use crate::{middleware::auth::AuthContext, models::feedback::{Feedback, FeedbackPayload}};
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
            "/{feedback_id}/",
            get(get_feedback_by_id).delete(delete_feedback).put(update_feedback),
        )
        .nest("/{feedback_id}/labels/", feedback_label::routes())
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
    auth_ctx: AuthContext,
    Json(payload): Json<FeedbackPayload>,
) -> impl IntoResponse {
    let result = sqlx::query_as!(Feedback, "INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *", 
        payload.title, payload.description, payload.status, payload.priority, auth_ctx.user_id, project_id)
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
    path = "/project/{project_id}/feedback/{feedback_id}/",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    responses((status = 200, description = "OK", body = Feedback), (status = 404, description = "Not Found"))
)]
pub async fn get_feedback_by_id(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let feedback = sqlx::query_as!(Feedback, "SELECT * FROM feedback WHERE id = $1 AND project_id = $2", feedback_id, project_id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    match feedback {
        Some(f) => Json(f).into_response(),
        None => (StatusCode::NOT_FOUND, "Feedback not found").into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/feedback/{feedback_id}/",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    responses((status = 200, description = "OK", body = Feedback), (status = 404, description = "Not Found"))
)]
pub async fn delete_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let feedback = sqlx::query_as!(
        Feedback,
        "SELECT * FROM feedback WHERE id = $1 AND project_id = $2",
        feedback_id, project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if feedback.is_none() {
        return (StatusCode::NOT_FOUND, "Feedback not found").into_response();
    }

    let deleted_feedback = sqlx::query_as!(
        Feedback,
        "DELETE FROM feedback WHERE id = $1 AND project_id = $2 RETURNING *",
        feedback_id, project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match deleted_feedback {
        Some(f) => (StatusCode::OK, Json(f)).into_response(),
        None => (StatusCode::NOT_FOUND, "Feedback not found").into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/project/{project_id}/feedback/{feedback_id}/",
    params(("project_id", Path, description = "Project ID"), ("feedback_id", Path, description = "Feedback ID")),
    request_body = FeedbackPayload,
    responses((status = 200, description = "OK", body = Feedback), (status = 404, description = "Not Found"))
)]
pub async fn update_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
    Json(payload): Json<FeedbackPayload>,
) -> impl IntoResponse {
    let feedback = sqlx::query_as!(
        Feedback,
        "SELECT * FROM feedback WHERE id = $1 AND project_id = $2",
        feedback_id, project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if feedback.is_none() {
        return (StatusCode::NOT_FOUND, "Feedback not found").into_response();
    }

    let updated_feedback = sqlx::query_as!(
        Feedback,
        "UPDATE feedback SET title = $1, description = $2, status = $3, priority = $4 WHERE id = $5 AND project_id = $6 RETURNING *",
        payload.title, payload.description, payload.status, payload.priority, feedback_id, project_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::OK, Json(updated_feedback)).into_response()
}