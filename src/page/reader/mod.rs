use leptos::prelude::*;
use leptos_router::hooks::use_query;
use leptos::Params;
use leptos_router::params::Params;


use crate::{
    components::{
        loading::Loading,
    },
};

use crate::{hooks::use_file_list::{use_file_list, UseFileList}, utils::global::ROOT_PATH};

#[derive(Params, PartialEq, Clone, Default)]
struct ReaderParams {
    page: Option<usize>,
    path: Option<String>,
}

#[component]
pub fn Reader() -> impl IntoView {
    let query = use_query::<ReaderParams>();
    let (path, set_path) = signal(query.read().as_ref().ok().and_then(|query| query.path.clone()).unwrap_or_default());
    let UseFileList { loading, files } = use_file_list(path);
    let image_files = files;
    let (current_image, set_current_image) = signal("");

    view! {
        <div> 
        "reader" 
        <div> {path} </div>
        </div>
    }
}

