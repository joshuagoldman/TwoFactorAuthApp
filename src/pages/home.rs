use std::time::Duration;

use async_std::task;
use leptos::{component, create_action, view, IntoView, RwSignal, SignalGet, SignalUpdate};

use crate::{
    api::{api_boundary::ProfileInfo, AuthorizedApi},
    misc,
};

#[component]
pub fn Home(
    authorized_api: AuthorizedApi,
    profile_info: RwSignal<Option<ProfileInfo>>,
) -> impl IntoView {
    let get_profile_info = create_action(move |authorized_api: &AuthorizedApi| {
        let authorized_api = authorized_api.clone();
        async move {
            task::sleep(Duration::from_secs(1)).await;
            match authorized_api.get_user_data().await {
                crate::api::api_boundary::ResultHandler::OkResult(user_data) => {
                    profile_info.update(|x| *x = Some(user_data));
                }
                crate::api::api_boundary::ResultHandler::ErrResult(_) => todo!(),
            }
        }
    });

    get_profile_info.dispatch(authorized_api);

    view! {
        {move || {
                match profile_info.get() {
                    Some(profile_info_some) => view! {
                        <div style="color:white;">{move || {
                                format!("Welcome mynigga, {}", profile_info_some.name)
                            }}
                        </div>
                    },
                    None => view! {
                        <div style="color:white;">{"Getting profile info"}</div>
                    }
                }
            }
        }
        <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
            <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                        on:click= {move |_| misc::log_out() }
            >Log Out</button>
        </div>
        <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
            <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                        on:click= {move |_| misc::go_to_page(crate::pages::Page::Reset) }
            >Reset Password</button>
        </div>
        <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
            <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                        on:click= {move |_| misc::go_to_page(crate::pages::Page::Delete) }
            >Delete Account</button>
        </div>
    }
}
