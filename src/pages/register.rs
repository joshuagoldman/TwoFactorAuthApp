use leptos::{
    component, create_action, create_rw_signal, create_trigger, view, Action, CollectView,
    IntoView, RwSignal, Signal, SignalGet, SignalUpdate, View,
};

use crate::api::api_boundary::{NewUser, NewUserResponse, ResultHandler};
use crate::api::UnauthorizedApi;
use crate::consts::{
    EMAIL_FIELD_STR, FIRST_NAME_FIELD_STR, LAST_NAME_FIELD_STR, PASSWORD_FIELD_STR,
    USER_NAME_FIELD_STR,
};
use crate::{components::form_field::FormField, misc::RegisterFormField};
use async_std::task;
use std::time::Duration;

fn value_fullfills_req(str_val: &String, requirement: impl Fn(&String) -> bool) -> bool {
    requirement(str_val)
}

fn on_register_click(
    all_reqs_fulfilled: Signal<bool>,
    form_fields: Vec<RegisterFormField>,
    on_register: Action<NewUser, ()>,
) {
    if all_reqs_fulfilled.get() {
        let username = form_fields
            .clone()
            .iter()
            .find(|x| x.name == USER_NAME_FIELD_STR.clone())
            .unwrap()
            .name;
        let first_name = form_fields
            .iter()
            .find(|x| x.name == FIRST_NAME_FIELD_STR.clone())
            .unwrap()
            .name;
        let last_name = form_fields
            .iter()
            .find(|x| x.name == LAST_NAME_FIELD_STR.clone())
            .unwrap()
            .name;
        let email = form_fields
            .iter()
            .find(|x| x.name == EMAIL_FIELD_STR.clone())
            .unwrap()
            .name;
        let password = form_fields
            .iter()
            .find(|x| x.name == PASSWORD_FIELD_STR.clone())
            .unwrap()
            .name;
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

    let form_fields = vec![
        RegisterFormField {
            name: USER_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: create_rw_signal(String::new()),
        },
        RegisterFormField {
            name: FIRST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: create_rw_signal(String::new()),
        },
        RegisterFormField {
            name: LAST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: create_rw_signal(String::new()),
        },
        RegisterFormField {
            name: EMAIL_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: create_rw_signal(String::new()),
        },
        RegisterFormField {
            name: PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: true,
            signal: create_rw_signal(String::new()),
        },
        RegisterFormField {
            name: "Repeat Password".to_string(),
            requirement: None,
            is_password: true,
            signal: create_rw_signal(String::new()),
        },
    ];

    let form_fields_password = form_fields.clone();
    let all_reqs_fullfilled = Signal::derive(move || {
        let password_fields: Vec<&RegisterFormField> = form_fields_password
            .iter()
            .filter(|form_field| form_field.is_password)
            .collect();

        if password_fields.len() > 1 {
            let password_field_1 = password_fields[0].signal.get();
            let password_field_2 = password_fields[1].signal.get();

            !password_field_1.is_empty()
                && !password_field_1.is_empty()
                && password_field_1 == password_field_2
        } else {
            false
        }
    });

    let text_fields = form_fields
        .iter()
        .filter(|form_field| {
            !form_field.is_password && !form_field.name.to_uppercase().contains("EMAIL")
        })
        .map(|form_field| {
            let form_field = form_field.clone();
            view! {
                <FormField form_field/>
            }
        })
        .collect::<Vec<View>>();

    let password_fields = form_fields
        .iter()
        .filter(|form_field| form_field.is_password)
        .map(|form_field| {
            let form_field = form_field.clone();
            view! {
                <FormField form_field/>
            }
        })
        .collect::<Vec<View>>();

    let form_fields_email = form_fields.clone();
    let email_field = move || {
        let email_field_opt = form_fields_email.iter().find(|form_field| {
            !form_field.is_password && form_field.name.to_uppercase().contains("EMAIL")
        });

        if let Some(form_field) = email_field_opt {
            let form_field = form_field.clone();
            view! {
                <FormField form_field/>
            }
        } else {
            view! {
                <div/>
            }
            .into_view()
        }
    };

    view! {
        <div class="container">
            <div class="row centered-form">
                <div class="col-xs-12 col-sm-8 col-md-4 col-sm-offset-2 col-md-offset-4">
                    <div class="panel panel-default">
                        <div class="panel-heading">
                                <h3 class="panel-title">{"Be a real nigga and sign up"}<small>{"It's free! (jews are fond of this concept)"}</small></h3>
                        </div>
                        <div class="panel-body">
                            <form role="form">
                                <div class="row">
                                    {move || {
                                      text_fields.clone().collect_view()
                                    }}
                                </div>
                                <div class="row">
                                    {move || {
                                      email_field.clone()
                                    }}
                                </div>
                                <div class="row">
                                    {move || {
                                      password_fields.clone().collect_view()
                                    }}
                                </div>
                                    <input
                                        type="submit"
                                        disabled= {move || !all_reqs_fullfilled.get()}
                                        value="Register"
                                        class="btn btn-info btn-block"
                                        on:click= {move |_|{
                                          on_register_click(all_reqs_fullfilled,
                                                            form_fields.clone(),
                                                            on_register);
                                        }}/>
                            </form>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
