use leptos::prelude::*;

use crate::classnames;

#[component]
pub fn Breadcrumb(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    let combined_class = format!("kapla-breadcrumb__list {}", class);
    view! {
        <nav>
            <ol role="list" class=combined_class>
                {children()}
            </ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = format!("kapla-breadcrumb-item {}", class);
    view! {
        <li class=merged_class>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbButton(
    #[prop(optional)] class: String,
    #[prop(optional, into)] current: Signal<bool>,
    children: Children,
) -> impl IntoView {
    let class_str = class.as_str();
    let button_class = classnames![
        "kapla-breadcrumb-button",
        ("kapla-breadcrumb-button--current", current.get()),
        class_str
    ];
    view! {
        <button class=button_class>
            {children()}
        </button>
    }
}
