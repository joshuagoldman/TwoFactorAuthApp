use leptos::{
    component, create_action, create_rw_signal, view, Action, IntoView, RwSignal, Show, Signal,
    SignalGet, SignalUpdate,
};

use crate::api::api_boundary::{NewUser, NewUserResponse, ResultHandler};
use crate::api::UnauthorizedApi;
use crate::components::fields_error::TextFieldErrors;
use crate::components::form_field::AllRegisterFields;
use crate::misc::GeneralFormField;
use crate::pages::register::functions::{
    all_reqs_fulfilled_func, get_form_fields_signals, on_register_click,
};
use async_std::task;
use std::collections::HashMap;
use std::time::Duration;

#[component]
pub fn Register(unatuhorized_api: UnauthorizedApi) -> impl IntoView {
    let registering = create_rw_signal(false);
    let registering_succeded: RwSignal<Option<NewUserResponse>> = create_rw_signal(None);
    let error_message: RwSignal<Option<String>> = create_rw_signal(None);

    let on_register = create_action(move |new_user: &NewUser| {
        let new_user = new_user.clone();
        let registering = registering.clone();

        async move {
            registering.update(|x| *x = true);
            task::sleep(Duration::from_secs(2)).await;
            match unatuhorized_api.register(&new_user).await {
                ResultHandler::OkResult(ok_res) => {
                    registering.update(|x| *x = false);
                    registering_succeded.update(|x| *x = Some(ok_res));
                }
                ResultHandler::ErrResult(err_msg) => {
                    registering.update(|x| *x = false);
                    error_message.update(|err| *err = Some(err_msg))
                }
            }
        }
    });

    let (form_fields, error_fields_signal) = get_form_fields_signals();

    let form_fields_map_signal = Signal::derive(move || {
        let mut form_fields_map: HashMap<String, GeneralFormField> = HashMap::new();

        for (_, field) in form_fields.get().iter().enumerate() {
            form_fields_map.insert(field.name.clone(), field.clone());
        }
        form_fields_map
    });

    let all_reqs_fulfilled = all_reqs_fulfilled_func(form_fields, error_fields_signal);

    view! {
        <div class="container">
            <div class="row d-flex justify-content-center align-items-center">
                <div class="blurry-card" style={move || format!("height:100%")}>
                    <div class="row justify-content-center">
                        <div class="col-md-10 col-lg-6 col-xl-5 order-2 order-lg-1">

                            <Show
                                when= move  || !registering.get() && registering_succeded.get().is_none()
                                fallback= move || view! { <div></div>}
                            >
                                <p class="text-center h3 fw-bold mb-2 mx-1 mx-md-4 mt-4">Register</p>
                            </Show>

                            <form class="mx-1 mx-md-4">
                                <Show
                                    when= move  || !registering.get() && registering_succeded.get().is_none()
                                    fallback= move || view! {
                                        <Registering is_registering=registering></Registering>
                                    }
                                >
                                    <RegisterFields
                                        form_fields_map= form_fields_map_signal
                                        all_reqs_fulfilled
                                        on_register
                                    >
                                    </RegisterFields>
                                </Show>
                                <TextFieldErrors
                                    error_fields_signal
                                >
                                </TextFieldErrors>
                            </form>
                        </div>
                        <Show
                            when= move  || registering_succeded.get().is_some()
                            fallback= move || view! { <div></div>}
                        >
                            <BarCode new_user_info=registering_succeded.get().unwrap_or_default()/>
                        </Show>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Registering(is_registering: RwSignal<bool>) -> impl IntoView {
    view! {
        <Show
            when= move  || is_registering.get()
            fallback= move || view! {
                <div>{"Registering succeeded. Scan the qr code above in an authentication app"}</div>
            }
        >
            <div>{"Registering..."}</div>
        </Show>
    }
}

#[component]
fn RegisterFields(
    form_fields_map: Signal<HashMap<String, GeneralFormField>>,
    all_reqs_fulfilled: Signal<bool>,
    on_register: Action<NewUser, ()>,
) -> impl IntoView {
    view! {

        <AllRegisterFields form_fields_map></AllRegisterFields>
        <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
            <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                        disabled= {move || !all_reqs_fulfilled.get()}
                        on:click= {move |_|{
                        on_register_click(all_reqs_fulfilled,
                                            form_fields_map.get(),
                                            on_register);
                        }}
            >Register</button>
        </div>
    }
}

#[component]
fn BarCode(new_user_info: NewUserResponse) -> impl IntoView {
    view! {
        <div class="d-flex flex-row align-items-center mb-4">
            <div>
                <img id="barcode"
                    src={new_user_info.qr_code}
                    alt=""
                    title="Scan this with an authentication app"
                    width="200"
                    height="200"
                />
            </div>
        </div>
    }
}
