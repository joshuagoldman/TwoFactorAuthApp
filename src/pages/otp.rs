use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::*;
use leptos_router::use_navigate;
use crate::API_TOKEN_OTP_KEY;
use crate::pages::Page;
use crate::{api, components::otp_form::OtpForm};

use crate::{
    api::OtpAuthorizedApi, api::AuthorizedApi,
};

use crate::otp_form::OtpFormProps;

#[component]
pub fn Otp<F>(cx: Scope, 
           otp_api_signal: RwSignal<Option<OtpAuthorizedApi>>,
           opt_verify_success_action: F) -> impl IntoView
where
    F: Fn(AuthorizedApi) + 'static + Clone
{
    let (is_sending_otp, set_is_sending_otp) = create_signal(cx, false);
    let (failed_otp_error, set_otp_error) = create_signal(cx, None::<String>);
    
    let send_otp_action = 
        create_action(cx, move |(otp_api_val_ref,otp_str) : &(OtpAuthorizedApi,String)| {
            log!("Try to verify OTP {}", otp_str);
            let otp_str = otp_str.clone();
            let otp_api_val = otp_api_val_ref.clone();
            let opt_verify_success_action = opt_verify_success_action.clone();
            async move {
                set_is_sending_otp.update(|w| *w = true);
                task::sleep(Duration::from_secs(3)).await;
                let result = otp_api_val.check_otp(&otp_str).await;
                set_is_sending_otp.update(|w| *w = false);
                match result {
                    Ok(res) => {
                        opt_verify_success_action(res);
                    }
                    Err(err) => {
                        let msg = match err {
                            api::Error::Fetch(js_err) => {
                                format!("{js_err:?}")
                            }
                            api::Error::Api(err) => err.message,
                        };
                        log::warn!(
                            "OTP Validation failed: {msg}"
                        );
                        LocalStorage::delete(API_TOKEN_OTP_KEY);
                        set_otp_error.update(|e| *e = Some(msg));
                    }
                }
            }
        });

    view! { cx,
        {move || match otp_api_signal.get() {
                Some(otp_api) => {       
                    view! { cx,
                        <Show
                        when = move || failed_otp_error.get().is_some()
                        fallback = move |_| view!{ cx,
                            <OtpForm
                                send_otp=send_otp_action
                                api=otp_api.clone()
                                is_sending_otp
                            />
                        }
                        >
                            <div style = "color:white;">
                                {failed_otp_error.get().unwrap()}
                            </div>
                        </Show>
                    }.into_view(cx)
                },
                None => {
                    {
                        move || {
                            let navigate = use_navigate(cx);
                            navigate(Page::Login.path(), Default::default()).expect("Login route");
                            view!{ cx,
                                <div/>
                            }
                        }
                    }.into_view(cx)
                }
            }
        }
    }
}
