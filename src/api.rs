use api_boundary::*;
use gloo_net::http::{Request, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use thiserror::Error;

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone, Debug)]
pub struct OtpAuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

#[derive(Clone)]
pub struct AuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }
    pub async fn register(&self, credentials: &NewUser) -> Result<NewUserResponse> {
        let url = format!("{}/create", self.url);
        let response = self
            .check_connection(Request::post(&url))
            .await?
            .json(credentials)?
            .send()
            .await?;
        into_json(response).await
    }
    pub async fn login(&self, credentials: &Credentials) -> Result<OtpAuthorizedApi> {
        let url = format!("{}/login", self.url);
        let response = self
            .check_connection(Request::post(&url))
            .await?
            .json(credentials)?
            .send()
            .await?;
        let login_resp: TokenResponse = into_json(response).await?;
        Ok(OtpAuthorizedApi::new(
            self.url,
            ApiToken {
                token: login_resp.token,
            },
        ))
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> Result<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}

impl OtpAuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }
    pub async fn check_otp(&self, otp: &String) -> Result<AuthorizedApi> {
        let url = format!("{}/verifyotp/{}", self.url, otp);
        let otp_resp: TokenResponse = self.send(Request::get(&url)).await?;
        Ok(AuthorizedApi::new(
            self.url,
            ApiToken {
                token: otp_resp.token,
            },
        ))
    }
    async fn send<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .check_connection(req)
            .await?
            .header("Authorization", &self.auth_header_value())
            .send()
            .await?;
        into_json(response).await
    }
    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> Result<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}

impl AuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }
    fn auth_header_value(&self) -> String {
        format!("Bearer {}", self.token.token)
    }
    async fn send<T>(&self, req: RequestBuilder) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .check_connection(req)
            .await?
            .header("Authorization", &self.auth_header_value())
            .send()
            .await?;
        into_json(response).await
    }
    pub async fn user_info(&self) -> Result<UserInfo> {
        let url = format!("{}/user", self.url);
        self.send(Request::get(&url)).await
    }
    pub async fn has_expired(&self) -> Result<bool> {
        let url = format!("{}/expired", self.url);
        self.send(Request::get(&url)).await
    }
    pub async fn check_connection(&self, req: RequestBuilder) -> Result<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Fetch(#[from] gloo_net::Error),
    #[error("{0:?}")]
    Api(api_boundary::Error),
}

impl From<api_boundary::Error> for Error {
    fn from(e: api_boundary::Error) -> Self {
        Self::Api(e)
    }
}

async fn into_json<T>(response: Response) -> Result<T>
where
    T: DeserializeOwned,
{
    // ensure we've got 2xx status
    if response.ok() {
        Ok(response.json().await?)
    } else {
        Err(response.json::<api_boundary::Error>().await?.into())
    }
}

pub async fn check_connection(url: &str, req: RequestBuilder) -> Result<RequestBuilder> {
    let resp = Request::get(format!("{}/", url).as_str()).send().await?;
    let res: CheckConnectionResponse = into_json(resp).await?;
    if res
        .str_resp
        .to_uppercase()
        .contains("WELCOME TO AUTH WEB API!")
    {
        Result::Ok(req)
    } else {
        Result::Err(Error::Api(api_boundary::Error {
            message: format!(
                "Error testing response res - unexpected result received: {}",
                res.str_resp
            ),
        }))
    }
}
