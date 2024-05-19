use leptos::{component, view, IntoView, Show, Signal, SignalGet, View};

use crate::misc::RegisterFormField;

pub fn get_all_error_fields(reg_form_fields: Vec<RegisterFormField>) -> Signal<Vec<String>> {
    Signal::derive(move || {
        let mut errors = reg_form_fields
            .iter()
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

        let password_fields = reg_form_fields
            .iter()
            .filter(|x| x.is_password)
            .map(|x| x.signal.get())
            .collect::<Vec<String>>();

        if password_fields.len() > 1 {
            let password_mismatch_error = !password_fields[0].is_empty()
                && !password_fields[1].is_empty()
                && password_fields[0] != password_fields[1];

            if password_mismatch_error {
                errors.push("passwords do not match".to_string());
            }
        }

        errors
    })
}

#[component]
pub fn TextFieldErrors(error_fields_signal: Signal<Vec<String>>) -> impl IntoView {
    view! {
        <Show
            when= move  || {error_fields_signal.get().len() > 0}
            fallback= move || view! { <div></div>}
        >
          <ErrorMessageRows all_error_fields = error_fields_signal.get()></ErrorMessageRows>
        </Show>
    }
}

#[component]
pub fn ErrorMessageRows(all_error_fields: Vec<String>) -> impl IntoView {
    view! {
        {
            all_error_fields
                .into_iter()
                .map(|x| {
                    view! {
                        <div style="color:red">{x}</div>
                    }.into_view()
                })
                .collect::<Vec<View>>()
        }
    }
}
