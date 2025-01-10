use leptos::prelude::*;

use crate::{
    components::{list::{List, ListItem}, loading::Loading}, context::AppContext, hooks::use_file_list::{use_file_list, UseFileList}
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
                                set_path.set(format!("{}/{}", path.read(), file_clone.name));
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
