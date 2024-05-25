use std::{collections::HashMap, time::Duration};

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{create_action, Action, RwSignal, Signal, SignalGet, SignalUpdate};
use leptos_router::use_navigate;

use crate::{
    api::{self, api_boundary::ResultHandler, AuthorizedApi},
    consts::{
        API_TOKEN_STORAGE_KEY, CURRENT_PASSWORD_FIELD_STR, NEW_PASSWORD_FIELD_STR,
        REPEAT_NEW_PASSWORD_FIELD_STR,
    },
    misc::GeneralFormField,
    pages::Page,
};

#[derive(Debug, Clone)]
pub enum PassVerificationAction {
    ResetPassword,
    DeleteAccount,
}

#[derive(Clone)]
pub struct PassVerificationActionData {
    pub authorized_api: AuthorizedApi,
    pub is_loading: RwSignal<bool>,
    pub is_enter_current_passwrd: RwSignal<bool>,
    pub current_password_signal: RwSignal<String>,
    pub new_password_signal: RwSignal<String>,
    pub new_password_repeat_signal: RwSignal<String>,
    pub result: RwSignal<Option<ResultHandler<String>>>,
    pub action_type: PassVerificationAction,
}

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
        let errors = reg_form_fields
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

        errors
    })
}

pub fn get_is_action_enabled_signal(
    action_type: PassVerificationAction,
    any_error: Signal<bool>,
    is_verification_mode: RwSignal<bool>,
    passwrods_are_equal: Signal<bool>,
) -> Signal<bool> {
    Signal::derive(move || match action_type {
        PassVerificationAction::ResetPassword => {
            if is_verification_mode.get() && !any_error.get() {
                true
            } else if !is_verification_mode.get() && !any_error.get() && passwrods_are_equal.get() {
                true
            } else {
                false
            }
        }
        PassVerificationAction::DeleteAccount => {
            if is_verification_mode.get() && !any_error.get() {
                true
            } else if !is_verification_mode.get() {
                true
            } else {
                false
            }
        }
    })
}

fn is_allowed_field(
    action_type: PassVerificationAction,
    curr_field: &GeneralFormField,
    is_verification_mode: RwSignal<bool>,
) -> bool {
    match action_type {
        PassVerificationAction::ResetPassword => {
            if is_verification_mode.get()
                && (curr_field.name == NEW_PASSWORD_FIELD_STR.to_string()
                    || curr_field.name == REPEAT_NEW_PASSWORD_FIELD_STR.to_string())
            {
                false
            } else {
                true
            }
        }
        PassVerificationAction::DeleteAccount => {
            if curr_field.name == NEW_PASSWORD_FIELD_STR.to_string()
                || curr_field.name == REPEAT_NEW_PASSWORD_FIELD_STR.to_string()
            {
                false
            } else {
                true
            }
        }
    }
}

fn get_verify_password_action(pass_ver_data: PassVerificationActionData) -> Action<(), ()> {
    create_action(move |_| {
        let pass_ver_data = pass_ver_data.clone();
        async move {
            match pass_ver_data
                .clone()
                .authorized_api
                .validate_password(&pass_ver_data.current_password_signal.get())
                .await
            {
                api::api_boundary::ResultHandler::OkResult(true) => {
                    pass_ver_data
                        .is_enter_current_passwrd
                        .update(|upd: &mut bool| *upd = false);
                }
                api::api_boundary::ResultHandler::OkResult(false) => {
                    pass_ver_data.result.update(|x| {
                        *x = Some(ResultHandler::ErrResult("Verification Failed".to_string()))
                    });
                    set_signal_state_to_init(pass_ver_data);
                }
                api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                    pass_ver_data
                        .result
                        .update(|x| *x = Some(ResultHandler::ErrResult(err_msg)));
                    set_signal_state_to_init(pass_ver_data);
                }
            }
        }
    })
}

