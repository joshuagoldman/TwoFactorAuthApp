use leptos::{component, ev, event_target_value, view, IntoView, RwSignal, SignalUpdate};

#[component]
pub fn OtpField(otp_field_val: RwSignal<String>) -> impl IntoView {
    view! {
        <div class="d-flex flex-row align-items-center">
            <div data-mdb-input-init class="form-outline flex-fill mb-0">
                <input type="text" class="form-control"
                        maxlength="1"
                        on:keyup = move |ev: ev::KeyboardEvent| {
                            match &*ev.key() {
                                "enter" => {
                                }
                                _=> {
                                    let val = event_target_value(&ev);
                                    otp_field_val.update(|p|*p = val);
                                }
                        }}
                />
            </div>
        </div>
    }
}
