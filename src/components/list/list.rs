use leptos::{ev::MouseEvent, prelude::*};

use crate::{classnames, utils::dom::mount_style};

use crate::utils::callback::BoxOneCallback;

#[component]
pub fn List(#[prop(optional)] class: String, children: Children) -> impl IntoView {
    mount_style("list", include_str!("list.css"));
    let class_str = class.as_str();
    let merged_class = classnames!["kapla-list", class_str];
    view! {
        <div class=merged_class>
            <div class="kapla-list-body">
                <div class="kapla-list-body-inner">
                    {children()}
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn ListItem(
    #[prop(optional)] class: String,
    #[prop(optional, into)] on_click: Option<BoxOneCallback<MouseEvent>>,
    children: Children
) -> impl IntoView {
    let class_str = class.as_str();
    let merged_class = classnames!["kapla-list-item", class_str];

    let on_click = move |e| {
    
        let Some(on_click) = on_click.as_ref() else {
            return;
        };
        on_click(e);
    };

    view! {
        <a class=merged_class on:click=on_click>
            <div class="kapla-list-item-content">
                <div class="kapla-list-item-content-main">
                {children()}
                </div>
                <div class="kapla-list-item-content-arraw">
                    <svg 
                        width="1em" height="1em" viewBox="0 0 48 48" xmlns="http://www.w3.org/2000/svg" 
                        xmlns:xlink="http://www.w3.org/1999/xlink" class="antd-mobile-icon" style="vertical-align: -0.125em;"
                    >
                        <g id="RightOutline-RightOutline" stroke="none" stroke-width="1" fill="none" fill-rule="evenodd">
                            <g id="RightOutline-RightOutlined"> 
                                <rect id="RightOutline-矩形" fill="#FFFFFF" opacity="0" x="0" y="0" width="48" height="48"></rect>
                                <path d="M17.3947957,5.11219264 L35.5767382,22.6612572 L35.5767382,22.6612572 C36.1304785,23.2125856 36.1630514,24.0863155 35.6744571,24.6755735 L35.5767382,24.7825775 L17.3956061,42.8834676 C17.320643,42.9580998 17.2191697,43 17.1133896,43 L13.9866673,43 C13.7657534,43 13.5866673,42.8209139 13.5866673,42.6 C13.5866673,42.4936115 13.6290496,42.391606 13.7044413,42.316542 L32.3201933,23.7816937 L32.3201933,23.7816937 L13.7237117,5.6866816 C13.5653818,5.53262122 13.5619207,5.27937888 13.7159811,5.121049 C13.7912854,5.04365775 13.8946805,5 14.0026627,5 L17.1170064,5 C17.2206403,5 17.3202292,5.04022164 17.3947957,5.11219264 Z" id="RightOutline-right" fill="currentColor" fill-rule="nonzero"></path>
                            </g>
                        </g>
                    </svg>
                </div>
            </div>
        </a>
    }
}
