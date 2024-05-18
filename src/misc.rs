use std::{future::Future, ops::Deref, rc::Rc, time::Duration};

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_rw_signal, leptos_dom::logging::console_log, view, ChildrenFn,
    IntoView, RwSignal, Show, SignalGet, SignalGetUntracked, SignalUpdate, ViewFn,
};
use leptos_router::{use_navigate, Route};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    api::{self, api_boundary::ResultHandler, AuthorizedApi, OtpAuthorizedApi, UnauthorizedApi},
    consts::{API_TOKEN_OTP_KEY, API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    pages::login::Login,
};

#[derive(Clone)]
pub struct Requirement {
    pub func: Rc<dyn Fn(&String) -> bool>,
    pub fail_msg: String,
}

#[derive(Clone, Default)]
pub struct RegisterFormField {
    pub name: String,
    pub requirement: Option<Requirement>,
    pub is_password: bool,
    pub signal: RwSignal<String>,
}

#[derive(Clone, Copy)]
pub struct ApiSignals {
    pub auth: RwSignal<Option<AuthorizedApi>>,
    pub otpauth: RwSignal<Option<OtpAuthorizedApi>>,
    pub unauth: RwSignal<Option<UnauthorizedApi>>,
    pub is_resolved: RwSignal<bool>,
}

impl ApiSignals {
    pub fn new() -> Self {
        let unauth = create_rw_signal(None);
        let otpauth = create_rw_signal(None);
        let auth = create_rw_signal(None);
        let is_resolved = create_rw_signal(false);

        ApiSignals {
            auth,
            otpauth,
            unauth,
            is_resolved,
        }
    }
}
pub fn go_to_page(page: crate::pages::Page) {
    let navigate = use_navigate();
    navigate(page.path(), Default::default());
}

pub async fn check_user_logged_in(api_set_signals: ApiSignals) {
    task::sleep(Duration::from_secs(2)).await;
    if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY.clone()) {
        let api = api::AuthorizedApi::new(&DEFAULT_API_URL, token);
        match api.has_expired().await {
            ResultHandler::OkResult(res) => {
                if res {
                    info!("login session token has expired");
                } else {
                    api_set_signals
                        .auth
                        .update(|api_curr| *api_curr = Some(api));
                    go_to_page(crate::pages::Page::Home);
                }
            }
            ResultHandler::ErrResult(err_message) => {
                info!("{}", err_message);
            }
        }
    } else if let Ok(token) = LocalStorage::get(API_TOKEN_OTP_KEY.clone()) {
        let api = api::OtpAuthorizedApi::new(&DEFAULT_API_URL, token);
        match api.has_expired().await {
            ResultHandler::OkResult(res) => {
                if res {
                    info!("Enter OTP session token has expired");
                } else {
                    api_set_signals
                        .otpauth
                        .update(|api_curr| *api_curr = Some(api));
                    go_to_page(crate::pages::Page::OtpValidation);
                    return;
                }
            }
            ResultHandler::ErrResult(err_message) => {
                info!("{}", err_message);
            }
        }
    } else {
        let api = api::UnauthorizedApi::new(&DEFAULT_API_URL);
        api_set_signals
            .unauth
            .update(|api_curr| *api_curr = Some(api));
    }
    api_set_signals.is_resolved.update(|x| *x = true);
}

pub enum ApiStateView<F>
where
    F: IntoView + 'static,
{
    UnAuth(Rc<dyn Fn(UnauthorizedApi) -> F>),
    OTPAuth(
        Rc<dyn Fn(UnauthorizedApi) -> F>,
        Rc<dyn Fn(OtpAuthorizedApi) -> F>,
    ),
    Auth(
        Rc<dyn Fn(UnauthorizedApi) -> F>,
        Rc<dyn Fn(AuthorizedApi) -> F>,
    ),
}

#[component]
pub fn ApiStateCheckView<F>(view: ApiStateView<F>) -> impl IntoView
where
    F: IntoView + 'static + Clone,
{
    let api_signals = ApiSignals::new();
    let check_logged_in =
        create_action(|api_set_signals: &ApiSignals| check_user_logged_in(api_set_signals.clone()));

    check_logged_in.dispatch(api_signals);

    let view = match view {
        ApiStateView::UnAuth(view_func_unauth) => {
            if let Some(unauth_api) = api_signals.unauth.get_untracked() {
                view_func_unauth(unauth_api)
            } else {
                let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                view_func_unauth(unauth_api)
            }
        }
        ApiStateView::OTPAuth(view_func_unauth, view_func_otp_auth) => {
            if let Some(otpAuth) = api_signals.otpauth.get_untracked() {
                view_func_otp_auth(otpAuth)
            } else if let Some(unauth_api) = api_signals.unauth.get_untracked() {
                view_func_unauth(unauth_api)
            } else {
                let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                view_func_unauth(unauth_api)
            }
        }
        ApiStateView::Auth(view_func_unauth, view_func_auth) => {
            if let Some(authApi) = api_signals.auth.get_untracked() {
                view_func_auth(authApi)
            } else if let Some(unauth_api) = api_signals.unauth.get_untracked() {
                view_func_unauth(unauth_api)
            } else {
                let unauth_api = UnauthorizedApi::new(&DEFAULT_API_URL);
                view_func_unauth(unauth_api)
            }
        }
    };

    view! {
        <Show when = move || api_signals.is_resolved.get()
                fallback = move || {
                    console_log("fallback");
                    view! { <div class="color:white;">{"Loading..."}</div>}
                }
        >
            {view.clone()}
        </Show>
    }
}
