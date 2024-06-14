use std::{
    fmt::{Debug, Display},
    future::Future,
};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ResultHandler<A> {
    OkResult(A),
    ErrResult(String),
}

impl<A> ResultHandler<A> {
    pub fn pipe<B>(self, f: impl FnOnce(A) -> ResultHandler<B>) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(ok_res_a) => f(ok_res_a),
            ResultHandler::ErrResult(err_res) => ResultHandler::ErrResult(format!("{}", err_res)),
        }
    }

    pub fn pipe_action<B>(self, f: impl FnOnce() -> ResultHandler<B>) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(_) => f(),
            ResultHandler::ErrResult(err_res) => ResultHandler::ErrResult(format!("{}", err_res)),
        }
    }

    pub async fn pipe_action_async<B, Fut>(self, f: impl FnOnce(A) -> Fut) -> ResultHandler<B>
    where
        Fut: Future<Output = ResultHandler<B>>,
    {
        match self {
            ResultHandler::OkResult(ok_a_res) => match f(ok_a_res).await {
                ResultHandler::OkResult(ok_b_res) => ResultHandler::OkResult(ok_b_res),
                ResultHandler::ErrResult(b_res_err) => {
                    ResultHandler::ErrResult(format!("{:?>}", b_res_err))
                }
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }

    pub fn pipe_result_action<B, C: Debug + Display>(
        self,
        f: impl FnOnce(A) -> Result<B, C>,
    ) -> ResultHandler<B> {
        match self {
            ResultHandler::OkResult(ok_ares) => match f(ok_ares) {
                Ok(ok_bres) => ResultHandler::OkResult(ok_bres),
                Err(err_res) => ResultHandler::ErrResult(format!("{:?>}", err_res)),
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }

    pub async fn pipe_result_action_async<B, C: Debug + Display, Fut>(
        self,
        f: impl FnOnce(A) -> Fut,
    ) -> ResultHandler<B>
    where
        Fut: Future<Output = Result<B, C>>,
    {
        match self {
            ResultHandler::OkResult(ok_a_res) => match f(ok_a_res).await {
                Ok(ok_b_res) => ResultHandler::OkResult(ok_b_res),
                Err(b_res_error) => ResultHandler::ErrResult(format!("{:?>}", b_res_error)),
            },
            ResultHandler::ErrResult(err_res) => {
                ResultHandler::ErrResult(format!("{:?>}", err_res))
            }
        }
    }
}

pub fn to_result_handler<B, C: Debug + Display>(res: Result<B, C>) -> ResultHandler<B> {
    match res {
        Ok(ok_res) => ResultHandler::OkResult(ok_res),
        Err(str_err) => ResultHandler::ErrResult(format!("{:?>}", str_err)),
    }
}
