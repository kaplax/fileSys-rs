use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
};
use crate::components::config_provider::ConfigProvider;
use crate::utils::url::get_url_params;
use leptos::prelude::*;
use leptos::web_sys::window;

#[component]
pub fn App() -> impl IntoView {
    let url = window().unwrap().location().search().unwrap();
    let url_params = get_url_params(url.as_str());
    let path = url_params
        .unwrap()
        .get("path")
        .unwrap_or_default()
        .to_string();
    let path_parts = path
        .split("/")
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>();

    let path_parts_len = path_parts.len();
    view! {
        <ConfigProvider>
            <div>
                <Breadcrumb>
                    {
                    path_parts.into_iter()
                    .enumerate()
                    .map(|(index, path)| {
                        let is_last = index == path_parts_len - 1;
                        view! {
                        <BreadcrumbItem>
                            <BreadcrumbButton >
                                {path}
                            </BreadcrumbButton>
                        </BreadcrumbItem>
                        {if !is_last {
                            Some(view! { <BreadcrumbDivider /> })
                        } else {
                            None
                        }}
                    }})
                    .collect::<Vec<_>>()
                }
                </Breadcrumb>
            </div>
        </ConfigProvider>
    }
}
