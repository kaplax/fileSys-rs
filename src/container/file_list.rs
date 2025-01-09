use leptos::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console::log_1;

use crate::{
    components::loading::Loading,
    components::list::{List, ListItem},
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
            <List>
                {
                  move || {
                      files.get()
                        .into_iter()
                        .map(|file| {
                          let file_clone = file.clone();
                          view! {
                            <ListItem on_click=move |_| {
                              log_1(&JsValue::from_str(&file_clone.name));
                            }>
                                <div>{file.name}</div>
                            </ListItem>
                          }
                        })
                        .collect::<Vec<_>>()
                  }
                }
              </List>
          </Loading>
        </div>
    }
}
