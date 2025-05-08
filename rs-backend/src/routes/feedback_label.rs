use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, delete},
    Json, Router,
};
use sqlx::PgPool;
use crate::models::label::{Label, LabelPayload};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_labels_for_feedback).post(add_label_to_feedback))
        .route("/{label_id}", delete(remove_label_from_feedback))
}

#[utoipa::path(
    get,
    path = "/project/{project_id}/feedback/{feedback_id}/labels/",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID")
    ),
    responses(
        (status = 200, description = "Labels retrieved successfully", body = [Label]),
        (status = 404, description = "Feedback not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn get_labels_for_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> impl IntoResponse {
    // Check if feedback exists
    let feedback_exists = sqlx::query!(
        "SELECT id FROM feedback WHERE id = $1 AND project_id = $2",
        feedback_id,
        project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_some();

    if !feedback_exists {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Feedback not found"}))).into_response();
    }

    // Retrieve labels associated with feedback
    match sqlx::query_as!(
        Label,
        "SELECT l.* FROM labels l
         JOIN feedback_labels fl ON l.id = fl.label_id
         WHERE fl.feedback_id = $1",
        feedback_id
    )
    .fetch_all(&pool)
    .await
    {
        Ok(labels) => (StatusCode::OK, Json(labels)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": "Failed to retrieve labels"})),
        )
            .into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/project/{project_id}/feedback/{feedback_id}/labels/",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID")
    ),
    request_body = i32,
    responses(
        (status = 201, description = "Label added to feedback"),
        (status = 404, description = "Feedback or label not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn add_label_to_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
    Json(payload): Json<LabelPayload>,
) -> impl IntoResponse {
    // Check if feedback exists
    let feedback_exists = sqlx::query!(
        "SELECT id FROM feedback WHERE id = $1 AND project_id = $2",
        feedback_id,
        project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_some();

    if !feedback_exists {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Feedback not found"}))).into_response();
    }

    // else, make a label and associate it with feedback
    let label = sqlx::query!(
        "INSERT INTO labels (name, color, project_id) VALUES ($1, $2, $3) RETURNING id",
        payload.name,
        payload.color,
        project_id
    )
    .fetch_one(&pool)
    .await;

    let label_id = match label {
        Ok(label) => label.id,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to create label"}))).into_response(),
    };

    let label = sqlx::query!(
        "INSERT INTO feedback_labels (feedback_id, label_id) VALUES ($1, $2)",
        feedback_id,
        label_id
    )
    .execute(&pool)
    .await;

    if label.is_err() {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to add label to feedback"}))).into_response();
    }

    (StatusCode::CREATED, Json(serde_json::json!({"message": "Label added to feedback", "id": label_id }))).into_response()
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/feedback/{feedback_id}/labels/{label_id}",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID"),
        ("label_id", Path, description = "Label ID")
    ),
    responses(
        (status = 200, description = "Label removed from feedback"),
        (status = 404, description = "Feedback or label not found"),
        (status = 500, description = "Internal server error")
    )
)]
pub async fn remove_label_from_feedback(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id, label_id)): Path<(i32, i32, i32)>,
) -> impl IntoResponse {
    // Check if feedback exists
    let feedback_exists = sqlx::query!(
        "SELECT id FROM feedback WHERE id = $1 AND project_id = $2",
        feedback_id,
        project_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_some();

    if !feedback_exists {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Feedback not found"}))).into_response();
    }

    // Check if label is associated with feedback
    let label_association_exists = sqlx::query!(
        "SELECT * FROM feedback_labels WHERE feedback_id = $1 AND label_id = $2",
        feedback_id,
        label_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap()
    .is_some();

    if !label_association_exists {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Label not associated with feedback"}))).into_response();
    }

    // Remove label from feedback
    if sqlx::query!(
        "DELETE FROM feedback_labels WHERE feedback_id = $1 AND label_id = $2",
        feedback_id,
        label_id
    )
    .execute(&pool)
    .await
    .is_err()
    {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to remove label from feedback"}))).into_response();
    }

    (StatusCode::OK, Json(serde_json::json!({"message": "Label removed from feedback"}))).into_response()
}
