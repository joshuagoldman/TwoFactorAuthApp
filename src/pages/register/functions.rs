use std::{collections::HashMap, rc::Rc};

use leptos::{create_rw_signal, Action, RwSignal, Signal, SignalGet};

use crate::{
    api::api_boundary::NewUser,
    components::fields_error::get_all_error_fields,
    consts::{
        EMAIL_FIELD_STR, FIRST_NAME_FIELD_STR, LAST_NAME_FIELD_STR, PASSWORD_FIELD_STR,
        REPEAT_PASSWORD_FIELD_STR, USER_NAME_FIELD_STR,
    },
    misc::{GeneralFormField, Requirement},
};

pub fn all_reqs_fulfilled_func(
    reg_form_fields_signal: Signal<Vec<GeneralFormField>>,
    error_fields_signal: Signal<Vec<String>>,
) -> Signal<bool> {
    Signal::derive(move || {
        let password_fields: Vec<GeneralFormField> = reg_form_fields_signal
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

pub fn on_register_click(
    all_reqs_fulfilled: Signal<bool>,
    form_fields_map: HashMap<String, GeneralFormField>,
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
            full_name: format!("{} {}", first_name, last_name),
            email,
            password,
        };
        on_register.dispatch(new_user);
    }
}

pub fn get_register_form_fields(
    user_name: RwSignal<String>,
    first_name: RwSignal<String>,
    last_name: RwSignal<String>,
    email: RwSignal<String>,
    password_field: RwSignal<String>,
    password_field_repeat: RwSignal<String>,
) -> Vec<GeneralFormField> {
    vec![
        GeneralFormField {
            name: USER_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: user_name,
        },
        GeneralFormField {
            name: FIRST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: first_name,
        },
        GeneralFormField {
            name: LAST_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: last_name,
        },
        GeneralFormField {
            name: EMAIL_FIELD_STR.clone(),
            requirement: Some(Requirement {
                func: Rc::new(move |str_val| return str_val == &"joshuagoldman94@gmail.com"),
                fail_msg: "Email not valid".to_string(),
            }),
            is_password: false,
            signal: email,
        },
        GeneralFormField {
            name: PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: true,
            signal: password_field,
        },
        GeneralFormField {
            name: REPEAT_PASSWORD_FIELD_STR.to_string(),
            requirement: None,
            is_password: true,
            signal: password_field_repeat,
        },
    ]
}

pub fn get_form_fields_signals() -> (Signal<Vec<GeneralFormField>>, Signal<Vec<String>>) {
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
