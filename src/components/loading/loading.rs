use leptos::prelude::*;

use crate::classnames;

#[component]
pub fn Loading(
    #[prop(optional, into)] visible: Signal<bool>,
    #[prop(optional)] class: String,
    children: Children,
) -> impl IntoView {
    let class_str = class.as_str();
    let class_name = classnames!["kapla-loading", class_str];
    view! {
        <div class=class_name>
            {move || {
              if visible.get() {
                Some(view! {
                  <div class="loading-text">Loading...</div>
                })
              } else {
                None
              }
            }}
            {children()}
        </div>
    }
}
