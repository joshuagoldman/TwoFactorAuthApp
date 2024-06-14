use gloo_net::http::{Request, RequestBuilder};
use serde::de::DeserializeOwned;

use super::{
    authorized_api::AuthorizedApi, check_connection, into_json, ApiToken, ResultHandler,
    TokenResponse,
};

#[derive(Clone, Debug)]
pub struct OtpAuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

impl OtpAuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }

    pub async fn check_otp(&self, otp: &String) -> ResultHandler<AuthorizedApi> {
        let url = format!("{}/verifyotp/{}", self.url, otp);
        self.send::<TokenResponse>(Request::delete(&url))
            .await
            .pipe_result_action(|token_resp| {
                std::result::Result::Ok(AuthorizedApi::new(
                    self.url,
                    ApiToken {
                        token: token_resp.token,
                    },
                )) as std::result::Result<AuthorizedApi, String>
            })
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
    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }

    pub async fn has_expired(&self) -> ResultHandler<bool> {
        let url = format!("{}/expired", self.url);
        self.send(Request::delete(&url)).await
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}
