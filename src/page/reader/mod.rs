use leptos::prelude::*;
use leptos::Params;
use leptos_router::hooks::use_query;
use leptos_router::params::Params;

use crate::components::loading::Loading;

use crate::{
    hooks::use_file_list::{use_file_list, UseFileList},
    utils::global::ROOT_PATH,
};

#[derive(Params, PartialEq, Clone, Default)]
struct ReaderParams {
    page: Option<usize>,
    path: Option<String>,
}

#[component]
pub fn Reader() -> impl IntoView {
    let query = use_query::<ReaderParams>();
    let (path, set_path) = signal(
        query
            .read()
            .as_ref()
            .ok()
            .and_then(|query| query.path.clone())
            .unwrap_or_default(),
    );

    let UseFileList { loading, files } = use_file_list(path);
    let image_files = files;
    let (current_image, set_current_image) = signal(String::new());

    Effect::watch(
        move || image_files.get(),
        move |files, _, _| {
            if let Some(first_file) = files.first() {
                let image_path = first_file.name.clone();
                set_current_image.set(image_path);
            }
        },
        true,
    );

    view! {
        <Show when=move || !loading.get() fallback=|| view! { <Loading>"Loading..."</Loading> }>
            <img src=move || format!("{}/{}/{}", ROOT_PATH, path.get().trim_start_matches('/'), current_image.get()) />
        </Show>
    }
}
