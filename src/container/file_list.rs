use std::collections::HashMap;

use leptos::prelude::*;
use leptos_router::{hooks::use_navigate, NavigateOptions};
use leptos_use::use_window;

use crate::{
    api::api::FileInfo,
    components::{
        list::{List, ListItem},
        loading::Loading,
    },
    context::AppContext,
    hooks::use_file_list::{use_file_list, UseFileList},
    utils::url::set_url_params,
};

#[component]
pub fn FileList() -> impl IntoView {
    let window = use_window();
    let urlSearchParams = window.as_ref().unwrap().location().search().unwrap();
    let AppContext { path, set_path } = use_context::<AppContext>().unwrap();

    let UseFileList { loading, files } = use_file_list(path);

    let navigate = use_navigate();

    let handle_click = move |file: &FileInfo, index: usize, urlSearchParams: &str, navigate: &dyn Fn(&str, NavigateOptions)| {
        if file.is_dir {
            let new_path = format!("{}/{}", path.read(), file.name);
            set_path.set(new_path.clone());
            set_url_params(&HashMap::from([("path".to_string(), new_path)]));
        } else {
            navigate(&format!("/reader{}&page={}", urlSearchParams, index), Default::default());
        }
    };

    view! {
        <div>
          <Loading visible=loading>
            <List>
                {
                  move || {
                      files.get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, file)| {
                          let file_clone = file.clone();
                          let urlSearchParams = urlSearchParams.clone();
                          let navigate = navigate.clone();
                          view! {
                            <ListItem on_click=move |_| {
                              handle_click(&file_clone, index, &urlSearchParams, &navigate);
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
