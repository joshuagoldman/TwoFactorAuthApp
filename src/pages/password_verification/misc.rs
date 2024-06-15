use std::{collections::HashMap, time::Duration};

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{Action, RwSignal, Signal, SignalGet, SignalUpdate};
use leptos_router::use_navigate;

use crate::{
    api::{authorized_api::AuthorizedApi, resulthandler::ResultHandler},
    consts::{API_TOKEN_STORAGE_KEY, NEW_PASSWORD_FIELD_STR, REPEAT_NEW_PASSWORD_FIELD_STR},
    misc::GeneralFormField,
    pages::Page,
};

use super::actions::{delete_account_action, get_verify_password_action, reset_action};

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
    pub pass_verification_map_signal: Signal<HashMap<String, GeneralFormField>>,
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

pub fn is_allowed_field(
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
            } else if !is_verification_mode.get()
                && (curr_field.name == NEW_PASSWORD_FIELD_STR.to_string()
                    || curr_field.name == REPEAT_NEW_PASSWORD_FIELD_STR.to_string())
            {
                true
            } else if is_verification_mode.get() {
                true
            } else {
                false
            }
        }
        PassVerificationAction::DeleteAccount => {
            if curr_field.name == NEW_PASSWORD_FIELD_STR.to_string()
                || curr_field.name == REPEAT_NEW_PASSWORD_FIELD_STR.to_string()
            {
                false
            } else if is_verification_mode.get() {
                true
            } else {
                false
            }
        }
    }
}

pub fn get_page_title(action_to_perform: PassVerificationAction) -> String {
    match action_to_perform {
        PassVerificationAction::ResetPassword => "Reset Password".to_string(),
        PassVerificationAction::DeleteAccount => "Delete Account".to_string(),
    }
}

pub fn get_action_to_perform_title(
    is_verification_mode: RwSignal<bool>,
    action_to_perform: PassVerificationAction,
) -> Signal<String> {
    Signal::derive(move || {
        if is_verification_mode.get() {
            "password verification...".to_string()
        } else {
            match action_to_perform {
                PassVerificationAction::ResetPassword => "Password reset...".to_string(),
                PassVerificationAction::DeleteAccount => "account deletion...".to_string(),
            }
        }
    })
}

pub fn get_action_to_perform(
    pass_verification_data: PassVerificationActionData,
    action_to_perform: PassVerificationAction,
) -> Signal<Action<(), ()>> {
    let pass_verification_data = pass_verification_data.clone();
    Signal::derive(move || {
        if pass_verification_data
            .clone()
            .is_enter_current_passwrd
            .get()
        {
            get_verify_password_action(
                pass_verification_data.clone(),
                pass_verification_data.current_password_signal.get(),
            )
        } else {
            match action_to_perform {
                PassVerificationAction::ResetPassword => reset_action(
                    pass_verification_data.clone(),
                    pass_verification_data.new_password_signal.get(),
                ),
                PassVerificationAction::DeleteAccount => {
                    delete_account_action(pass_verification_data.clone())
                }
            }
        }
    })
}

pub fn set_signal_state_to_init(pass_verification_data: PassVerificationActionData) {
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

pub fn verify_password_ok_handle(pass_verification_data: PassVerificationActionData) {
    pass_verification_data
        .is_enter_current_passwrd
        .update(|upd: &mut bool| *upd = false);
    pass_verification_data
        .is_loading
        .update(|upd: &mut bool| *upd = false);
    pass_verification_data
        .current_password_signal
        .update(|x| *x = String::new());
}

pub async fn ok_result_handle(pass_verification_data: PassVerificationActionData) {
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
