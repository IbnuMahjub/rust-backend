use axum::{extract::Json as AxumJson, response::IntoResponse, Json};
use mysql::params;
use mysql::prelude::*;

use crate::db::get_conn;
use crate::models::user::{NewUser, User};

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

pub async fn get_users() -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let users: Vec<User> = conn
                .query_map("SELECT id, name, email FROM users", |(id, name, email)| {
                    User { id, name, email }
                })
                .unwrap_or_default();

            Json(ApiResponse {
                status: "success".into(),
                message: "Daftar pengguna berhasil diambil.".into(),
                data: Some(users),
            })
        }
        Err(e) => Json(ApiResponse::<Vec<User>> {
            status: "error".into(),
            message: format!("Gagal koneksi DB: {}", e),
            data: None,
        }),
    }
}

pub async fn create_user(AxumJson(payload): AxumJson<NewUser>) -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let result = conn.exec_drop(
                "INSERT INTO users (name, email) VALUES (:name, :email)",
                params! {
                    "name" => &payload.name,
                    "email" => &payload.email,
                },
            );

            match result {
                Ok(_) => Json(ApiResponse::<()> {
                    status: "success".into(),
                    message: "Pengguna berhasil ditambahkan.".into(),
                    data: None,
                }),
                Err(e) => Json(ApiResponse::<()> {
                    status: "error".into(),
                    message: format!("Gagal menambahkan pengguna: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => Json(ApiResponse::<()> {
            status: "error".into(),
            message: format!("Gagal koneksi DB: {}", e),
            data: None,
        }),
    }
}
