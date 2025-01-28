use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::utils::callback::BoxOneCallback;

#[component]
pub fn Button(
  #[prop(optional)] class: String, 
  #[prop(optional, into)] on_click: Option<BoxOneCallback<MouseEvent>>,
  children: Children
) -> impl IntoView {


    let on_click = move |e| {
        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click(e);
    };

    view! {
        <button class=class on:click=on_click>
            {children()}
        </button>
    }
}