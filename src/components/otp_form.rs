use crate::api::OtpAuthorizedApi;
use leptos::*;
use web_sys::MouseEvent;

#[component]
pub fn OtpForm(
    send_otp: Action<(OtpAuthorizedApi, String), ()>,
    is_sending_otp: ReadSignal<bool>,
    api: OtpAuthorizedApi,
) -> impl IntoView {
    let (first_input_signal, first_input_set) = create_signal(None::<String>);
    let (second_input_signal, second_input_set) = create_signal(None::<String>);
    let (third_input_signal, third_input_set) = create_signal(None::<String>);
    let (fourth_input_signal, fourth_input_set) = create_signal(None::<String>);
    let (fifth_input_signal, fifth_input_set) = create_signal(None::<String>);
    let (sixth_input_signal, sixth_input_set) = create_signal(None::<String>);

    let send_otp_enabled = first_input_signal.get().is_some()
        && second_input_signal.get().is_some()
        && third_input_signal.get().is_some()
        && fourth_input_signal.get().is_some()
        && fifth_input_signal.get().is_some()
        && sixth_input_signal.get().is_some();

    let on_key_up_ev = move |signal: WriteSignal<Option<String>>, api: OtpAuthorizedApi| {
        move |ev: ev::KeyboardEvent| {
            let api = api.clone();
            match (&*ev.key(), send_otp_enabled) {
                ("Enter", false) => {
                    let otp = format!(
                        "{}{}{}{}{}{}",
                        first_input_signal.get().unwrap(),
                        second_input_signal.get().unwrap(),
                        third_input_signal.get().unwrap(),
                        fourth_input_signal.get().unwrap(),
                        fifth_input_signal.get().unwrap(),
                        sixth_input_signal.get().unwrap()
                    );
                    send_otp.dispatch((api, otp));
                }
                (_, _) => {
                    let val = event_target_value(&ev);
                    if val.is_empty() {
                        signal.update(|p| *p = None);
                    } else {
                        signal.update(|p| *p = Some(val));
                    }
                }
            }
        }
    };

    let on_change_ev = move |signal: WriteSignal<Option<String>>| {
        move |ev: web_sys::Event| {
            let val = event_target_value(&ev);
            if val.is_empty() {
                signal.update(|p| *p = None);
            } else {
                signal.update(|p| *p = Some(val));
            }
        }
    };

    let on_click_ev = move |api: OtpAuthorizedApi| {
        move |_ev: MouseEvent| {
            let api = api.clone();
            let otp = format!(
                "{}{}{}{}{}{}",
                first_input_signal.get().unwrap(),
                second_input_signal.get().unwrap(),
                third_input_signal.get().unwrap(),
                fourth_input_signal.get().unwrap(),
                fifth_input_signal.get().unwrap(),
                sixth_input_signal.get().unwrap()
            );
            send_otp.dispatch((api, otp));
        }
    };

    let send_otp_ready = Signal::derive(move || {
        first_input_signal.get().is_some()
            && second_input_signal.get().is_some()
            && third_input_signal.get().is_some()
            && fourth_input_signal.get().is_some()
            && fifth_input_signal.get().is_some()
            && sixth_input_signal.get().is_some()
    });

    view! {
        <div class="container height-100 d-flex justify-content-center align-items-center">
            <div class="position-relative">
                <div class="card p-2 text-center">
                    <div style = "color:white;">"Please enter 6-digit code given from your authentication app."</div>
                    <div id="otp" class="inputs d-flex flex-row justify-content-center mt-2">
                        <OtpInput
                            on_keyup=on_key_up_ev(first_input_set, api.clone())
                            on_change=on_change_ev(first_input_set)
                        />
                        <OtpInput
                            on_keyup=on_key_up_ev(second_input_set, api.clone())
                            on_change=on_change_ev(second_input_set)
                        />
                        <OtpInput
                            on_keyup=on_key_up_ev(third_input_set, api.clone())
                            on_change=on_change_ev(third_input_set)
                        />
                        <OtpInput
                            on_keyup=on_key_up_ev(fourth_input_set, api.clone())
                            on_change=on_change_ev(fourth_input_set)
                        />
                        <OtpInput
                            on_keyup=on_key_up_ev(fifth_input_set, api.clone())
                            on_change=on_change_ev(fifth_input_set)
                        />
                        <OtpInput
                            on_keyup=on_key_up_ev(sixth_input_set, api.clone())
                            on_change=on_change_ev(sixth_input_set)
                        />
                    </div>
                    <div style = "color:white;"
                         prop:hidden = move || !is_sending_otp.get()>
                        "Sending OTP..."
                    </div>
                    <div class="mt-4">
                        <button prop:hidden = move || !send_otp_ready.get()
                                class="btn"
                                style="background-color:blue;color:white"
                                on:click=on_click_ev(api.clone())
                                prop:disabled = move || send_otp_enabled>"Validate"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn OtpInput<F, G>(on_keyup: F, on_change: G) -> impl IntoView
where
    F: FnMut(ev::KeyboardEvent) + 'static + Clone,
    G: FnMut(web_sys::Event) + 'static + Clone,
{
    view! {
        <input class="m-2 text-center form-control rounded"
            type="text"
            id="sixth"
            maxlength="1"
            on:keyup=on_keyup
            on:change=on_change
             />
    }
}
