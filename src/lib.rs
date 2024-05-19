use std::rc::Rc;

use api::AuthorizedApi;
use leptos::component;
use leptos::*;
use leptos_router::*;

mod api;
mod components;
mod consts;
mod misc;
mod pages;

use crate::api::api_boundary::ProfileInfo;
use crate::api::OtpAuthorizedApi;
use crate::misc::{ApiStateCheckView, ApiStateView};
use crate::pages::home::Home;
use crate::pages::login::view::Login;
use crate::pages::register::view::Register;
use crate::pages::Page;

#[component]
pub fn App() -> impl IntoView {
    let unauth_view_func = move |unauth_api: api::UnauthorizedApi| {
        view! {
            <Login unauth_api
            />
        }
    };
    let unauth_view_func = Rc::new(unauth_view_func);

    let register_view_func = move |unatuhorized_api: api::UnauthorizedApi| {
        view! {
            <Register unatuhorized_api
            />
        }
    };
    let register_view_func = Rc::new(register_view_func);

    let otp_auth_view_func = move |otp_auth_api: OtpAuthorizedApi| {
        view! {
            <div style="color:white">{"Not yet implemented"}</div>
        }
        .into_view()
    };
    let otp_auth_view_func = Rc::new(otp_auth_view_func);

    let profile_info: RwSignal<Option<ProfileInfo>> = create_rw_signal(None);
    let auth_view_func = move |authorized_api: AuthorizedApi| {
        view! {
            <Home authorized_api
                  profile_info
            />
        }
    };
    let auth_view_func = Rc::new(auth_view_func);
    let unauth_view_func_login = unauth_view_func.clone();

    view! {
        <Router>
            <main>
                <Routes>
                  <Route
                        path=Page::Register.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view = misc::ApiStateView::UnAuth(register_view_func.clone())
                            />
                        }

                  />
                  <Route
                        path=Page::Login.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view = misc::ApiStateView::UnAuth(unauth_view_func_login.clone())
                            />
                        }

                  />
                  <Route
                        path=Page::OtpValidation.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view = misc::ApiStateView::OTPAuth(unauth_view_func.clone(),
                                                                  otp_auth_view_func.clone())
                            />
                        }

                  />
                </Routes>
            </main>
        </Router>
    }
}
