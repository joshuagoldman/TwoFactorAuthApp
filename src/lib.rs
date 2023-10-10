use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

use api_boundary::*;

mod api;
mod components;
mod pages;

use crate::api::AuthorizedApi;
use lazy_static::lazy_static;

use self::{components::*, pages::*};

lazy_static! {
    static ref DEFAULT_API_URL: String =
        std::env::var("DEFAULT_API_URL").expect("DEFAULT_API_URL is expected!");
}

lazy_static! {
    static ref API_TOKEN_STORAGE_KEY: String =
        std::env::var("API_TOKEN_STORAGE_KEY").expect("API_TOKEN_STORAGE_KEY is expected!");
}

lazy_static! {
    static ref API_TOKEN_OTP_KEY: String =
        std::env::var("API_TOKEN_OTP_KEY").expect("API_TOKEN_OTP_KEY is expected!");
}

#[component]
pub fn App() -> impl IntoView {
    // -- signals -- //

    let authorized_api = create_rw_signal(None::<api::AuthorizedApi>);
    let otp_authorized_api = create_rw_signal(None::<api::OtpAuthorizedApi>);
    let token_has_been_verified = create_rw_signal(false);
    let user_info = create_rw_signal(None::<UserInfo>);
    let logged_in =
        Signal::derive(move || authorized_api.get().is_some() && user_info.get().is_some());

    // -- actions -- //

    let check_if_token_expired = move |api: api::AuthorizedApi| async move {
        match api.has_expired().await {
            Ok(res) => res,
            _ => true,
        }
    };

    let fetch_user_info = create_action(move |api: &AuthorizedApi| {
        let api = api.clone();
        async move {
            if !check_if_token_expired(api.clone()).await {
                authorized_api.update(|a| *a = Some(api.clone()));
                token_has_been_verified.update(|a| *a = true);
                async_std::task::sleep(std::time::Duration::new(1, 0)).await;
                match api.user_info().await {
                    Ok(info) => {
                        async_std::task::sleep(std::time::Duration::new(1, 0)).await;
                        LocalStorage::delete(API_TOKEN_OTP_KEY.as_str());
                        otp_authorized_api.update(|x| *x = None);
                        user_info.update(|i| *i = Some(info));
                    }
                    Err(err) => {
                        authorized_api.update(|a| *a = None);
                        token_has_been_verified.update(|a| *a = true);
                        LocalStorage::delete(API_TOKEN_STORAGE_KEY.as_str());
                        log::error!("Unable to fetch user info: {err}")
                    }
                }
            } else {
                authorized_api.update(|a| *a = None);
                token_has_been_verified.update(|a| *a = true);
                LocalStorage::delete(API_TOKEN_STORAGE_KEY.as_str());
                log::error!("Token has expired!")
            }
        }
    });

    let set_opt_token_into_local_storage = create_action(move |_| async move {
        match otp_authorized_api.get() {
            Some(api) => {
                LocalStorage::set(API_TOKEN_OTP_KEY.as_str(), api.token)
                    .expect("LocalStorage::set");
            }
            None => {
                log::error!(
                    "Unable to fetch otp validation info: validate credentials are first required!"
                )
            }
        }
    });

    // -- init API -- //

    let unauthorized_api = api::UnauthorizedApi::new(DEFAULT_API_URL.as_str());
    if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY.as_str()) {
        let api = api::AuthorizedApi::new(DEFAULT_API_URL.as_str(), token);
        fetch_user_info.dispatch(api);
    } else {
        token_has_been_verified.update(|a| *a = true);
    }

    if let Ok(token) = LocalStorage::get(API_TOKEN_OTP_KEY.as_str()) {
        let api = api::OtpAuthorizedApi::new(DEFAULT_API_URL.as_str(), token);
        otp_authorized_api.update(|a| *a = Some(api));
    }

    log::debug!("User is logged in: {}", logged_in.get());

    view! {
      <Router>
        <NavBar logged_in
                on_logout = move || {
                    let navigate = use_navigate();
                    navigate(Page::Login.path(), Default::default());
                    LocalStorage::delete(API_TOKEN_STORAGE_KEY.as_str());
                    authorized_api.update(|a| *a = None);
                    user_info.update(|a| *a = None);
                }/>
        <main>
          <Routes>
            <Route
              path=Page::Home.path()
              view=move || view! {
                <Home
                    user_info = user_info.into()
                    api_signal= authorized_api
                    token_has_been_verified = token_has_been_verified
                    log_out_action = move || {
                      let navigate = use_navigate();
                      navigate(Page::Login.path(), Default::default());                    }
                />
              }
            />
            <Route
              path=Page::Login.path()
              view=move || view! {
                <Login
                  api = unauthorized_api
                  on_success = move |api: api::OtpAuthorizedApi| {
                      log::info!("Login first step succceeded!");
                      otp_authorized_api.update(|x| *x = Some(api.clone()));
                      set_opt_token_into_local_storage.dispatch(());
                      let navigate = use_navigate();
                      navigate(Page::OtpValidation.path(), Default::default());                  } />
              }
            />
            <Route
              path=Page::Register.path()
              view=move || view! {
                <Register api = unauthorized_api />
              }
            />
            <Route
              path=Page::OtpValidation.path()
              view=move || view! {
                <Otp
                    otp_api_signal=otp_authorized_api
                    opt_verify_success_action=move |api: api::AuthorizedApi| {
                        log::info!("Authentication successfully performed!");
                        authorized_api.update(|x| *x = Some(api.clone()));
                        LocalStorage::set(API_TOKEN_STORAGE_KEY.as_str(), api.clone().token).expect("LocalStorage::set");
                        let navigate = use_navigate();
                        navigate(Page::Home.path(), Default::default());
                        fetch_user_info.dispatch(api);
                    }
                />
              }
            />
          </Routes>
        </main>
      </Router>
    }
}
