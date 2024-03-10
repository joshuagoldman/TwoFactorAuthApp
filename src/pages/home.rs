use leptos::{component, create_action, view, IntoView, RwSignal, Show, SignalGet};

use crate::api::{api_boundary::ProfileInfo, AuthorizedApi};

#[component]
pub fn Home(
    authorized_api: AuthorizedApi,
    profile_info: RwSignal<Option<ProfileInfo>>,
) -> impl IntoView {
    view! {
        {move || {
                match profile_info.get() {
                    Some(profile_info_some) => view! {
                        <div style="color:red;">{move || {
                                format!("Welcome mynigga, {}", profile_info_some.name)
                            }}
                        </div>
                    },
                    None => view! {
                        <div style="color:red;">{"Getting profile info"}</div>
                    }
                }
            }
        }
    }
}
