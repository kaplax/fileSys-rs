use std::collections::HashMap;
use leptos::logging::log;

use leptos::prelude::*;

use crate::{
    components::{list::{List, ListItem}, loading::Loading}, context::AppContext, hooks::use_file_list::{use_file_list, UseFileList}, utils::url::set_url_params
};

#[component]
pub fn FileList(
) -> impl IntoView {
    let AppContext { path, set_path } = use_context::<AppContext>().unwrap();

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
                              if file_clone.is_dir {
                                let new_path = format!("{}/{}", path.read(), file_clone.name);
                                log!("new_path2: {:?}", new_path);
                                set_path.set(new_path.clone());
                                set_url_params(&HashMap::from([("path".to_string(), new_path)]));
                              }
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