pub fn reset_action(pass_ver_data: PassVerificationActionData) -> Action<(), ()> {
    create_action(move |_| {
        let authorized_api = pass_ver_data.authorized_api.clone();
        let pass_ver_data = pass_ver_data.clone();

        async move {
            pass_ver_data
                .clone()
                .is_loading
                .update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;

            match authorized_api
                .reset_password(
                    &authorized_api.token.token,
                    &pass_ver_data.new_password_signal.get(),
                )
                .await
            {
                api::api_boundary::ResultHandler::OkResult(token_resp) => {
                    ok_result_handle(pass_ver_data).await;
                }
                api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                    pass_ver_data
                        .result
                        .update(|x| *x = Some(ResultHandler::ErrResult(err_msg)));
                    set_signal_state_to_init(pass_ver_data);
                    LocalStorage::delete(&API_TOKEN_STORAGE_KEY.clone());
                }
            }
        }
    })
}

pub fn delete_account_action(pass_ver_data: PassVerificationActionData) -> Action<(), ()> {
    create_action(move |_| {
        let authorized_api = pass_ver_data.authorized_api.clone();
        let pass_ver_data = pass_ver_data.clone();

        async move {
            pass_ver_data
                .clone()
                .is_loading
                .update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;

            match authorized_api.delete_account().await {
                api::api_boundary::ResultHandler::OkResult(token_resp) => {
                    ok_result_handle(pass_ver_data).await;
                }
                api::api_boundary::ResultHandler::ErrResult(err_msg) => {
                    pass_ver_data
                        .result
                        .update(|x| *x = Some(ResultHandler::ErrResult(err_msg)));
                    set_signal_state_to_init(pass_ver_data);
                }
            }
        }
    })
}

pub fn get_page_title(is_enter_current_passwrd: RwSignal<bool>) -> Signal<String> {
    Signal::derive(move || {
        if is_enter_current_passwrd.get() {
            "Current password".to_string()
        } else {
            "New password".to_string()
        }
    })
}

pub fn get_action_to_perform_title(
    is_verification_mode: RwSignal<bool>,
    action_to_perform: PassVerificationAction,
) -> Signal<String> {
    Signal::derive(move || {
        if is_verification_mode.get() {
            "Verify Password".to_string()
        } else {
            match action_to_perform {
                PassVerificationAction::ResetPassword => "Reset Password".to_string(),
                PassVerificationAction::DeleteAccount => "Delete Account".to_string(),
            }
        }
    })
}

pub fn get_action_to_perform(
    pass_verification_data: PassVerificationActionData,
    action_to_perform: PassVerificationAction,
) -> Action<(), ()> {
    if pass_verification_data.is_enter_current_passwrd.get() {
        get_verify_password_action(pass_verification_data)
    } else {
        match action_to_perform {
            PassVerificationAction::ResetPassword => reset_action(pass_verification_data),
            PassVerificationAction::DeleteAccount => delete_account_action(pass_verification_data),
        }
    }
}

fn set_signal_state_to_init(pass_verification_data: PassVerificationActionData) {
    pass_verification_data
        .current_password_signal
        .update(|x| *x = String::new());
    pass_verification_data
        .new_password_signal
        .update(|x| *x = String::new());
    pass_verification_data
        .new_password_repeat_signal
        .update(|x| *x = String::new());
    pass_verification_data
        .is_enter_current_passwrd
        .update(|x| *x = true);
    pass_verification_data
        .is_loading
        .update(|upd: &mut bool| *upd = false);
}

async fn ok_result_handle(pass_verification_data: PassVerificationActionData) {
    let action_name = match pass_verification_data.action_type {
        PassVerificationAction::ResetPassword => "Resetting password",
        PassVerificationAction::DeleteAccount => "Deleting account",
    };
    LocalStorage::delete(API_TOKEN_STORAGE_KEY.clone());
    let ok_msg = format!("{} succeeded. We'll shortly log you out...", action_name);
    pass_verification_data
        .result
        .update(|x| *x = Some(ResultHandler::OkResult(ok_msg)));
    pass_verification_data
        .is_loading
        .update(|upd: &mut bool| *upd = false);
    task::sleep(Duration::from_secs(4)).await;
    let navigate = use_navigate();
    navigate(Page::Login.path(), Default::default());
}
