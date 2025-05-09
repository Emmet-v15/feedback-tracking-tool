use crate::models::comment::{Comment, CommentPayload, CommentWithUser};
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, put},
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_comments).post(create_comment))
        .route("/{comment_id}", put(put_comment).delete(delete_comment))
}

#[utoipa::path(
    get,
    path = "/project/{project_id}/feedback/{feedback_id}/comments/",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID")
    ),
    responses(
        (status = 200, description = "OK", body = [CommentWithUser]),
        (status = 404, description = "Not Found")
    )
)]
pub async fn get_comments(
    State(pool): State<PgPool>,
    Path((project_id, feedback_id)): Path<(i32, i32)>,
) -> impl axum::response::IntoResponse {
    let feedback_exists_raw = sqlx::query!("SELECT id FROM feedback WHERE id = $1 AND project_id = $2", feedback_id, project_id)
        .fetch_optional(&pool)
        .await;
    let feedback_exists = feedback_exists_raw.unwrap().is_some();
        if !feedback_exists {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Feedback not found"}))).into_response();
    }
    match sqlx::query_as!(
        CommentWithUser,
        r#"SELECT c.id, c.content, c.created_at, c.user_id, c.feedback_id, u.username
            FROM comments c
            JOIN users u ON c.user_id = u.id
            WHERE c.feedback_id = $1"#,
        feedback_id
    )
    .fetch_all(&pool)
    .await {
        Ok(comments) => {
            if comments.is_empty() {
                return (StatusCode::OK, Json(Vec::<CommentWithUser>::new())).into_response();
            }
            Json(comments).into_response()
        },
        Err(e) => {
            eprintln!("DB error in get_comments: {e:?}");
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": "Failed to fetch comments"}))).into_response()
        }
    }
}

// Handler to create a new comment; parsing JSON dynamically
#[utoipa::path(
    post,
    path = "/project/{project_id}/feedback/{feedback_id}/comments/",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID")
    ),
    request_body = Comment,
    responses(
        (status = 201, description = "Created", body = Comment),
        (status = 400, description = "Bad Request")
    )
)]
pub async fn create_comment(
    State(pool): State<PgPool>,
    Path((_project_id, feedback_id)): Path<(i32, i32)>,
    auth_ctx: crate::middleware::auth::AuthContext,
    Json(payload): Json<serde_json::Value>,
) -> impl IntoResponse {
    let content = payload
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    if content.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Missing or empty content"})),
        )
        .into_response();
    }

    let comment = sqlx::query_as!(
        Comment,
        "INSERT INTO comments (content, user_id, feedback_id) VALUES ($1, $2, $3) RETURNING *",
        content,
        auth_ctx.user_id,
        feedback_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(comment)).into_response()
}

#[utoipa::path(
    put,
    path = "/project/{project_id}/feedback/{feedback_id}/comments/{comment_id}",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID"),
        ("comment_id", Path, description = "Comment ID")
    ),
    request_body = Comment,
    responses(
        (status = 200, description = "OK", body = Option<Comment>),
        (status = 404, description = "Not Found")
    )
)]
pub async fn put_comment(
    State(pool): State<PgPool>,
    Path((_project_id, feedback_id, comment_id)): Path<(i32, i32, i32)>,
    auth_ctx: crate::middleware::auth::AuthContext,
    Json(payload): Json<CommentPayload>,
) -> impl IntoResponse {
    // Extract user_id from auth_ctx
    let user_id = auth_ctx.user_id;

    // Check if the comment exists and belongs to the user
    let comment = sqlx::query_as!(
        Comment,
        "SELECT * FROM comments WHERE id = $1 AND feedback_id = $2 AND user_id = $3",
        comment_id,
        feedback_id,
        user_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    if comment.is_none() {
        return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Comment not found or unauthorized"}))).into_response();
    }

    // Update the comment
    let updated_comment = sqlx::query_as!(
        Comment,
        "UPDATE comments SET content = $1 WHERE id = $2 RETURNING *",
        payload.content,
        comment_id
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    Json(Some(updated_comment)).into_response()
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}/feedback/{feedback_id}/comments/{comment_id}",
    params(
        ("project_id", Path, description = "Project ID"),
        ("feedback_id", Path, description = "Feedback ID"),
        ("comment_id", Path, description = "Comment ID")
    ),
    responses(
        (status = 200, description = "OK", body = Option<Comment>),
        (status = 404, description = "Not Found")
    )
)]
pub async fn delete_comment(
    State(pool): State<PgPool>,
    Path((_project_id, _feedback_id, comment_id)): Path<(i32, i32, i32)>,
) -> impl IntoResponse {
    let comment = sqlx::query_as!(
        Comment,
        "DELETE FROM comments WHERE id = $1 RETURNING *",
        comment_id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();
    match comment {
        Some(comment) => (StatusCode::OK, Json(comment)).into_response(),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "Comment not found"}))).into_response(),
    }
}
