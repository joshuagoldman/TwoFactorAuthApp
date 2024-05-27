use std::collections::HashMap;

use leptos::{component, view, Action, IntoView, Show, Signal, SignalGet};

use crate::{
    components::form_field::FormField,
    consts::{CURRENT_PASSWORD_FIELD_STR, NEW_PASSWORD_FIELD_STR},
    misc::GeneralFormField,
};

#[component]
pub fn PasswordVerificationForm(
    login_fields_map_signal: Signal<HashMap<String, GeneralFormField>>,
    action_enabled: Signal<bool>,
    action_name: Signal<String>,
    is_success: Signal<bool>,
    action_to_perform: Signal<Action<(), ()>>,
) -> impl IntoView {
    view! {

        <Show
            when= move  || is_success.get()
            fallback= move || view!{
                <div></div>
            }
        >

            <AllPasswordVerificationFields login_fields_map_signal></AllPasswordVerificationFields>
            <div class="d-flex justify-content-center mx-4 mb-3 mb-lg-4">
                <button  type="button" data-mdb-button-init data-mdb-ripple-init class="btn btn-primary btn-lg"
                            disabled= {move || !action_enabled.get()}
                            on:click= {move |_| action_to_perform.get().dispatch(())}
                >{move || action_name.get()}</button>
            </div>
        </Show>
    }
}

#[component]
pub fn AllPasswordVerificationFields(
    login_fields_map_signal: Signal<HashMap<String, GeneralFormField>>,
) -> impl IntoView {
    let current_pass_field_signal = Signal::derive(move || {
        login_fields_map_signal
            .get()
            .get(&CURRENT_PASSWORD_FIELD_STR.clone())
            .unwrap()
            .clone()
    });
    let new_password_field_signal = Signal::derive(move || {
        login_fields_map_signal
            .get()
            .get(&NEW_PASSWORD_FIELD_STR.clone())
            .unwrap()
            .clone()
    });

    view! {
        <FormField form_field= current_pass_field_signal.get()/>
        <FormField form_field= new_password_field_signal.get()/>
    }
}
