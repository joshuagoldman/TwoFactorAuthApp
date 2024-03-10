use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_signal, view, IntoView, Show, SignalGet, SignalUpdate,
};

use crate::{
    api::{self, api_boundary::Credentials, UnauthorizedApi},
    components::login_form::*,
    consts::API_TOKEN_OTP_KEY,
    misc,
};

use super::Page;

#[component]
pub fn Login(unauth_api: UnauthorizedApi) -> impl IntoView {
    let (login_error, login_error_set) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let login_action = create_action(move |login_data: &Credentials| {
        let login_data = login_data.clone();
        async move {
            set_wait_for_response.update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;
            match unauth_api.login(&login_data).await {
                api::api_boundary::ResultHandler::OkResult(token_resp) => {
                    //                   let auth_api = AuthorizedApi::new(
                    //                       &DEFAULT_API_URL,
                    //                       ApiToken {
                    //                           token: token_resp.clone().token,
                    //                       },
                    //                   );
                    LocalStorage::set(API_TOKEN_OTP_KEY.clone(), token_resp.token.clone());
                    misc::go_to_page(Page::OtpValidation)
                }
                api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                    login_error_set.update(|x| *x = Some(err_msg));
                }
            }
            set_wait_for_response.update(|upd: &mut bool| *upd = false);
        }
    });

    view! {
        <Show
            when=move || wait_for_response.get()
            fallback = || view! { <div style="color:red">{"Loading"}</div>}
        >
            <LoginForm login_action
                       login_error
       />
        </Show>
    }
}
