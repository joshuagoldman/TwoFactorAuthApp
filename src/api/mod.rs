use gloo_net::http::{Request, RequestBuilder, Response};
use serde::de::DeserializeOwned;
pub mod api_boundary;
pub mod authorized_api;
pub mod otp_authorized_api;
pub mod unauthorized_api;

use api_boundary::*;

pub async fn into_json<T>(response: Response) -> ResultHandler<T>
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
