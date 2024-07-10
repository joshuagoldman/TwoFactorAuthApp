use leptos::{component, ev, event_target_value, view, Action, IntoView, RwSignal, SignalUpdate};
#[component]
pub fn OtpField(
    otp_field_val: RwSignal<String>,
    additional_form_action: Action<(), ()>,
) -> impl IntoView {
    view! {
        <div>
            <input type="text" class="form-control"
                    maxlength="1"
                    on:keyup = move |ev: ev::KeyboardEvent| {
                        match &*ev.key() {
                            "enter" => {
                            }
                            _=> {
                                let val = event_target_value(&ev);
                                additional_form_action.dispatch(());
                                otp_field_val.update(|p|*p = val);
                            }
                    }}
            />
        </div>
    }
}
