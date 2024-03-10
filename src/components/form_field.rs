use leptos::{component, view, IntoView};

use crate::misc::RegisterFormField;

#[component]
pub fn FormField(form_field: RegisterFormField) -> impl IntoView {
    view! {
        <div class="col-xs-6 col-sm-6 col-md-6">
            {move || {
                if form_field.is_password {
                    view! {
                        <input type="password" name="password_confirmation" id="password_confirmation" class="form-control input-sm" placeholder="Confirm Password"/>
                    }
                }
                else {
                    view! {
                        <input type="text" name="last_name" id="last_name" class="form-control input-sm" placeholder="Last Name"/>
                    }
                }
            }}
        </div>
    }
}
