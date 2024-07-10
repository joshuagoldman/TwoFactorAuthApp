use api::api_boundary::ApiToken;
use api::otp_authorized_api::OtpAuthorizedApi;
use api_state::api_state_view_infos::{
    get_delete_view, get_home_view, get_login_view, get_otp_view, get_register_view,
    get_reset_view, get_unauth_view,
};
use consts::DEFAULT_API_URL;
use leptos::component;
use leptos::*;
use leptos_router::*;
use misc::ApiStateCheckView;
use pages::otp;

mod api;
mod api_state;
mod components;
mod consts;
mod misc;
mod pages;

use crate::pages::page_not_found::PageNotFound;
use crate::pages::Page;

#[component]
pub fn App() -> impl IntoView {
    let unauth_view_func = get_unauth_view();
    let login_view_info = get_login_view();
    let register_view_info = get_register_view();
    let otp_view_info = get_otp_view(unauth_view_func.clone());
    let home_view_info = get_home_view(unauth_view_func.clone());
    let reset_view_info = get_reset_view(unauth_view_func.clone());
    let delete_view_info = get_delete_view(unauth_view_func.clone());

    let otp_tst_view = OtpAuthorizedApi::new(
        &DEFAULT_API_URL,
        ApiToken {
            token: "ss".to_string(),
        },
    );

    view! {
        <Router>
            <main>
                <Routes>
                  <Route
                        path="test"
                        view= move || view! {
                        <otp::Otp otp_auth_api=otp_tst_view.clone()/>
                    }

                  />
                  <Route
                        path=Page::Reset.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = reset_view_info.clone()/>
                        }

                  />
                  <Route
                        path=Page::Delete.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = delete_view_info.clone()/>
                        }

                  />
                  <Route
                        path=Page::Register.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = register_view_info.clone()/>
                        }

                  />
                  <Route
                        path=Page::Login.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = login_view_info.clone()/>
                        }

                  />
                  <Route
                        path=Page::OtpValidation.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = otp_view_info.clone()/>
                        }

                  />
                  <Route
                        path=Page::Home.path().to_string().clone()
                        view= move || view! {
                            <ApiStateCheckView
                                view_info = home_view_info.clone()/>
                        }
                  />
                  <Route
                        path="*any"
                        view= move || view! {
                            <PageNotFound/>
                        }
                  />
                </Routes>
            </main>
        </Router>
    }
}
