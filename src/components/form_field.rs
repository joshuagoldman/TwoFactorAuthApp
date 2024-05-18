use std::collections::HashMap;

use leptos::{
    component, ev, event_target_value, view, IntoView, Signal, SignalGet, SignalUpdate, View,
};

use crate::{
    consts::{
        EMAIL_FIELD_STR, FIRST_NAME_FIELD_STR, LAST_NAME_FIELD_STR, PASSWORD_FIELD_STR,
        REPEAT_PASSWORD_FIELD_STR, USER_NAME_FIELD_STR,
    },
    misc::RegisterFormField,
};

#[component]
pub fn AllRegisterFields(
    form_fields_map: Signal<HashMap<String, RegisterFormField>>,
) -> impl IntoView {
    view! {
        <TextFields form_fields_map></TextFields>
        <PasswordFields form_fields_map></PasswordFields>
        <EmailField form_fields_map></EmailField>
    }
}

#[component]
pub fn TextFields(form_fields_map: Signal<HashMap<String, RegisterFormField>>) -> impl IntoView {
    let text_fields_signal = Signal::derive(move || {
        vec![
            form_fields_map
                .get()
                .get(&FIRST_NAME_FIELD_STR.clone())
                .unwrap()
                .clone(),
            form_fields_map
                .get()
                .get(&LAST_NAME_FIELD_STR.clone())
                .unwrap()
                .clone(),
            form_fields_map
                .get()
                .get(&USER_NAME_FIELD_STR.clone())
                .unwrap()
                .clone(),
        ]
    });

    view! {
        {move || {
            text_fields_signal.get()
                .into_iter()
                .map(|form_field| {
                    view! {
                        <FormField form_field/>
                    }
                })
                .collect::<Vec<View>>()
            }
        }
    }
}

#[component]
pub fn PasswordFields(
    form_fields_map: Signal<HashMap<String, RegisterFormField>>,
) -> impl IntoView {
    let password_fields_signal = Signal::derive(move || {
        vec![
            form_fields_map
                .get()
                .get(&PASSWORD_FIELD_STR.clone())
                .unwrap()
                .clone(),
            form_fields_map
                .get()
                .get(&REPEAT_PASSWORD_FIELD_STR.clone())
                .unwrap()
                .clone(),
        ]
    });

    view! {
        {move || {
            password_fields_signal.get()
                .into_iter()
                .map(|form_field| {
                    view! {
                        <FormField form_field/>
                    }
                })
                .collect::<Vec<View>>()
            }
        }
    }
}

#[component]
pub fn EmailField(form_fields_map: Signal<HashMap<String, RegisterFormField>>) -> impl IntoView {
    let email_field_signal = Signal::derive(move || {
        form_fields_map
            .get()
            .get(&EMAIL_FIELD_STR.clone())
            .unwrap()
            .clone()
    });

    view! {
        <FormField form_field= email_field_signal.get()/>
    }
}

#[component]
pub fn FormField(form_field: RegisterFormField) -> impl IntoView {
    let form_type = if form_field.is_password {
        "password"
    } else {
        "text"
    };
    let style = Signal::derive(move || match form_field.requirement.clone() {
        Some(rqrmnt) => {
            if !form_field.signal.get().is_empty() && rqrmnt.func.clone()(&form_field.signal.get())
            {
                "color:black;"
            } else {
                "color:red"
            }
        }
        None => {
            if !form_field.signal.get().is_empty() {
                "color:black;"
            } else {
                "color:red"
            }
        }
    });

    view! {
        <div class="d-flex flex-row align-items-center mb-4">
            <i class="fas fa-envelope fa-lg me-3 fa-fw"></i>
            <div data-mdb-input-init class="form-outline flex-fill mb-0">
            <input type={form_type} class="form-control"
                    style={move || style.get()}
                    on:keyup = move |ev: ev::KeyboardEvent| {
                        match &*ev.key() {
                            "enter" => {
                            }
                            _=> {
                                let val = event_target_value(&ev);
                                form_field.signal.update(|p|*p = val);
                            }
                    }}
            />
            <label class="form-label" for="form3Example3c">{form_field.name}</label>
            </div>
        </div>
    }
}
