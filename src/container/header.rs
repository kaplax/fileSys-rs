use crate::components::breadcrumb::{
    Breadcrumb, BreadcrumbButton, BreadcrumbDivider, BreadcrumbItem,
};
use leptos::prelude::*;

#[component]
pub fn Header(
    #[prop(into)] on_click: Callback<(usize,)>,
    #[prop(into)] path_parts: Signal<Vec<String>>,
) -> impl IntoView {

    let path_parts_len = Memo::new(move |_| path_parts.get().len());

    view! {
        <header class="border-b border-gray-200 shadow-sm py-2 px-4 sticky top-0 bg-white z-10">
            <Breadcrumb>
                <BreadcrumbItem>
                    <BreadcrumbButton on_click=move |_| on_click.run((0,))>
                    <a class="text-primary-color">"全部文件"</a>
                    </BreadcrumbButton>
                    {move || {
                        if path_parts.get().len() > 0 {
                            Some(view! { <BreadcrumbDivider /> })
                        } else {
                            None
                        }
                    }}
                </BreadcrumbItem>
                {
                    move || {
                        path_parts.get().into_iter()
                        .enumerate()
                        .map(|(index, path)| {
                            let is_last = index == path_parts_len.get().overflowing_sub(1).0;
                            view! {
                                <BreadcrumbItem>
                                    <BreadcrumbButton on_click=move |_| on_click.run((index + 1,))>
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
