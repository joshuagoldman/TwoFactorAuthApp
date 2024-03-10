use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_rw_signal, view, IntoView, RwSignal, Show, Signal, SignalGet,
    SignalUpdate,
};

use regex::Regex;

use crate::{
    api::{
        api_boundary::{ProfileInfo, ResultHandler},
        AuthorizedApi, OtpAuthorizedApi,
    },
    consts::API_TOKEN_STORAGE_KEY,
    misc,
    pages::Page,
};

#[component]
pub fn Otp(otp_auth_api: OtpAuthorizedApi) -> impl IntoView {
    let otp_num_1_signal: RwSignal<String> = create_rw_signal(String::new());
    let otp_num_2_signal: RwSignal<String> = create_rw_signal(String::new());
    let otp_num_3_signal: RwSignal<String> = create_rw_signal(String::new());
    let otp_num_4_signal: RwSignal<String> = create_rw_signal(String::new());
    let otp_num_5_signal: RwSignal<String> = create_rw_signal(String::new());
    let otp_num_6_signal: RwSignal<String> = create_rw_signal(String::new());
    let is_verifying_otp = create_rw_signal(false);
    let err_msg: RwSignal<Option<String>> = create_rw_signal(None);

    let re = Regex::new(r"\d{1}").unwrap();

    let all_otp_entered = Signal::derive(move || {
        otp_num_1_signal.get().is_empty()
            && !otp_num_2_signal.get().is_empty()
            && !otp_num_3_signal.get().is_empty()
            && !otp_num_4_signal.get().is_empty()
            && !otp_num_5_signal.get().is_empty()
            && !otp_num_6_signal.get().is_empty()
            && re.is_match(otp_num_1_signal.get().as_str())
            && re.is_match(otp_num_2_signal.get().as_str())
            && re.is_match(otp_num_3_signal.get().as_str())
            && re.is_match(otp_num_4_signal.get().as_str())
            && re.is_match(otp_num_5_signal.get().as_str())
            && re.is_match(otp_num_6_signal.get().as_str())
    });

    let otp_signal_derived = Signal::derive(move || {
        format!(
            "{}{}{}{}{}{}",
            otp_num_1_signal.get(),
            otp_num_2_signal.get(),
            otp_num_3_signal.get(),
            otp_num_4_signal.get(),
            otp_num_5_signal.get(),
            otp_num_6_signal.get()
        )
    });

    let on_verify_otp = create_action(move |otp: &String| {
        let otp = otp.clone();
        let otp_auth_api = otp_auth_api.clone();
        async move {
            is_verifying_otp.update(|x| *x = true);
            err_msg.update(|x| *x = None);

            task::sleep(Duration::from_secs(4)).await;
            match otp_auth_api.check_otp(&otp).await {
                ResultHandler::OkResult(ok_res) => {
                    LocalStorage::set(API_TOKEN_STORAGE_KEY.clone(), ok_res.token.token.clone());
                    misc::go_to_page(Page::Home);
                }
                ResultHandler::ErrResult(err_msg_str) => {
                    err_msg.update(|err| *err = Some(err_msg_str));
                }
            }
            is_verifying_otp.update(|x| *x = false);
        }
    });

    view! {
        <Show
            when= {move || !is_verifying_otp.get()}
            fallback= {move || view! {<div style="color:red">{"Verifying OTP..."}</div>}}
        >
            <section class="container-fluid bg-body-tertiary d-block">
                <div class="row justify-content-center">
                    <div class="col-12 col-md-6 col-lg-4" style="min-width: 500px;">
                        <div class="card bg-white mb-5 mt-5 border-0" style="box-shadow: 0 12px 15px rgba(0, 0, 0, 0.02);">
                        <div class="card-body p-5 text-center">
                            <h4>{"Verify"}</h4>
                            <p>{"Check your code on qr-code app..."}</p>

                            <div class="otp-field mb-4">
                            <input type="number" />
                            <input type="number" disabled />
                            <input type="number" disabled />
                            <input type="number" disabled />
                            <input type="number" disabled />
                            <input type="number" disabled />
                            </div>

                            <button class="btn btn-primary mb-3"
                                    disabled= {move || all_otp_entered.get()}
                                    on:click= {move |_| {
                                         on_verify_otp.dispatch(otp_signal_derived.get())
                                    }}
                            >
                                {"Verify"}
                            </button>
                            <div
                                 disabled={move || err_msg.get().is_none()}
                                 style="color:red"
                            >
                              {move || err_msg.get()}
                            </div>
                        </div>
                        </div>
                    </div>
                </div>
            </section>
        </Show>
    }
}
