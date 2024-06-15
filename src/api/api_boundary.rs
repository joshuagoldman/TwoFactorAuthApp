use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Credentials {
    pub password: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub full_name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NewUserResponse {
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateNewUserResponse {
    pub user: NewUserResponse,
    pub qr_code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub full_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileInfo {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TokenResponse {
    pub token: String,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResetPasswordRequest {
    pub new_password: String,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordRequest {
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtpRequest {
    pub otp: TokenResponse,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CheckConnectionResponse {
    pub str_resp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub str_resp: String,
}
