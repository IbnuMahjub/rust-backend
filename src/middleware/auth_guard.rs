use crate::handlers::auth::{verify_token, Claims};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

pub struct AuthenticatedUser(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String); // ✅ Tambahkan ini

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        if let Some(header_value) = auth_header {
            if header_value.starts_with("Bearer ") {
                let token = header_value.trim_start_matches("Bearer ").trim();

                match verify_token(token) {
                    Ok(claims) => Ok(AuthenticatedUser(claims)),
                    Err(_) => Err((StatusCode::UNAUTHORIZED, "Token tidak valid".to_string())),
                }
            } else {
                Err((StatusCode::UNAUTHORIZED, "Format token salah".to_string()))
            }
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                "Token tidak ditemukan".to_string(),
            ))
        }
    }
}
