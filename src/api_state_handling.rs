use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, leptos_dom::logging::console_log, view, IntoView, Show, Signal,
    SignalGet, SignalUpdate,
};
use log::info;

use crate::{
    api::{
        api_boundary::ApiToken, authorized_api::AuthorizedApi,
        otp_authorized_api::OtpAuthorizedApi, resulthandler::ResultHandler,
        unauthorized_api::UnauthorizedApi,
    },
    consts::{API_TOKEN_OTP_KEY, API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    misc::{
        go_to_page, is_move_to_default_page, ApiSignals, ApiState, ApiStateView, ApiStateViewInfo,
    },
    pages::Page,
};

pub async fn check_user_logged_in(api_set_signals: ApiSignals, chosen_page: Page) {
    console_log(
        format!(
            "{:>?}",
            LocalStorage::get::<ApiToken>(API_TOKEN_OTP_KEY.clone())
        )
        .as_str(),
    );
    console_log(
        format!(
            "{:>?}",
            LocalStorage::get::<ApiToken>(API_TOKEN_STORAGE_KEY.clone())
        )
        .as_str(),
    );
    if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY.clone()) {
        let api = AuthorizedApi::new(&DEFAULT_API_URL, token);
        match api.has_expired().await {
            ResultHandler::OkResult(res) => {
                if res {
                    info!("login session token has expired");
                } else {
                    api_set_signals
                        .auth
                        .update(|api_curr| *api_curr = Some(api));
                    if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::Auth)
                    {
                        task::sleep(Duration::from_secs(2)).await;
                        go_to_page(defaut_page);
                        return;
                    }
                }
            }
            ResultHandler::ErrResult(err_message) => {
                info!("{}", err_message);
            }
        }
    } else if let Ok(token) = LocalStorage::get(API_TOKEN_OTP_KEY.clone()) {
        let api = OtpAuthorizedApi::new(&DEFAULT_API_URL, token);
        match api.has_expired().await {
            ResultHandler::OkResult(res) => {
                if res {
                    info!("Enter OTP session token has expired");
                } else {
                    api_set_signals
                        .otpauth
                        .update(|api_curr| *api_curr = Some(api));
                    if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::Otp) {
                        task::sleep(Duration::from_secs(2)).await;
                        go_to_page(defaut_page);
                        return;
                    }
                }
            }
            ResultHandler::ErrResult(err_message) => {
                info!("{}", err_message);
            }
        }
    } else {
        let api = UnauthorizedApi::new(&DEFAULT_API_URL);
        api_set_signals
            .unauth
            .update(|api_curr| *api_curr = Some(api));
        if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::UnAuth) {
            task::sleep(Duration::from_secs(2)).await;
            go_to_page(defaut_page);
            return;
        }
    }
    api_set_signals.is_resolved.update(|x| *x = true);
}

#[component]
pub fn ApiStateCheckView<F>(view_info: ApiStateViewInfo<F>) -> impl IntoView
where
    F: IntoView + 'static + Clone,
{
    let api_signals = ApiSignals::new();
    let check_logged_in = create_action(move |api_set_signals: &ApiSignals| {
        check_user_logged_in(api_set_signals.clone(), view_info.page)
    });

    check_logged_in.dispatch(api_signals);

    let view_signal = Signal::derive(move || {
        let view = match view_info.view.clone() {
            ApiStateView::UnAuth(view_func_unauth) => {
                if let Some(unauth_api) = api_signals.unauth.get() {
                    view_func_unauth(unauth_api)
                } else {
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    view_func_unauth(unauth_api)
                }
            }
            ApiStateView::OTPAuth(view_func_unauth, view_func_otp_auth) => {
                if let Some(otpAuth) = api_signals.otpauth.get() {
                    view_func_otp_auth(otpAuth)
                } else if let Some(unauth_api) = api_signals.unauth.get() {
                    go_to_page(crate::pages::Page::Login);
                    view_func_unauth(unauth_api)
                } else {
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    go_to_page(crate::pages::Page::Login);
                    view_func_unauth(unauth_api)
                }
            }
            ApiStateView::Auth(view_func_unauth, view_func_auth) => {
                if let Some(authApi) = api_signals.auth.get() {
                    view_func_auth(authApi)
                } else if let Some(unauth_api) = api_signals.unauth.get() {
                    view_func_unauth(unauth_api)
                } else {
                    let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                    view_func_unauth(unauth_api)
                }
            }
        };
        view
    });

    view! {
        <Show when = move || api_signals.is_resolved.get()
                fallback = move || {
                    view! { <div style="color:white;">{"Loading..."}</div>}
                }
        >
            {move || view_signal.get()}
        </Show>
    }
}
