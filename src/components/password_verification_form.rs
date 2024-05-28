use std::collections::HashMap;

use leptos::{component, view, Action, CollectView, IntoView, Show, Signal, SignalGet, View};

use crate::{
    components::form_field::FormField,
    consts::{CURRENT_PASSWORD_FIELD_STR, NEW_PASSWORD_FIELD_STR},
    misc::GeneralFormField,
    pages::password_verification::misc::{is_allowed_field, PassVerificationActionData},
};

#[component]
pub fn PasswordVerificationForm(
    pass_verification_data: PassVerificationActionData,
    action_enabled: Signal<bool>,
    action_name: Signal<String>,
    is_success: Signal<bool>,
    action_to_perform: Signal<Action<(), ()>>,
    additional_form_action: Action<(), ()>,
) -> impl IntoView {
    view! {

        <Show
            when= move  || !is_success.get()
            fallback= move || view!{
                <div></div>
            }
        >

            <AllPasswordVerificationFields pass_verification_data = pass_verification_data.clone()
                                           additional_form_action>
            </AllPasswordVerificationFields>
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
    pass_verification_data: PassVerificationActionData,
    additional_form_action: Action<(), ()>,
) -> impl IntoView {
    let allowed_fields = Signal::derive(move || {
        pass_verification_data
            .pass_verification_map_signal
            .get()
            .into_iter()
            .map(|x| x.1)
            .filter(|x| {
                is_allowed_field(
                    pass_verification_data.action_type.clone(),
                    x,
                    pass_verification_data.is_enter_current_passwrd,
                )
            })
            .map(|form_field| {
                view! {
                    <FormField form_field
                               additional_form_action/>
                }
            })
            .collect::<Vec<View>>()
            .collect_view()
    });

    view! {
        {move || {
            allowed_fields.get()
        }}
    }
}
