use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::SignalUpdate;
use log::info;

use crate::{
    api::{
        api_boundary::ApiToken, authorized_api::AuthorizedApi,
        otp_authorized_api::OtpAuthorizedApi, resulthandler::ResultHandler,
        unauthorized_api::UnauthorizedApi,
    },
    consts::{API_TOKEN_OTP_KEY, API_TOKEN_STORAGE_KEY, DEFAULT_API_URL},
    misc::{go_to_page, is_move_to_default_page, ApiSignals, ApiState},
    pages::Page,
};

pub async fn check_user_logged_in(api_set_signals: ApiSignals, chosen_page: Page) {
    let otp_token = LocalStorage::get(API_TOKEN_OTP_KEY.clone());
    let user_session_token = LocalStorage::get(API_TOKEN_STORAGE_KEY.clone());

    if let Ok(token) = user_session_token {
        authorized_action(token, api_set_signals, chosen_page).await;
    } else if let Ok(token) = otp_token {
        otp_authorized_action(token, api_set_signals, chosen_page).await;
    } else {
        unauthorized_action(api_set_signals, chosen_page).await;
    }
    api_set_signals.is_resolved.update(|x| *x = true);
}

async fn otp_authorized_action(token: ApiToken, api_set_signals: ApiSignals, chosen_page: Page) {
    let api = OtpAuthorizedApi::new(&DEFAULT_API_URL, token.clone());
    match api.has_expired().await {
        ResultHandler::OkResult(true) => {
            info!("Enter OTP session token has expired");
        }
        ResultHandler::OkResult(false) => {
            api_set_signals
                .otpauth
                .update(|api_curr| *api_curr = Some(api));
            if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::Otp) {
                task::sleep(Duration::from_millis(500)).await;
                go_to_page(defaut_page);
                return;
            }
        }
        ResultHandler::ErrResult(err_message) => {
            info!("{}", err_message);
        }
    }
}

async fn authorized_action(token: ApiToken, api_set_signals: ApiSignals, chosen_page: Page) {
    let api = AuthorizedApi::new(&DEFAULT_API_URL, token);
    match api.has_expired().await {
        ResultHandler::OkResult(true) => {
            let otp_token_res = LocalStorage::get(API_TOKEN_OTP_KEY.clone());
            if let Ok(otp_token) = otp_token_res {
                otp_authorized_action(otp_token, api_set_signals, chosen_page).await;
            } else {
                info!("User session token has expired");
            }
        }
        ResultHandler::OkResult(false) => {
            api_set_signals
                .auth
                .update(|api_curr| *api_curr = Some(api));
            if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::Auth) {
                task::sleep(Duration::from_millis(500)).await;
                go_to_page(defaut_page);
                return;
            }
        }
        ResultHandler::ErrResult(err_message) => {
            info!("{}", err_message);
        }
    }
}

async fn unauthorized_action(api_set_signals: ApiSignals, chosen_page: Page) {
    let api = UnauthorizedApi::new(&DEFAULT_API_URL);
    api_set_signals
        .unauth
        .update(|api_curr| *api_curr = Some(api));
    if let Some(defaut_page) = is_move_to_default_page(chosen_page, ApiState::UnAuth) {
        task::sleep(Duration::from_secs(2)).await;
        go_to_page(defaut_page);
        return;
    }
}
