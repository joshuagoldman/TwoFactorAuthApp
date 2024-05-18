use leptos::leptos_dom::logging::console_log;
use leptos::{
    component, create_action, create_rw_signal, create_trigger, view, Action, CollectView,
    IntoView, RwSignal, Show, Signal, SignalGet, SignalGetUntracked, SignalUpdate, SignalWith,
    View,
};

use crate::api::api_boundary::{NewUser, NewUserResponse, ResultHandler};
use crate::api::UnauthorizedApi;
use crate::components::fields_error::{get_all_error_fields, TextFieldErrors};
use crate::components::form_field::{AllRegisterFields, EmailField, PasswordFields, TextFields};
use crate::consts::{
    EMAIL_FIELD_STR, FIRST_NAME_FIELD_STR, LAST_NAME_FIELD_STR, PASSWORD_FIELD_STR,
    REPEAT_PASSWORD_FIELD_STR, USER_NAME_FIELD_STR,
};
use crate::misc::RegisterFormField;
use crate::misc::Requirement;
use async_std::task;
use std::collections::HashMap;
use std::rc::Rc;
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
        let mut form_fields_map: HashMap<String, RegisterFormField> = HashMap::new();

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
    form_fields_map: Signal<HashMap<String, RegisterFormField>>,
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

fn all_reqs_fulfilled_func(
    reg_form_fields_signal: Signal<Vec<RegisterFormField>>,
    error_fields_signal: Signal<Vec<String>>,
) -> Signal<bool> {
    Signal::derive(move || {
        let password_fields: Vec<RegisterFormField> = reg_form_fields_signal
            .get()
            .into_iter()
            .filter(|form_field| form_field.is_password)
            .collect();

        if password_fields.len() > 1 {
            let password_field_1 = password_fields[0].signal.get();
            let password_field_2 = password_fields[1].signal.get();

            let res = !password_field_1.is_empty()
                && !password_field_1.is_empty()
                && password_field_1 == password_field_2
                && error_fields_signal.get().is_empty();

            res
        } else {
            false
        }
    })
}

fn on_register_click(
    all_reqs_fulfilled: Signal<bool>,
    form_fields_map: HashMap<String, RegisterFormField>,
    on_register: Action<NewUser, ()>,
) {
    if all_reqs_fulfilled.get() {
        let username = form_fields_map
            .get(&USER_NAME_FIELD_STR.clone())
            .unwrap()
            .signal
            .get()
            .clone();
        let first_name = form_fields_map
            .get(&FIRST_NAME_FIELD_STR.clone())
            .unwrap()
            .signal
            .get()
            .clone();
        let last_name = form_fields_map
            .get(&LAST_NAME_FIELD_STR.clone())
            .unwrap()
            .signal
            .get()
            .clone();
        let email = form_fields_map
            .get(&EMAIL_FIELD_STR.clone())
            .unwrap()
            .signal
            .get()
            .clone();
        let password = form_fields_map
            .get(&PASSWORD_FIELD_STR.clone())
            .unwrap()
            .signal
            .get()
            .clone();
        let new_user = NewUser {
            username,
            first_name,
            last_name,
            email,
            password,
        };
        on_register.dispatch(new_user);
    }
}

fn get_register_form_fields(
    user_name: RwSignal<String>,
    first_name: RwSignal<String>,
    last_name: RwSignal<String>,
    email: RwSignal<String>,
    password_field: RwSignal<String>,
    password_field_repeat: RwSignal<String>,
) -> Vec<RegisterFormField> {
    vec![
        RegisterFormField {
            name: USER_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: user_name,
        },
        RegisterFormField {
            name: FIRST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: first_name,
        },
        RegisterFormField {
            name: LAST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: last_name,
        },
        RegisterFormField {
            name: EMAIL_FIELD_STR.clone(),
            requirement: Some(Requirement {
                func: Rc::new(move |str_val| return str_val == &"joshuagoldman94@gmail.com"),
                fail_msg: "Email not valid".to_string(),
            }),
            is_password: false,
            signal: email,
        },
        RegisterFormField {
            name: PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: true,
            signal: password_field,
        },
        RegisterFormField {
            name: REPEAT_PASSWORD_FIELD_STR.to_string(),
            requirement: None,
            is_password: true,
            signal: password_field_repeat,
        },
    ]
}

fn get_form_fields_signals() -> (Signal<Vec<RegisterFormField>>, Signal<Vec<String>>) {
    let user_name = create_rw_signal(String::new());
    let first_name = create_rw_signal(String::new());
    let last_name = create_rw_signal(String::new());
    let email = create_rw_signal(String::new());
    let password_field = create_rw_signal(String::new());
    let password_field_repeat = create_rw_signal(String::new());

    let reg_forms_fields = get_register_form_fields(
        user_name,
        first_name,
        last_name,
        email,
        password_field,
        password_field_repeat,
    );

    let register_forms_signal = Signal::derive(move || {
        let reg_forms_fields = get_register_form_fields(
            user_name,
            first_name,
            last_name,
            email,
            password_field,
            password_field_repeat,
        );

        reg_forms_fields
    });

    let error_messages_signal = get_all_error_fields(reg_forms_fields);

    (register_forms_signal, error_messages_signal)
}
