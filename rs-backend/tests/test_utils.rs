use axum_test::TestServer;
use rs_backend::app::build_app_with_pool;
use sqlx::{PgPool, postgres::PgPoolOptions};
use serde_json::Value;
use argon2::PasswordHasher;

pub async fn test_db_pool() -> PgPool {
    dotenv::dotenv().ok(); // Ensure .env is loaded for tests
    let db_url = std::env::var("DATABASE_URL_TEST").unwrap_or_else(|_| std::env::var("DATABASE_URL").unwrap());
    PgPoolOptions::new()
        .max_connections(1)
        .connect(&db_url)
        .await
        .expect("Failed to connect to test database")
}

#[allow(dead_code)]
pub async fn create_test_user(pool: &PgPool, username: &str, password: &str, email: &str, role: &str) -> i32 {
    let salt = argon2::password_hash::SaltString::generate(&mut argon2::password_hash::rand_core::OsRng);
    let argon2 = argon2::Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string();

    let row = sqlx::query!(
        "INSERT INTO users (username, password_hash, email, role) VALUES ($1, $2, $3, $4) RETURNING id",
        username,
        password_hash, // Store Argon2 hashed password
        email,
        role
    )
    .fetch_one(pool)
    .await
    .expect("Failed to insert test user");
    row.id
}

#[allow(dead_code)]
pub async fn login_and_get_jwt(server: &axum_test::TestServer, username: &str, password: &str) -> String {
    let body = serde_json::json!({"username": username, "password": password});
    let response = server.post("/login").json(&body).await;
    assert_eq!(response.status_code(), 200);
    response.json::<Value>().as_str().unwrap().to_string()
}

#[allow(dead_code)]
pub async fn register_test_user(server: &axum_test::TestServer, username: &str, password: &str, email: &str, role: &str) {
    let body = serde_json::json!({
        "username": username,
        "password": password,
        "email": email,
        "role": role
    });
    let response = server.post("/register").json(&body).await;
    assert_eq!(response.status_code(), 201);
}

#[allow(dead_code)]
pub async fn create_project_and_feedback(pool: &PgPool, username: &str) -> (i32, i32, i32) {
    let user_id = sqlx::query!("SELECT id FROM users WHERE username = $1", username)
        .fetch_one(pool).await.unwrap().id;
    let project_id = sqlx::query!("INSERT INTO projects (name, description, owner_id) VALUES ($1, $2, $3) RETURNING id", "Comment Project", "desc", user_id)
        .fetch_one(pool).await.unwrap().id;
    let feedback_id = sqlx::query!("INSERT INTO feedback (title, description, status, priority, creator_id, project_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id", "Feedback for Comment", "desc", "open", "medium", user_id, project_id)
        .fetch_one(pool).await.unwrap().id;
    (user_id, project_id, feedback_id)
}

#[allow(dead_code)]
pub async fn setup_test_environment() -> (TestServer, String, i32, i32, i32) {
    let pool = test_db_pool().await;
    truncate_tables(&pool).await;
    let server = TestServer::new(build_app_with_pool(pool.clone()).await).unwrap();
    register_test_user(&server, "username", "password", "email@email.com", "student").await;
    let jwt: String = login_and_get_jwt(&server, "username", "password").await;
    let (user_id, project_id, feedback_id) = create_project_and_feedback(&pool, "username").await;
    (server, jwt, user_id, project_id, feedback_id)
}

pub async fn truncate_tables(pool: &PgPool) {
    // Disable and re-enable referential integrity for truncation
    sqlx::query!("SET session_replication_role = 'replica';").execute(pool).await.unwrap();
    sqlx::query!("TRUNCATE TABLE feedback_labels, project_enrollments, comments, feedback, labels, projects, users RESTART IDENTITY CASCADE;")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query!("SET session_replication_role = 'origin';").execute(pool).await.unwrap();
}