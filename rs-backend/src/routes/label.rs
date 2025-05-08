use crate::models::label::Label;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
    response::IntoResponse,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/label", get(get_labels).post(create_label))
        .route(
            "/label/{label_id}",
            get(get_label_by_id).delete(delete_label),
        )
}

#[utoipa::path(
    get,
    path = "/label",
    responses(
        (status = 200, description = "OK", body = [Label])
    )
)]
pub async fn get_labels(State(pool): State<PgPool>) -> axum::response::Response {
    match sqlx::query_as!(Label, "SELECT * FROM labels")
        .fetch_all(&pool)
        .await {
        Ok(labels) => Json(labels).into_response(),
        Err(e) => {
            eprintln!("DB error in get_labels: {e:?}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch labels").into_response()
        }
    }
}

#[utoipa::path(
    post,
    path = "/label",
    request_body = Label,
    responses(
        (status = 201, description = "Created", body = Label)
    )
)]
pub async fn create_label(State(pool): State<PgPool>, Json(payload): Json<Label>) -> axum::response::Response {
    match sqlx::query_as!(
        Label,
        "INSERT INTO labels (name, color, project_id) VALUES ($1, $2, $3) RETURNING *",
        payload.name,
        payload.color,
        payload.project_id
    )
    .fetch_one(&pool)
    .await {
        Ok(label) => (axum::http::StatusCode::CREATED, Json(label)).into_response(),
        Err(e) => {
            eprintln!("DB error in create_label: {e:?}");
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to create label").into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/label/{id}",
    params(("id", Path, description = "Label ID")),
    responses(
        (status = 200, description = "OK", body = Label),
        (status = 404, description = "Not Found")
    )
)]
pub async fn get_label_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> Json<Option<Label>> {
    let label = sqlx::query_as!(Label, "SELECT * FROM labels WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    Json(label)
}

#[utoipa::path(
    delete,
    path = "/label/{id}",
    params(("id", Path, description = "Label ID")),
    responses(
        (status = 200, description = "OK", body = Label),
        (status = 404, description = "Not Found")
    )
)]
pub async fn delete_label(State(pool): State<PgPool>, Path(id): Path<i32>) -> Json<Option<Label>> {
    let label = sqlx::query_as!(Label, "DELETE FROM labels WHERE id = $1 RETURNING *", id)
        .fetch_optional(&pool)
        .await
        .unwrap();
    Json(label)
}
