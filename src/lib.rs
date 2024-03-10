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
use crate::misc::ApiSignals;
use crate::misc::RouteWithApiStateCheck;
use crate::pages::home::Home;
use crate::pages::login::Login;
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

    let otp_auth_view_func = move |authorized_api: AuthorizedApi| {
        view! {
            <div style="color:red">{"Not yet implemented"}</div>
        }
    };
    let otp_auth_view_func = Rc::new(&otp_auth_view_func);

    let profile_info: RwSignal<Option<ProfileInfo>> = create_rw_signal(None);
    let auth_view_func = move |authorized_api: AuthorizedApi| {
        view! {
            <Home authorized_api
                  profile_info
            />
        }
    };
    let auth_view_func = Rc::new(auth_view_func);

    view! {
        <Router>
            <main>
                <Routes>
                    <RouteWithApiStateCheck
                        path=Page::Home.path().to_string().clone()
                        view = misc::ApiStateView::Auth(unauth_view_func.clone(),auth_view_func)
                    />
                    <RouteWithApiStateCheck
                        path=Page::Login.path().to_string().clone()
                        view = misc::ApiStateView::UnAuth(unauth_view_func)
                    />
                </Routes>
            </main>
        </Router>
    }
}
