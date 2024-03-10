use std::time::Duration;

use async_std::task;
use gloo_storage::{LocalStorage, Storage};
use leptos::{
    component, create_action, create_signal, view, IntoView, Show, SignalGet, SignalUpdate,
};
use uuid::Uuid;

use crate::{
    api::{self, api_boundary::Credentials, AuthorizedApi, UnauthorizedApi},
    components::login_form::*,
    consts::API_TOKEN_STORAGE_KEY,
    misc,
};

use crate::consts::DEFAULT_API_URL;

use super::Page;

#[component]
pub fn Login(unauth_api: UnauthorizedApi) -> impl IntoView {
    let (login_error, login_error_set) = create_signal(None::<String>);
    let (wait_for_response, set_wait_for_response) = create_signal(false);

    let login_action = create_action(move |login_data: &Credentials| {
        let login_data = login_data.clone();
        async move {
            set_wait_for_response.update(|upd: &mut bool| *upd = true);
            task::sleep(Duration::from_secs(2)).await;
            let auth_api = AuthorizedApi::new(
                &DEFAULT_API_URL,
                api::api_boundary::ApiToken {
                    token: Uuid::new_v4().to_string(),
                },
            );
            set_wait_for_response.update(|upd: &mut bool| *upd = false);
            LocalStorage::set(API_TOKEN_STORAGE_KEY.clone(), auth_api);
            misc::go_to_page(Page::Home)
            //let result = unauthApi.login(&login_data).await;
        }
    });

    view! {
        <Show
            when=move || wait_for_response.get()
            fallback = || view! { <div style="color:red">{"Loading"}</div>}
        >
            <LoginForm login_action/>
        </Show>
    }
}
