use axum::{
    body::Body, extract::FromRequestParts, http::Request, http::StatusCode, http::request::Parts,
    middleware::Next, response::Response,
};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};

use crate::models::user::Claims;
use jsonwebtoken::{DecodingKey, Validation, decode};
use std::sync::Arc;

#[allow(dead_code)] // Suppress the warning about unused fields
#[derive(Clone)]
pub struct AuthContext {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}

// Add an extractor to make the AuthContext usable in route handlers
impl<S> FromRequestParts<S> for AuthContext
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    // native async fn now matches the trait exactly
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_ctx = parts
            .extensions
            .get::<Arc<AuthContext>>()
            .ok_or(StatusCode::UNAUTHORIZED)?;

        Ok(auth_ctx.as_ref().clone())
    }
}

pub async fn auth_middleware(
    bearer: Option<TypedHeader<Authorization<Bearer>>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Get JWT from Authorization header
    let bearer = bearer.ok_or(StatusCode::UNAUTHORIZED)?;
    let token = bearer.token();

    // Validate JWT
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let claims = token_data.claims;

    println!("User ID: {}", claims.sub);
    println!("Username: {}", claims.username);
    println!("Token: {}", token);
    
    // Add authenticated user info to request extensions
    let auth_ctx = AuthContext {
        user_id: claims.sub,
        username: claims.username,
        role: claims.role,
    };
    request.extensions_mut().insert(Arc::new(auth_ctx));

    // Continue with the request
    Ok(next.run(request).await)
}
