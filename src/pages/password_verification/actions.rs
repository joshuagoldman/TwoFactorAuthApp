use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{create_action, Action, SignalGet, SignalUpdate};

use crate::{
    api::{self, api_boundary::ResultHandler},
    consts::API_TOKEN_STORAGE_KEY,
};

use super::misc::{ok_result_handle, set_signal_state_to_init, PassVerificationActionData};

pub fn get_verify_password_action(pass_ver_data: PassVerificationActionData) -> Action<(), ()> {
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
