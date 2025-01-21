use std::collections::HashMap;

use crate::components::config_provider::ConfigProvider;
use crate::container::file_list::FileList;
use crate::container::header::Header;
use crate::context::AppContext;
use crate::utils::dom::mount_style;
use crate::utils::url::{get_url_params, set_url_params};
use leptos::prelude::*;
use leptos_use::use_window;

#[component]
pub fn Main() -> impl IntoView {
    mount_style("reset", include_str!("../../styles/reset.css"));
    let window = use_window();
    let url = window.as_ref().unwrap().location().search().unwrap();
    let url_params = get_url_params(url.as_str());
    let path = url_params.unwrap().get("path").unwrap_or_default();
    let (path, set_path) = signal(path);
    provide_context(AppContext { path, set_path });

    let path_parts = Memo::new(move |_| {
        path.get()
            .split("/")
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect::<Vec<String>>()
    });

    let handleBreadcrumbClick = move |index: usize| {
        let new_path = {
            let breadcrumbs = path_parts.read();
            format!("/{}", breadcrumbs[..index].join("/"))
        };
        set_path.set(new_path.clone());
        set_url_params(&HashMap::from([("path".to_string(), new_path)]));
    };

    let on_back = move |_| {
        let breadcrumbs = path_parts.get();  // 先获取克隆值，避免借用冲突
        if !breadcrumbs.is_empty() {
            handleBreadcrumbClick(breadcrumbs.len() - 1);
        }
    };

    view! {
        <ConfigProvider>
            <Header path_parts=path_parts on_click=handleBreadcrumbClick />
            <main>
                <FileList />
            </main>
            {
                move || {
                    if path_parts.read().len() > 0 {
                        Some(view! {
                            <footer
                                class="fixed bottom-0 flex justify-center items-center w-full h-10 bg-white"
                                on:click=on_back
                            >
                                "Back"
                            </footer>
                        })
                    } else {
                        None
                    }
                }
            }
        </ConfigProvider>
    }
}
