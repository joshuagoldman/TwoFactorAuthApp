use gloo_net::http::{Request, RequestBuilder, Response};
use qrcode::{EcLevel, QrCode, Version};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub mod api_boundary;

use api_boundary::*;
use uuid::Uuid;

use crate::consts::DEFAULT_API_URL;

#[derive(Clone, Copy)]
pub struct UnauthorizedApi {
    url: &'static str,
}

#[derive(Clone, Debug)]
pub struct OtpAuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct AuthorizedApi {
    url: &'static str,
    pub token: ApiToken,
}

impl UnauthorizedApi {
    pub const fn new(url: &'static str) -> Self {
        Self { url }
    }

    pub async fn login(&self, credentials: &Credentials) -> ResultHandler<TokenResponse> {
        //       let url = format!("{}/login", self.url);
        //       self.check_connection(Request::post(&url))
        //           .await .pipe_result_action(|ok_req| ok_req.json(credentials))
        //           .pipe_result_action_async(|ok_req| ok_req.send())
        //           .await
        //           .pipe_action_async(|response| into_json(response))
        //           .await

        if credentials.username == "joshua".to_string()
            && credentials.password == "joshua".to_string()
        {
            ResultHandler::OkResult(TokenResponse {
                token: Uuid::new_v4().to_string(),
            })
        } else {
            ResultHandler::ErrResult("Wrong user name or password".to_string())
        }
    }

    pub async fn register(&self, new_user: &NewUser) -> ResultHandler<NewUserResponse> {
        //let url = format!("{}/register", self.url);
        //self.check_connection(Request::post(&url))
        //           .await
        //           .pipe_result_action(|ok_req| ok_req.json(credentials))
        //           .pipe_result_action_async(|ok_req| ok_req.send())
        //           .await
        //           .pipe_action_async(|response| into_json(response))
        //           .await

        // Create a QR code
        let qr_code = QrCode::with_version(
            "https://www.youtube.com/watch?v=qX-2csu0AqE",
            Version::Normal(3),
            EcLevel::L,
        )
        .expect("Failed to generate QR code");

        // Convert the QR code to a string representation
        let qr_code_string = qr_code.render().light_color(' ').dark_color('#').build();

        ResultHandler::OkResult(NewUserResponse {
            first_name: new_user.first_name.clone(),
            last_name: new_user.last_name.clone(),
            qr_code: qr_code_string,
        })
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
        check_connection(&self.url, req).await
    }
}

impl OtpAuthorizedApi {
    pub const fn new(url: &'static str, token: ApiToken) -> Self {
        Self { url, token }
    }

    pub async fn check_otp(&self, otp: &String) -> ResultHandler<AuthorizedApi> {
        //       let url = format!("{}/verifyotp/{}", self.url, otp);
        //       self.send(Request::get(&url))
        //           .await
        //           .pipe(|otp_resp: OtpSuccessResult| {
        //               ResultHandler::OkResult(AuthorizedApi::new(
        //                   self.url,
        //                   ApiToken {
        //                       token: otp_resp.token.token,
        //                   },
        //               ))
        //           });

        if otp == &"123456".to_string() {
            let auth_api = AuthorizedApi::new(
                &DEFAULT_API_URL,
                ApiToken {
                    token: Uuid::new_v4().to_string(),
                },
            );
            ResultHandler::OkResult(auth_api)
        } else {
            ResultHandler::ErrResult("Wrong OTP entered".to_string())
        }
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
        //let url = format!("{}/otpExpired", self.url);
        //let res = self.send(Request::get(&url)).await;
        //res
        ResultHandler::OkResult(false)
    }

    pub async fn check_connection(&self, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
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
    pub async fn has_expired(&self) -> ResultHandler<bool> {
        //let url = format!("{}/expired", self.url);
        //let res = self.send(Request::get(&url)).await;
        //res
        ResultHandler::OkResult(false)
    }

    pub async fn get_user_data(&self) -> ResultHandler<ProfileInfo> {
        //let url = format!("{}/profileInfo", self.url);
        //let res = self.send(Request::get(&url)).await;
        //res

        ResultHandler::OkResult(ProfileInfo {
            name: "Swag Johnson".to_string(),
            id: "ss".to_string(),
        })
    }

    pub async fn reset_password(
        &self,
        password: &String,
        new_password: &String,
    ) -> ResultHandler<bool> {
        //       let url = format!("{}/login", self.url);
        //       self.check_connection(Request::post(&url))
        //           .await .pipe_result_action(|ok_req| ok_req.json(credentials))
        //           .pipe_result_action_async(|ok_req| ok_req.send())
        //           .await
        //           .pipe_action_async(|response| into_json(response))
        //           .await

        let validate_password_req = ResetPasswordRequest {
            new_password: password.clone(),
            curr_password: new_password.clone(),
            token: self.token.token.clone(),
        };
        if password != &"password".to_string() {
            ResultHandler::OkResult(true)
        } else {
            ResultHandler::ErrResult("Reset of password failed".to_string())
        }
    }

    pub async fn validate_password(&self, password: &String) -> ResultHandler<bool> {
        //       let url = format!("{}/login", self.url);
        //       self.check_connection(Request::post(&url))
        //           .await .pipe_result_action(|ok_req| ok_req.json(credentials))
        //           .pipe_result_action_async(|ok_req| ok_req.send())
        //           .await
        //           .pipe_action_async(|response| into_json(response))
        //           .await

        if password == &"password".to_string() {
            ResultHandler::OkResult(true)
        } else {
            ResultHandler::ErrResult("Wrong password".to_string())
        }
    }

    pub async fn delete_account(&self) -> ResultHandler<bool> {
        //       let url = format!("{}/delete", self.url);
        // self.check_connection(Request::delete(&url))
        //           .await
        //           .pipe_result_action(|ok_req| ok_req.json(credentials))
        //           .pipe_result_action_async(|ok_req| ok_req.send())
        //           .await
        //           .pipe_action_async(|response| into_json(response))
        //           .await

        ResultHandler::OkResult(true)
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

async fn into_json<T>(response: Response) -> ResultHandler<T>
where
    T: DeserializeOwned,
{
    let res = response.text().await;
    to_result_handler(res).pipe_result_action(|json_str| serde_json::from_str(&json_str))
}

pub async fn check_connection(url: &str, req: RequestBuilder) -> ResultHandler<RequestBuilder> {
    to_result_handler(Request::get(format!("{}/", url).as_str()).send().await)
        .pipe_action_async(into_json)
        .await
        .pipe(|check_resp_obj: CheckConnectionResponse| {
            if check_resp_obj
                .str_resp
                .to_uppercase()
                .contains("WELCOME TO AUTH WEB API!")
            {
                ResultHandler::OkResult(req)
            } else {
                ResultHandler::ErrResult(format!(
                    "Error testing response res - unexpected result received: {}",
                    check_resp_obj.str_resp
                ))
            }
        })
}
