use std::time::Duration;

use async_std::task;
use leptos::{
    component, create_action, create_rw_signal, view, IntoView, RwSignal, Signal, SignalGet,
    SignalUpdate,
};

use crate::{
    api::{api_boundary::ProfileInfo, authorized_api::AuthorizedApi, resulthandler::ResultHandler},
    components::fields_error::TextFieldErrors,
    misc,
};

#[component]
pub fn Home(
    authorized_api: AuthorizedApi,
    profile_info: RwSignal<Option<ProfileInfo>>,
) -> impl IntoView {
    let error_message_signal: RwSignal<Option<String>> = create_rw_signal(None);
    let error_fields_signal = Signal::derive(move || match error_message_signal.get() {
        Some(err_msg) => vec![err_msg],
        None => Vec::new(),
    });
    let get_profile_info = create_action(move |authorized_api: &AuthorizedApi| {
        let authorized_api = authorized_api.clone();
        async move {
            task::sleep(Duration::from_secs(1)).await;
            match authorized_api.get_user_data().await {
                ResultHandler::OkResult(user_data) => {
                    profile_info.update(|x| *x = Some(user_data));
                }
                ResultHandler::ErrResult(err) => {
                    error_message_signal.update(|x| *x = Some(err));
                }
            }
        }
    });

    get_profile_info.dispatch(authorized_api);

    view! {
        {move || {
                match profile_info.get() {
                    Some(profile_info_some) => view! {
                        <div style="color:white;">{move || {
                                format!("Welcome, {}", profile_info_some.full_name)
                            }}
                        </div>
                    },
                    None => view! {
                        <div>
                            <div style="color:white;">{"Getting profile info"}</div>
                            <TextFieldErrors
                                error_fields_signal
                            >
                            </TextFieldErrors>
                        </div>
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
