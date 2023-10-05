use crate::api::AuthorizedApi;
use api_boundary::UserInfo;
use leptos::*;

#[component]
pub fn Home<F>(
    user_info: RwSignal<Option<UserInfo>>,
    api_signal: RwSignal<Option<AuthorizedApi>>,
    token_has_been_verified: RwSignal<bool>,
    log_out_action: F,
) -> impl IntoView
where
    F: Fn() + Clone + 'static,
{
    let full_log_out_action = create_action(move |_| {
        let log_out_action = log_out_action.clone();
        async move {
            api_signal.update(|a| *a = None);
            user_info.update(|i| *i = None);
            log_out_action();
        }
    });

    view! {
      <Show
        when = move || api_signal.get().is_some()
        fallback = move || view!{
          <Show
            when = move || token_has_been_verified.get() && api_signal.get().is_none()
            fallback = move || view!{
              <div/>
            }
            >
              {move || {
                  full_log_out_action.dispatch(());
                  view!{
                    <div/>
                  }
                }
              }
          </Show>
          <div>
              {"..."}
          </div>
        }
        >
        <Show
          when = move || user_info.get().is_some()
          fallback = move || view!{
            <div style = "color:white;">
                {"Fetching data..."}
            </div>
          }
          >
            <p style = "color:white;">"Hej och välkommen, "{ user_info.get().unwrap().full_name }"."</p>
        </Show>
      </Show>
    }
}
