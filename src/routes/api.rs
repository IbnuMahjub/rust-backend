use crate::handlers::user::{create_user, get_users};
use axum::{
    routing::{get, post},
    Router,
};

pub fn api_routes() -> Router {
    Router::new().route("/users", get(get_users).post(create_user))
}
