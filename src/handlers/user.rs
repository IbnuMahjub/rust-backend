use crate::db::get_conn;
use crate::handlers::auth::generate_token;
use crate::middleware::auth_guard::AuthenticatedUser;
use crate::models::user::{LoginRequest, NewUser, User};
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordVerifier, SaltString};
use argon2::{Argon2, PasswordHasher};
use axum::{extract::Json as AxumJson, response::IntoResponse, Json};
use mysql::{params, prelude::*};

#[derive(serde::Serialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub message: String,
    pub data: Option<T>,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: User,
}

pub async fn get_users() -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let result: Result<Vec<User>, _> = conn
                .query_map("SELECT id, name, email FROM users", |(id, name, email)| {
                    User { id, name, email }
                });

            match result {
                Ok(users) => Json(ApiResponse {
                    status: "success".into(),
                    message: "Daftar pengguna berhasil diambil.".into(),
                    data: Some(users),
                }),
                Err(e) => Json(ApiResponse::<Vec<User>> {
                    status: "error".into(),
                    message: format!("Gagal mengambil data: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => Json(ApiResponse::<Vec<User>> {
            status: "error".into(),
            message: format!("Gagal koneksi ke database: {}", e),
            data: None,
        }),
    }
}

pub async fn create_user(AxumJson(payload): AxumJson<NewUser>) -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let salt = SaltString::generate(&mut OsRng);
            let argon2 = Argon2::default();

            let hashed_password = match argon2.hash_password(payload.password.as_bytes(), &salt) {
                Ok(hash) => hash.to_string(),
                Err(e) => {
                    return Json(ApiResponse::<()> {
                        status: "error".into(),
                        message: format!("Gagal mengenkripsi password: {}", e),
                        data: None,
                    });
                }
            };

            let result = conn.exec_drop(
                "INSERT INTO users (name, email, password) VALUES (:name, :email, :password)",
                params! {
                    "name" => &payload.name,
                    "email" => &payload.email,
                    "password" => &hashed_password,
                },
            );

            match result {
                Ok(_) => Json(ApiResponse::<()> {
                    status: "success".into(),
                    message: "Pengguna berhasil didaftarkan.".into(),
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
            message: format!("Gagal koneksi ke database: {}", e),
            data: None,
        }),
    }
}

pub async fn login_user(AxumJson(payload): AxumJson<LoginRequest>) -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let user_record: Result<Option<(u64, String, String, String)>, _> = conn.exec_first(
                "SELECT id, name, email, password FROM users WHERE email = :email",
                params! { "email" => &payload.email },
            );

            match user_record {
                Ok(Some((id, name, email, hashed_password))) => {
                    let parsed_hash = PasswordHash::new(&hashed_password).unwrap();
                    let argon2 = Argon2::default();

                    if argon2
                        .verify_password(payload.password.as_bytes(), &parsed_hash)
                        .is_ok()
                    {
                        let user = User { id, name, email };
                        let token = generate_token(id as i32).unwrap();

                        Json(ApiResponse::<LoginResponse> {
                            status: "success".into(),
                            message: "Login berhasil.".into(),
                            data: Some(LoginResponse { token, user }),
                        })
                    } else {
                        Json(ApiResponse::<LoginResponse> {
                            status: "error".into(),
                            message: "Email atau password salah.".into(),
                            data: None,
                        })
                    }
                }
                Ok(None) => Json(ApiResponse::<LoginResponse> {
                    status: "error".into(),
                    message: "Email tidak ditemukan.".into(),
                    data: None,
                }),
                Err(e) => Json(ApiResponse::<LoginResponse> {
                    status: "error".into(),
                    message: format!("Gagal menjalankan query: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => Json(ApiResponse::<LoginResponse> {
            status: "error".into(),
            message: format!("Gagal koneksi ke database: {}", e),
            data: None,
        }),
    }
}

pub async fn me(AuthenticatedUser(claims): AuthenticatedUser) -> impl IntoResponse {
    match get_conn() {
        Ok(mut conn) => {
            let user = conn.exec_first::<(u64, String, String), _, _>(
                "SELECT id, name, email FROM users WHERE id = :id",
                params! {
                    "id" => claims.sub,
                },
            );

            match user {
                Ok(result) => {
                    let data = result.map(|(id, name, email)| User { id, name, email });
                    Json(ApiResponse {
                        status: if data.is_some() { "success" } else { "error" }.into(),
                        message: if data.is_some() {
                            "Data user berhasil diambil.".into()
                        } else {
                            "User tidak ditemukan.".into()
                        },
                        data,
                    })
                }
                Err(e) => Json(ApiResponse {
                    status: "error".into(),
                    message: format!("Gagal menjalankan query: {}", e),
                    data: None,
                }),
            }
        }
        Err(e) => Json(ApiResponse {
            status: "error".into(),
            message: format!("Gagal koneksi ke database: {}", e),
            data: None,
        }),
    }
}
