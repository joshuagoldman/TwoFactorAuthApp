use std::{collections::HashMap, rc::Rc, time::Duration};

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_rw_signal, leptos_dom::logging::console_log, view, IntoView,
    RwSignal, Show, Signal, SignalGet, SignalUpdate,
};
use leptos_router::use_navigate;
use log::info;

use crate::{
    api::{
        api_boundary::{ApiToken, ResultHandler},
        authorized_api::AuthorizedApi,
        otp_authorized_api::OtpAuthorizedApi,
        unauthorized_api::UnauthorizedApi,
    },
    consts::{API_TOKEN_OTP_KEY, API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    pages::Page,
};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ApiState {
    #[default]
    UnAuth,
    Otp,
    Auth,
}

impl ApiState {
    fn to_string(&self) -> String {
        match self {
            ApiState::UnAuth => "UnAuth".to_string(),
            ApiState::Otp => "Otp".to_string(),
            ApiState::Auth => "Auth".to_string(),
        }
    }
    pub fn compare(&self, compare_val: Self) -> bool {
        self.to_string() == compare_val.to_string()
    }
}

#[derive(Clone)]
pub struct Requirement {
    pub func: Rc<dyn Fn(&String) -> bool>,
    pub fail_msg: String,
}

#[derive(Clone, Default)]
pub struct GeneralFormField {
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

pub fn log_out() {
    LocalStorage::delete(API_TOKEN_OTP_KEY.clone());
    LocalStorage::delete(API_TOKEN_STORAGE_KEY.clone());
    let navigate = use_navigate();
    navigate(Page::Login.path(), Default::default());
}

pub fn get_api_state_by_page_map() -> HashMap<Page, ApiState> {
    let mut api_state_page_map: HashMap<Page, ApiState> = HashMap::new();

    let unauth_pages = vec![Page::Login, Page::Register];
    let otp_pages = vec![Page::OtpValidation];
    let auth_pages = vec![Page::Home, Page::Reset, Page::Delete];

    for (_, page) in unauth_pages.iter().enumerate() {
        api_state_page_map.insert(page.clone(), ApiState::UnAuth);
    }
    for (_, page) in otp_pages.iter().enumerate() {
        api_state_page_map.insert(page.clone(), ApiState::Otp);
    }
    for (_, page) in auth_pages.iter().enumerate() {
        api_state_page_map.insert(page.clone(), ApiState::Auth);
    }

    api_state_page_map
}

pub fn is_move_to_default_page(page: Page, api_state: ApiState) -> Option<Page> {
    let api_state_page_map = get_api_state_by_page_map();

    let default_page = match api_state {
        ApiState::UnAuth => Page::Login,
        ApiState::Otp => Page::OtpValidation,
        ApiState::Auth => Page::Home,
    };

    if let Some(page_actual_api_state) = api_state_page_map.get(&page) {
        if page_actual_api_state.compare(api_state) {
            None
        } else {
            Some(default_page)
        }
    } else {
        Some(default_page)
    }
}

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

#[derive(Clone)]
pub struct ApiStateViewInfo<F>
where
    F: IntoView + 'static + Clone,
{
    pub page: Page,
    pub view: ApiStateView<F>,
}

#[derive(Clone)]
pub enum ApiStateView<F>
where
    F: IntoView + 'static + Clone,
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
