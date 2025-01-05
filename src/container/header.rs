use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
};
use crate::utils::url::get_url_params;
use leptos::prelude::*;
use leptos::web_sys::window;

#[component]
pub fn Header() -> impl IntoView {
    let url = window().unwrap().location().search().unwrap();
    let url_params = get_url_params(url.as_str());
    let path = url_params.unwrap().get("path").unwrap_or_default();
    let path_parts = path
        .split("/")
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>();

    let path_parts_len = path_parts.len();
    view! {
        <header class="border-b border-gray-200 shadow-sm py-2 px-4 ">
        <Breadcrumb>
          <BreadcrumbItem>
            <BreadcrumbButton>
              <a class="text-primary-color">"全部文件"</a>
            </BreadcrumbButton>
          </BreadcrumbItem>
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
                    }
                })
                .collect::<Vec<_>>()
            }
        </Breadcrumb>
    </header>
      }
}
