use gloo_net::http::{Request, RequestBuilder};

use super::{
    check_connection, into_json, CreateNewUserResponse, Credentials, NewUser, ResultHandler,
    TokenResponse,
};

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn login(&self, credentials: &Credentials) -> ResultHandler<TokenResponse> {
        let url = format!("{}/login", self.url);
        self.check_connection(Request::post(&url))
            .await
            .pipe_result_action(|ok_req| ok_req.json(credentials))
            .pipe_result_action_async(|ok_req| ok_req.send())
            .await
            .pipe_action_async(|response| into_json(response))
            .await
    }

    pub async fn register(&self, new_user: &NewUser) -> ResultHandler<CreateNewUserResponse> {
        let url = format!("{}/create", self.url);
        self.check_connection(Request::post(&url))
            .await
            .pipe_result_action(|ok_req| ok_req.json(new_user))
            .pipe_result_action_async(|ok_req| ok_req.send())
            .await
            .pipe_action_async(|response| into_json(response))
            .await
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}
