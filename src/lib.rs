use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::*;

use api_boundary::*;

mod api;
mod components;
mod pages;

use crate::api::AuthorizedApi;

use self::{components::*, pages::*};

const DEFAULT_API_URL: &str = "http://localhost:3000";
const API_TOKEN_STORAGE_KEY: &str = "api-token";
const API_TOKEN_OTP_KEY: &str = "api-token-otp";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // -- signals -- //

    let authorized_api = create_rw_signal(cx, None::<api::AuthorizedApi>);
    let otp_authorized_api = create_rw_signal(cx, None::<api::OtpAuthorizedApi>);
    let token_has_been_verified = create_rw_signal(cx, false);
    let user_info = create_rw_signal(cx, None::<UserInfo>);
    let logged_in = Signal::derive(cx, move || authorized_api.get().is_some() && user_info.get().is_some());

    // -- actions -- //

    let check_if_token_expired = move |api: api::AuthorizedApi| async move {
        match api.has_expired().await {
          Ok(res) => res,
          _ => true
        }
    };

    let fetch_user_info = create_action(cx, move |api: &AuthorizedApi| {
      let api = api.clone();
      async move {
          if !check_if_token_expired(api.clone()).await {
            authorized_api.update(|a| *a = Some(api.clone()));
            token_has_been_verified.update(|a| *a = true);
            async_std::task::sleep(std::time::Duration::new(1,0)).await;
            match api.user_info().await {
              Ok(info) => {
                  async_std::task::sleep(std::time::Duration::new(1,0)).await;
                  LocalStorage::delete(API_TOKEN_OTP_KEY);
                  otp_authorized_api.update(|x| *x = None);
                  user_info.update(|i| *i = Some(info));
              }
              Err(err) => {
                  token_has_been_verified.update(|a| *a = true);
                  LocalStorage::delete(API_TOKEN_STORAGE_KEY);
                  log::error!("Unable to fetch user info: {err}")
              }
            }
          }
          else {
            token_has_been_verified.update(|a| *a = true);
            LocalStorage::delete(API_TOKEN_STORAGE_KEY);
            log::error!("Token has expired!")
          }
      }
    });

    let set_opt_token_into_local_storage = create_action(cx, move |_| async move {
        match otp_authorized_api.get() {
            Some(api) => {
                LocalStorage::set(API_TOKEN_OTP_KEY, api.token)
                    .expect("LocalStorage::set");
            },
            None => {
                log::error!("Unable to fetch otp validation info: validate credentials are first required!")
            }
        }
    });

    // -- init API -- //

    let unauthorized_api = api::UnauthorizedApi::new(DEFAULT_API_URL);
    if let Ok(token) = LocalStorage::get(API_TOKEN_STORAGE_KEY) {
        let api = api::AuthorizedApi::new(DEFAULT_API_URL, token);
        fetch_user_info.dispatch(api);
    }
    else{
      token_has_been_verified.update(|a| *a = true);
    }

    if let Ok(token) = LocalStorage::get(API_TOKEN_OTP_KEY) {
        let api = api::OtpAuthorizedApi::new(DEFAULT_API_URL, token);
        otp_authorized_api.update(|a| *a = Some(api));
    }

    log::debug!("User is logged in: {}", logged_in.get());

    view! { cx,
      <Router>
        <NavBar logged_in 
                on_logout = move || {
                    let navigate = use_navigate(cx);
                    navigate(Page::Login.path(), Default::default()).expect("Login route");
                    LocalStorage::delete(API_TOKEN_STORAGE_KEY);
                    authorized_api.update(|a| *a = None);
                    user_info.update(|a| *a = None);
                }/>
        <main>
          <Routes>
            <Route
              path=Page::Home.path()
              view=move |cx| view! { cx,
                <Home 
                    user_info = user_info.into()
                    api_signal= authorized_api
                    token_has_been_verified = token_has_been_verified
                    log_out_action = move || {
                      let navigate = use_navigate(cx);
                      navigate(Page::Login.path(), Default::default()).expect("Login route");
                    }
                />
              }
            />
            <Route
              path=Page::Login.path()
              view=move |cx| view! { cx,
                <Login
                  api = unauthorized_api
                  on_success = move |api: api::OtpAuthorizedApi| {
                      log::info!("Login first step succceeded!");
                      otp_authorized_api.update(|x| *x = Some(api.clone()));
                      set_opt_token_into_local_storage.dispatch(());
                      let navigate = use_navigate(cx);
                      navigate(Page::OtpValidation.path(), Default::default()).expect("OTP validation route");
                  } />
              }
            />
            <Route
              path=Page::Register.path()
              view=move |cx| view! { cx,
                <Register api = unauthorized_api />
              }
            />
            <Route
              path=Page::OtpValidation.path()
              view=move |cx| view! { cx,
                <Otp
                    otp_api_signal=otp_authorized_api
                    opt_verify_success_action=move |api: api::AuthorizedApi| {
                        log::info!("Authentication successfully performed!");
                        authorized_api.update(|x| *x = Some(api.clone()));
                        LocalStorage::set(API_TOKEN_STORAGE_KEY, api.clone().token).expect("LocalStorage::set");
                        let navigate = use_navigate(cx);
                        navigate(Page::Home.path(), Default::default()).expect("Home route");
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
