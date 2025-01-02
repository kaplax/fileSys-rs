use leptos::prelude::*;

use crate::utils::dom::mount_style;

#[component]
pub fn ConfigProvider(
  children: Children
) -> impl IntoView {
  mount_style("config-provider", include_str!("./config-provider.css"));
  view! {
    <div>
      {children()}
    </div>
  }
}
