use std::collections::HashMap;

use leptos::{RwSignal, Signal, SignalGet};

use crate::{
    consts::{PASSWORD_FIELD_STR, USER_NAME_FIELD_STR},
    misc::RegisterFormField,
};

fn get_login_fields(
    user_name: RwSignal<String>,
    password_field: RwSignal<String>,
) -> Vec<RegisterFormField> {
    vec![
        RegisterFormField {
            name: USER_NAME_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: user_name,
        },
        RegisterFormField {
            name: PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: true,
            signal: password_field,
        },
    ]
}

pub fn get_login_form_signals(
    user_name: RwSignal<String>,
    password_field: RwSignal<String>,
) -> (
    Signal<Vec<RegisterFormField>>,
    Signal<Vec<String>>,
    Signal<HashMap<String, RegisterFormField>>,
) {
    let login_form_fields = get_login_fields(user_name, password_field);

    let login_forms_signal = Signal::derive(move || {
        let login_form_fields = get_login_fields(user_name, password_field);

        login_form_fields
    });

    let error_messages_signal = get_login_error_fields(user_name, password_field);

    let login_fields_map_signal = Signal::derive(move || {
        let mut form_fields_map: HashMap<String, RegisterFormField> = HashMap::new();

        for (_, field) in get_login_fields(user_name, password_field)
            .iter()
            .enumerate()
        {
            form_fields_map.insert(field.name.clone(), field.clone());
        }
        form_fields_map
    });

    (
        login_forms_signal,
        error_messages_signal,
        login_fields_map_signal,
    )
}

pub fn get_login_error_fields(
    user_name: RwSignal<String>,
    password_field: RwSignal<String>,
) -> Signal<Vec<String>> {
    Signal::derive(move || {
        let reg_form_fields = get_login_fields(user_name, password_field);
        let errors = reg_form_fields
            .iter()
            .map(|x| {
                let signal_val = x.signal.get();

                match x.requirement.clone() {
                    Some(rqrmnt) => {
                        if signal_val.is_empty() {
                            Some(format!("field {} is empty", x.name))
                        } else if !rqrmnt.func.clone()(&signal_val) {
                            Some(rqrmnt.fail_msg)
                        } else {
                            None
                        }
                    }
                    None => {
                        if signal_val.is_empty() {
                            Some(format!("field {} is empty", x.name))
                        } else {
                            None
                        }
                    }
                }
            })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();

        errors
    })
}
