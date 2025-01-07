use leptos::prelude::*;

use crate::{
    components::loading::Loading,
    hooks::use_file_list::{use_file_list, UseFileList},
};

#[component]
pub fn FileList(
  path: String,
) -> impl IntoView {

    let UseFileList {
        loading,
        files,
    } = use_file_list(path);


    view! {
        <div>
          <Loading visible=loading>
                {
                    move || {
                        files.get()
                            .into_iter()
                            .map(|file| view! {
                                <div>{file.name}</div>
                            })
                            .collect::<Vec<_>>()
                    }
                }
          </Loading>
        </div>
    }
}
