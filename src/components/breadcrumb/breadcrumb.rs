use leptos::prelude::*;

use crate::{classnames, utils::dom::mount_style};

#[component]
pub fn Breadcrumb(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    let combined_class = format!("kapla-breadcrumb__list {}", class);
    mount_style("breadcrumb", include_str!("breadcrumb.css"));
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

#[component]
pub fn BreadcrumbDivider(#[prop(optional)] class: String) -> impl IntoView {
    let class_str = class.as_str();
    view! {
        <li class=classnames!["kapla-breadcrumb-divider", class_str] aria-hidden="true">
            <svg
                fill="currentColor"
                aria-hidden="true"
                width="1em"
                height="1em"
                viewBox="0 0 20 20"
            >
                <path
                    d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z"
                    fill="currentColor"
                ></path>
            </svg>
        </li>
    }
}