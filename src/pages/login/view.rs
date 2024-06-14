use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_rw_signal, view, IntoView, RwSignal, Show, Signal, SignalGet,
    SignalUpdate,
};

use crate::{
    api::{
        self,
        api_boundary::{ApiToken, Credentials},
        unauthorized_api::UnauthorizedApi,
    },
    components::{fields_error::TextFieldErrors, login_form::*},
    consts::API_TOKEN_OTP_KEY,
    misc,
    pages::{login::functions::get_login_form_signals, Page},
};

#[component]
pub fn Login(unauth_api: UnauthorizedApi) -> impl IntoView {
    let login_error_signal = create_rw_signal(None::<String>);
    let logging_in_signal = create_rw_signal(false);
    let user_name = create_rw_signal(String::new());
    let password_field = create_rw_signal(String::new());
    let (_, error_fields_signal, login_fields_map_signal) =
        get_login_form_signals(user_name, password_field);

    let credentials = Signal::derive(move || {
        if error_fields_signal.get().len() == 0 {
            Some(Credentials {
                username: user_name.get(),
                password: password_field.get(),
            })
        } else {
            None
        }
    });

    let on_login = create_action(move |login_data: &Credentials| {
        let login_data = login_data.clone();
        async move {
            logging_in_signal.update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;
            match unauth_api.login(&login_data).await {
                api::api_boundary::ResultHandler::OkResult(token_resp) => {
                    LocalStorage::set(
                        API_TOKEN_OTP_KEY.clone(),
                        ApiToken {
                            token: token_resp.token,
                        },
                    )
                    .unwrap();
                    misc::go_to_page(Page::OtpValidation)
                }
                api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                    login_error_signal.update(|x| *x = Some(err_msg));
                }
            }
            user_name.update(|x| *x = String::new());
            password_field.update(|x| *x = String::new());
            logging_in_signal.update(|upd: &mut bool| *upd = false);
        }
    });

    let additional_form_action = create_action(move |_| async move {
        login_error_signal.update(|x| *x = None);
    });

    view! {
        <div class="container">
            <div class="row d-flex justify-content-center align-items-center">
                <div class="blurry-card" style={move || format!("height:100%")}>
                    <div class="row justify-content-center">
                        <div class="col-md-10 col-lg-6 col-xl-5 order-2 order-lg-1">

                            <Show
                                when= move  || !logging_in_signal.get()
                                fallback= move || view! { <div></div>}
                            >
                                <p class="text-center h3 fw-bold mb-2 mx-1 mx-md-4 mt-4">Login</p>
                            </Show>

                            <form class="mx-1 mx-md-4">
                                <Show
                                    when= move  || !logging_in_signal.get()
                                    fallback= move || view! {
                                        <div>{"Logging in..."}</div>
                                    }
                                >
                                  <LoginForm
                                    login_fields_map_signal
                                    credentials
                                    on_login
                                    additional_form_action
                                  >
                                  </LoginForm>
                                </Show>
                                <TextFieldErrors error_fields_signal />
                                <LoginError login_error_signal/>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn LoginError(login_error_signal: RwSignal<Option<String>>) -> impl IntoView {
    view! {
        <Show
            when= move  || {login_error_signal.get().is_some()}
            fallback= move || view! { <div></div>}
        >
          <div style="color:red">{move || login_error_signal.get()}</div>

        </Show>
    }
}
