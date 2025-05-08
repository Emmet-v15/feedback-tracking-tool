use std::net::SocketAddr;

use axum::{Router, middleware};
use sqlx::postgres::PgPoolOptions;
use utoipa::OpenApi;
use utoipa::openapi::Components;
use utoipa::openapi::security::{Http, HttpAuthScheme, SecurityScheme, SecurityRequirement};
use utoipa_swagger_ui::SwaggerUi;
use colored::*;

use crate::{middleware::auth::auth_middleware, routes::*};

struct SecuritySchemas;

impl utoipa::Modify for SecuritySchemas {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // make sure components exists
        let components: &mut Components =
            openapi.components.get_or_insert_with(Default::default);

        // 1) define the Bearer scheme under the key "bearerAuth"
        components
            .security_schemes
            .insert(
                "bearerAuth".to_string(),
                SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
            );

        // 2) apply a global security requirement referring to that key
        //    second arg is an empty Vec of scopes
        openapi
            .security
            .get_or_insert_with(Vec::new)
            .push(SecurityRequirement::new("bearerAuth", Vec::<String>::new()));
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        // Feedback routes
        feedback::get_feedback,
        feedback::create_feedback,
        feedback::get_feedback_by_id,
        feedback::delete_feedback,
        feedback_comment::get_comments,
        feedback_comment::create_comment,
        feedback_comment::put_comment,
        feedback_comment::delete_comment,
        feedback_label::get_labels_for_feedback,
        feedback_label::add_label_to_feedback,
        feedback_label::remove_label_from_feedback,

        // Auth routes
        auth::login,
        auth::register,

        user::get_self_user,
        user::get_user_by_id,
        user::get_users,

        project::get_projects,
        project::create_project,
        project::get_project_by_id,
        project::delete_project,

        project_enrollment::get_enrollments,
        project_enrollment::create_enrollment,
        project_enrollment::delete_enrollment,

        health::health_check,
        // user::get_users,
        // user::create_user,
        // user::get_user_by_id,
    ),
    components(
        schemas(),
        responses()
    ),
    modifiers(&SecuritySchemas)
)]


pub struct ApiDoc;

pub async fn build_app_with_pool(pool: sqlx::PgPool) -> Router {
    let swagger = SwaggerUi::new("/swagger-ui")
        .url("/api-doc/openapi.json", ApiDoc::openapi());

    let app = Router::new()
        .nest("/project/", project::routes().with_state(pool.clone()))
        .merge(user::routes().with_state(pool.clone()))
        .layer(middleware::from_fn(auth_middleware))
        .merge(health::routes())
        .merge(auth::routes().with_state(pool.clone()))
        .merge(swagger)
                .layer(middleware::from_fn(|req: axum::http::Request<axum::body::Body>, next: axum::middleware::Next| async move {
            let method = req.method().to_string().blue();
            let path = req.uri().path().cyan();
            let response = next.run(req).await;
            let status = response.status().as_u16();
            let status_colored = match status {
                200..=299 => status.to_string().green(),
                400..=499 => status.to_string().yellow(),
                500..=599 => status.to_string().red(),
                _ => status.to_string().normal(),
            };
            println!("{} {} {}", method, path, status_colored);
            response
        }));
    app
}

pub async fn build_app() -> Result<Router, sqlx::Error> {
    dotenv::dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await?;
    Ok(build_app_with_pool(pool).await)
}

pub async fn run_server(app: Router, addr: &str) -> std::io::Result<()> {
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
}
