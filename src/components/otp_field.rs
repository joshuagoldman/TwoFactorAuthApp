use leptos::{component, ev, event_target_value, view, Action, IntoView, RwSignal, SignalUpdate};
use serde_json::from_str;
use wasm_bindgen::JsCast;
use web_sys::{Document, HtmlElement, Window};

#[component]
pub fn OtpField(
    otp_field_val: RwSignal<String>,
    additional_form_action: Action<(), ()>,
    otp_field_num: usize,
) -> impl IntoView {
    view! {
        <div>
            <input type="text" class="form-control"
                   id={move || format!("otpfield{}", otp_field_num)}
                    maxlength="1"
                    on:keyup = move |ev: ev::KeyboardEvent| {
                        match &*ev.key() {
                            "enter" => {
                            }
                            _=> {
                                let val = event_target_value(&ev);
                                additional_form_action.dispatch(());
                                otp_field_val.update(|p|*p = val.clone());

                                if let Ok(_) = from_str::<usize>(&val) {
                                    focus_on_next_otp_field(otp_field_num);
                                }
                            }
                    }}
            />
        </div>
    }
}

fn focus_on_next_otp_field(curr_id: usize) {
    if curr_id > 5 {
        return;
    }

    let window: Window;
    if let Some(window_found) = web_sys::window() {
        window = window_found;
    } else {
        return;
    }

    let doc: Document;
    if let Some(document_found) = window.document() {
        doc = document_found;
    } else {
        return;
    }

    if let Ok(Some(el)) = doc.query_selector(format!("#otpfield{}", curr_id + 1).as_str()) {
        if let Some(html_el) = el.dyn_ref::<HtmlElement>() {
            let _ = html_el.focus().unwrap_or(());
        }
    }
}
