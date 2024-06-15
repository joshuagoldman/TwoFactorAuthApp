use gloo_net::http::{Request, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    check_connection, into_json, ApiToken, PasswordRequest, ProfileInfo, ResultHandler,
    UserResponse,
};

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

impl AuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }

    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }

    pub async fn has_expired(&self) -> ResultHandler<bool> {
        let url = format!("{}/expired", self.url);
        let res = self.send(Request::get(&url)).await;
        res
    }

    pub async fn get_user_data(&self) -> ResultHandler<ProfileInfo> {
        let url = format!("{}/user", self.url);
        let res = self.send(Request::get(&url)).await;
        res
    }

    pub async fn reset_password(&self, new_password: &String) -> ResultHandler<bool> {
        let url = format!("{}/changepass", self.url);
        self.check_connection(Request::post(&url))
            .await
            .pipe(|req_ok| {
                ResultHandler::OkResult(req_ok.header("Authorization", &self.auth_header_value()))
            })
            .pipe_result_action(|ok_req| {
                ok_req.json(&PasswordRequest {
                    password: new_password.clone(),
                })
            })
            .pipe_result_action_async(|ok_req| ok_req.send())
            .await
            .pipe_action_async(|response| into_json::<UserResponse>(response))
            .await
            .pipe_result_action(|_| {
                std::result::Result::Ok(true) as std::result::Result<bool, String>
            })
    }

    pub async fn validate_password(&self, password: &String) -> ResultHandler<bool> {
        let url = format!("{}/verifypass", self.url);
        self.check_connection(Request::post(&url))
            .await
            .pipe(|req_ok| {
                ResultHandler::OkResult(req_ok.header("Authorization", &self.auth_header_value()))
            })
            .pipe_result_action(|ok_req| {
                ok_req.json(&PasswordRequest {
                    password: password.clone(),
                })
            })
            .pipe_result_action_async(|ok_req| ok_req.send())
            .await
            .pipe_action_async(|response| into_json::<UserResponse>(response))
            .await
            .pipe_result_action(|_| {
                std::result::Result::Ok(true) as std::result::Result<bool, String>
            })
    }

    pub async fn delete_account(&self) -> ResultHandler<bool> {
        let res = self
            .send::<UserResponse>(Request::delete(&self.url))
            .await
            .pipe_result_action(|_| {
                std::result::Result::Ok(true) as std::result::Result<bool, String>
            });

        res
    }

    async fn send<T>(&self, req: RequestBuilder) -> ResultHandler<T>
    where
        T: DeserializeOwned,
    {
        self.check_connection(req)
            .await
            .pipe(|req_ok| {
                ResultHandler::OkResult(req_ok.header("Authorization", &self.auth_header_value()))
            })
            .pipe_result_action_async(|req_ok| req_ok.send())
            .await
            .pipe_action_async(|resp| into_json(resp))
            .await
    }
    pub async fn check_connection(&self, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}
