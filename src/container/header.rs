use leptos::prelude::*;
use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
};
use crate::context::AppContext;

#[component]
pub fn Header() -> impl IntoView {
    let AppContext { path , set_path: _} = use_context::<AppContext>().unwrap();
    let path_parts = Memo::new(move |_| {
        path.read()
        .split("/")
        .filter(|part| !part.is_empty())
        .map(|part| part.to_string())
        .collect::<Vec<String>>()
    });

    let path_parts_len = path_parts.get().len();

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
                        let is_last = index == path_parts_len.overflowing_sub(1).0;
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
            }
        </Breadcrumb>
    </header>
      }
}
