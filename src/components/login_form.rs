use std::collections::HashMap;

use crate::{
    api::api_boundary::Credentials,
    components::form_field::FormField,
    consts::{PASSWORD_FIELD_STR, USER_NAME_FIELD_STR},
    misc::GeneralFormField,
};
use leptos::*;

#[component]
pub fn LoginForm(
    login_fields_map_signal: Signal<HashMap<String, GeneralFormField>>,
    credentials: Signal<Option<Credentials>>,
    on_login: Action<Credentials, ()>,
) -> impl IntoView {
    view! {

        <AllLoginFields login_fields_map_signal></AllLoginFields>
        <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
            <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                        disabled= {move || credentials.get().is_none()}
                        on:click= {move |_| on_login.dispatch(credentials.get().unwrap())}
            >Login</button>
        </div>
    }
}

#[component]
pub fn AllLoginFields(
    login_fields_map_signal: Signal<HashMap<String, GeneralFormField>>,
) -> impl IntoView {
    let username_field_signal = Signal::derive(move || {
        login_fields_map_signal
            .get()
            .get(&USER_NAME_FIELD_STR.clone())
            .unwrap()
            .clone()
    });
    let password_field_signal = Signal::derive(move || {
        login_fields_map_signal
            .get()
            .get(&PASSWORD_FIELD_STR.clone())
            .unwrap()
            .clone()
    });

    view! {
        <FormField form_field= username_field_signal.get()/>
        <FormField form_field= password_field_signal.get()/>
    }
}
