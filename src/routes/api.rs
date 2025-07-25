use crate::handlers::user::{create_user, get_users, login_user, me};
use crate::middleware::auth_guard::AuthenticatedUser;
use axum::{
    routing::{get, post},
    Router,
};

pub fn api_routes() -> Router {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/login", post(login_user))
        .route("/me", get(me))
}
