use std::collections::HashMap;

use leptos::{RwSignal, Signal, SignalGet};

use crate::{
    consts::{CURRENT_PASSWORD_FIELD_STR, NEW_PASSWORD_FIELD_STR, REPEAT_NEW_PASSWORD_FIELD_STR},
    misc::GeneralFormField,
};

use super::misc::{is_allowed_field, PassVerificationAction};

fn get_password_verification_fields(
    current_password_signal: RwSignal<String>,
    new_password_signal: RwSignal<String>,
    new_password_repeat_signal: RwSignal<String>,
) -> Vec<GeneralFormField> {
    vec![
        GeneralFormField {
            name: CURRENT_PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: true,
            signal: current_password_signal,
        },
        GeneralFormField {
            name: NEW_PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: new_password_signal,
        },
        GeneralFormField {
            name: REPEAT_NEW_PASSWORD_FIELD_STR.clone(),
            requirement: None,
            is_password: false,
            signal: new_password_repeat_signal,
        },
    ]
}

pub fn get_password_verification_form_signals(
    current_password_signal: RwSignal<String>,
    new_password_signal: RwSignal<String>,
    new_password_repeat_signal: RwSignal<String>,
    action_type: PassVerificationAction,
    is_verification_mode: RwSignal<bool>,
) -> (
    Signal<Vec<GeneralFormField>>,
    Signal<Vec<String>>,
    Signal<HashMap<String, GeneralFormField>>,
) {
    let password_verification_form_signal = Signal::derive(move || {
        let password_verificaion_form_fields = get_password_verification_fields(
            current_password_signal,
            new_password_signal,
            new_password_repeat_signal,
        );
        password_verificaion_form_fields
    });

    let error_messages_signal = get_password_verification_error_fields(
        current_password_signal,
        new_password_signal,
        new_password_repeat_signal,
        is_verification_mode,
        action_type,
    );

    let password_verification_fields_map_signal = Signal::derive(move || {
        let mut form_fields_map: HashMap<String, GeneralFormField> = HashMap::new();

        let password_verification_form_fields_map_signal = get_password_verification_fields(
            current_password_signal,
            new_password_signal,
            new_password_repeat_signal,
        );
        for (_, field) in password_verification_form_fields_map_signal
            .iter()
            .enumerate()
        {
            form_fields_map.insert(field.name.clone(), field.clone());
        }
        form_fields_map
    });

    (
        password_verification_form_signal,
        error_messages_signal,
        password_verification_fields_map_signal,
    )
}

pub fn get_password_verification_error_fields(
    current_password_signal: RwSignal<String>,
    new_password_signal: RwSignal<String>,
    new_password_repeat_signal: RwSignal<String>,
    is_verification_mode: RwSignal<bool>,
    action_type: PassVerificationAction,
) -> Signal<Vec<String>> {
    Signal::derive(move || {
        let reg_form_fields = get_password_verification_fields(
            current_password_signal,
            new_password_signal,
            new_password_repeat_signal,
        );
        let mut errors = reg_form_fields
            .iter()
            .filter(|x| is_allowed_field(action_type.clone(), x, is_verification_mode))
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

        if !is_verification_mode.get()
            && !new_password_signal.get().is_empty()
            && !new_password_repeat_signal.get().is_empty()
            && new_password_signal.get() != new_password_repeat_signal.get()
        {
            errors.push("Passwords are not equal".to_string());
        }

        errors
    })
}
