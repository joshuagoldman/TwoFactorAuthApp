use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_rw_signal, ev, event_target_value, view, IntoView, RwSignal,
    Show, Signal, SignalGet, SignalUpdate,
};
use web_sys::console::error_0;

use crate::{
    api::{self, api_boundary::ProfileInfo, AuthorizedApi},
    consts::API_TOKEN_STORAGE_KEY,
    misc,
    pages::Page,
};

#[component]
pub fn PasswordReset(authorized_api: AuthorizedApi) -> impl IntoView {
    let error_msg = create_rw_signal(None::<String>);
    let is_enter_current_passwrd = create_rw_signal(true);
    let is_loading = create_rw_signal(false);
    let page_title = Signal::derive(move || {
        if is_enter_current_passwrd.get() {
            "Enter existing password".to_string()
        } else {
            "Enter new password".to_string()
        }
    });
    let field_label = Signal::derive(move || {
        if is_enter_current_passwrd.get() {
            "Current password".to_string()
        } else {
            "New password".to_string()
        }
    });
    let button_title = Signal::derive(move || {
        if is_enter_current_passwrd.get() {
            "Validate current password".to_string()
        } else {
            "Update new password".to_string()
        }
    });
    let curr_field_val = create_rw_signal(String::new());
    let is_button_click_allowed = Signal::derive(move || !curr_field_val.get().is_empty());

    let reset_action = create_action(move |field_val: &String| {
        let authorized_api = authorized_api.clone();

        async move {
            is_loading.update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;

            if is_enter_current_passwrd.get() {
                match authorized_api
                    .validate_password(&authorized_api.token.token, &field_val)
                    .await
                {
                    api::api_boundary::ResultHandler::OkResult(token_resp) => {
                        LocalStorage::delete(&API_TOKEN_STORAGE_KEY.clone());
                        misc::go_to_page(Page::Login)
                    }
                    api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                        error_msg.update(|x| *x = Some(err_msg));
                    }
                }
            } else {
            }
            set_wait_for_response.update(|upd: &mut bool| *upd = false);
        }
    });

    view! {
            <div class="row d-flex justify-content-center align-items-center h-10">
                <div class="col-12 col-md-8 col-lg-6 col-xl-5">
                    <div class="blurry-card" style="border-radius: 1rem;">
                        <div class="card-body p-5 text-center">
                            <div class="mb-md-5 mt-md-4 pb-5">

                                <h2 class="fw-bold text-white mb-2 text-uppercase">{"Login"}</h2>
                                <p class="text-white mb-5">{move || page_title.get()}</p>

                                <div class="form-outline text-white form-white mb-4">
                                    <input type="password"
                                            id="typePasswordX"
                                            class="form-control form-control-lg"
                                            on:keyup = move |ev: ev::KeyboardEvent| {
                                                match &*ev.key() {
                                                    "enter" => {
                                                        if is_button_click_allowed.get() {
                                                            dispatch_action();
                                                        }
                                                    }
                                                    _=> {
                                                        let val = event_target_value(&ev);
                                                        curr_field_val.update(|p|*p = val);
                                                    }
                                                }
                                            } />
                                    <label class="form-label" for="typeEmailX">{"Username"}</label>
                                </div>

                                <button class="btn btn-outline-light btn-lg px-5"
                                        type="submit"
                                        disabled= {move || is_button_click_allowed.get()}
                                        on:click = move |_| {
                                            dispatch_action();
                                        }>{move || button_title.get()}</button>
                                <div>
                                    <p class="mt-5 pb-lg-2 text-white"
                                       style={move || {
                                           if !error_msg.get().is_some() {
                                              "color:red;display:none"
                                            }
                                            else {
                                              "color:red"
                                            }
                                        }}>{move || error_msg.get()}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
    }
}
