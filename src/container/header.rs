use std::collections::HashMap;

use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
};
use crate::context::AppContext;
use crate::utils::url::set_url_params;
use leptos::logging::log;
use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    let AppContext { path, set_path } = use_context::<AppContext>().unwrap();
    let path_parts = Memo::new(move |_| {
        path.read()
            .split("/")
            .filter(|part| !part.is_empty())
            .map(|part| part.to_string())
            .collect::<Vec<String>>()
    });

    let path_parts_len = Memo::new(move |_| path_parts.get().len());

    let on_click = move |index: usize| {
        // runtime error why
        // let breadcrumbs = path_parts.read();
        // if breadcrumbs.len() > 0 {
        //     let new_path = format!("/{}", breadcrumbs[..index].join("/"));
        //     log!("new_path1: {:?}", new_path);
        //     set_path.set(new_path.clone());
        //     set_url_params(&HashMap::from([("path".to_string(), new_path)]));
        // }

        // success why
        let new_path = {
            let breadcrumbs = path_parts.read();
            format!("/{}", breadcrumbs[..index].join("/"))
        };
        set_path.set(new_path.clone());
        set_url_params(&HashMap::from([("path".to_string(), new_path)]));
    };

    view! {
        <header class="border-b border-gray-200 shadow-sm py-2 px-4 ">
        <Breadcrumb>
          <BreadcrumbItem>
            <BreadcrumbButton>
              <a class="text-primary-color">"全部文件"</a>
            </BreadcrumbButton>
          </BreadcrumbItem>
            {
                move || {
                    path_parts.get().into_iter()
                    .enumerate()
                    .map(|(index, path)| {
                        let is_last = index == path_parts_len.get().overflowing_sub(1).0;
                        view! {
                            <BreadcrumbItem>
                                <BreadcrumbButton on_click=move |_| on_click(index + 1)>
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
            }
        </Breadcrumb>
    </header>
      }
}
