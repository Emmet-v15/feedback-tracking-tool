use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, Router, extract::State, http::StatusCode, routing::post};
use jsonwebtoken::{EncodingKey, Header, encode};
use sqlx::PgPool;

use crate::models::user::{Claims, LoginRequest, RegisterRequest, User};

pub fn routes() -> Router<PgPool> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "Created"),
        (status = 409, description = "Conflict"),
        (status = 400, description = "Bad Request"),
    )
)]
pub async fn register(
    State(pool): State<PgPool>,
    Json(request): Json<RegisterRequest>,
) -> Result<StatusCode, StatusCode> {
    // Validate role
    let allowed_roles = ["admin", "teacher", "student"];
    if !allowed_roles.contains(&request.role.as_str()) {
        println!("Registration error: invalid role '{}'. Allowed: admin, teacher, student", request.role);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check for duplicate username
    let existing = sqlx::query!("SELECT 1 as exists FROM users WHERE username = $1", request.username)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if existing.is_some() {
        println!("Registration error: duplicate username '{}'.", request.username);
        return Err(StatusCode::CONFLICT);
    }

    // Generate password hash
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();

    // Insert new user
    let result = sqlx::query!(
        "INSERT INTO users (username, password_hash, email, role) VALUES ($1, $2, $3, $4)",
        request.username,
        password_hash,
        request.email,
        request.role
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => {
            println!("Registration error: {}", e);
            if e.to_string().contains("duplicate") {
                return Err(StatusCode::CONFLICT);
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "OK"),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn login(
    State(pool): State<PgPool>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<String>, StatusCode> {
    // Find user
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1",
        request.username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::UNAUTHORIZED)?;

    // Verify password
    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Argon2::default()
        .verify_password(request.password.as_bytes(), &parsed_hash)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Generate token
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let exp = chrono::Utc::now() + chrono::Duration::hours(1); // Token expiration time

    let claims = Claims {
        sub: user.id.expect("User id must be set after creation"),
        exp: exp.timestamp() as usize,
        username: user.username,
        role: user.role,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(token))
}
