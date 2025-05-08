use crate::models::project::{Project, ProjectPayload};
use crate::middleware::auth::AuthContext;
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::get,
    response::IntoResponse,
    http::StatusCode,
};
use sqlx::PgPool;

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/", get(get_projects).post(create_project))
        .route(
            "/{project_id}",
            get(get_project_by_id).put(update_project).delete(delete_project),
        )
        .nest("/{project_id}/enrollment/", super::project_enrollment::routes())
        .nest("/{project_id}/feedback/", super::feedback::routes())
    }

#[utoipa::path(
    get,
    path = "/project/",
    responses(
        (status = 200, description = "OK", body = [Project])
    )
)]
pub async fn get_projects(State(pool): State<PgPool>) -> Json<Vec<Project>> {
    println!("Fetching all projects");
    let projects = sqlx::query_as!(Project, "SELECT * FROM projects")
        .fetch_all(&pool)
        .await
        .unwrap_or_else(|_| vec![]);
    Json(projects)
}

#[utoipa::path(
    post,
    path = "/project/",
    request_body = ProjectPayload,
    responses(
        (status = 201, description = "Created", body = Project),
        (status = 400, description = "Bad Request"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn create_project(
    State(pool): State<PgPool>,
    auth_ctx: AuthContext,
    Json(payload): Json<ProjectPayload>,
) -> impl IntoResponse {
    // only teachers or admins can create
    if auth_ctx.role != "teacher" && auth_ctx.role != "admin" {
        return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response();
    }

    // 1) ensure owner exists
    let owner_exists: bool = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
        auth_ctx.user_id
    )
    .fetch_one(&pool)
    .await
    .unwrap_or(None)
    .unwrap_or(false);

    if !owner_exists {
        return (StatusCode::BAD_REQUEST, "owner_id not found".to_string()).into_response();
    }

    // 2) try insert and map errors to 500
    match sqlx::query_as!(
        Project,
        "INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING *",
        payload.name,
        payload.description,
        auth_ctx.user_id
    )
    .fetch_one(&pool)
    .await
    {
        Ok(project) => (StatusCode::CREATED, Json(project)).into_response(),
        Err(e) => {
            eprintln!("DB error creating project: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to create project".to_string(),
            )
                .into_response()
        }
    }
}

#[utoipa::path(
    get,
    path = "/project/{project_id}",
    params(("project_id", Path, description = "Project ID")),
    responses(
        (status = 200, description = "OK", body = Project),
        (status = 404, description = "Not Found")
    )
)]
pub async fn get_project_by_id(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let project = sqlx::query_as!(Project, "SELECT * FROM projects WHERE id = $1", id)
        .fetch_optional(&pool)
        .await
        .unwrap();

    match project {
        Some(project) => (StatusCode::OK, Json(project)).into_response(),
        None => (StatusCode::NOT_FOUND, "Project not found".to_string()).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/project/{project_id}",
    params(("project_id", Path, description = "Project ID")),
    request_body = ProjectPayload,
    responses(
        (status = 200, description = "OK", body = Project),
        (status = 404, description = "Not Found")
    )
)]
pub async fn update_project(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    Json(payload): Json<ProjectPayload>,
) -> impl IntoResponse {
    let project = sqlx::query_as!(
        Project,
        "UPDATE projects SET name = $1, description = $2 WHERE id = $3 RETURNING *",
        payload.name,
        payload.description,
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match project {
        Some(project) => (StatusCode::OK, Json(project)).into_response(),
        None => (StatusCode::NOT_FOUND, "Project not found".to_string()).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/project/{project_id}",
    params(("project_id", Path, description = "Project ID")),
    responses(
        (status = 200, description = "OK", body = Project),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Not Found")
    )
)]
pub async fn delete_project(
    State(pool): State<PgPool>,
    auth_ctx: AuthContext,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    // only teachers or admins can delete
    if auth_ctx.role != "teacher" && auth_ctx.role != "admin" {
        return (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()).into_response();
    }

    let project = sqlx::query_as!(
        Project,
        "DELETE FROM projects WHERE id = $1 RETURNING *",
        id
    )
    .fetch_optional(&pool)
    .await
    .unwrap();

    match project {
        Some(project) => (StatusCode::OK, Json(project)).into_response(),
        None => (StatusCode::NOT_FOUND, "Project not found".to_string()).into_response(),
    }
}