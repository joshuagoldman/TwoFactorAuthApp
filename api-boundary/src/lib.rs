use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub full_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
    pub full_name: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUserResponse {
    pub user: UserResponse,
    pub qr_code: String
}


#[derive(Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    pub token: String,
    pub otp: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub username: String,
    pub full_name: String
}

impl UserInfo {
    pub fn new() -> Self {
        Self {
            email: "".to_string(),
            username: "".to_string(),
            full_name: "".to_string()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiToken {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}
