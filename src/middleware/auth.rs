use axum::{
    body::Body,
    extract::FromRequest,
    http::{self, HeaderMap, Request},
};

use crate::error::{AppError, Result};
use crate::utils::jwt::{self, Claims};

pub struct AuthUser(pub Claims);

impl AuthUser {
    pub fn cliams(&self) -> &Claims {
        &self.0
    }
}

fn extract_bearer(header: &HeaderMap) -> Result<&str> {
    header
        .get(http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing auth header".into()))
        .and_then(|s| {
            s.strip_prefix("Bearer ")
                .ok_or_else(|| AppError::Unauthorized("Invalid auth header".into()))
        })
}


#[axum::async_trait]
impl<S> FromRequest<S> for AuthUser 
where S: Send + Sync
{
    type Rejection = AppError;

    async fn from_request(req: Request<Body>, _state: &S) -> std::result::Result<Self, Self::Rejection> {
        let header = req.headers();
        let token = extract_bearer(header)?;
        let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".into());   
        let claims = jwt::verify_token(token, &secret)?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized("Not a valid access token".into()));
        }
        Ok(AuthUser(claims))
    }
}
