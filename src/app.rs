use crate::api::api::{get_file_list, FileInfo, FileListReq};
use crate::container::header::Header;
use crate::components::config_provider::ConfigProvider;
use crate::utils::request::Req;
use leptos::prelude::*;
use web_sys::console::log_1;


#[component]
pub fn App() -> impl IntoView {
    let async_files = LocalResource::new(|| async move {
        get_file_list(Req {
            params: FileListReq {
                path: "/".into(),
            },
            body: None,
        })
        .await
    });

    let (files, set_files) = signal::<Vec<FileInfo>>(vec![]);
    Effect::new(move || {
        if let Some(Err(e)) = async_files.read().as_deref() {
            log_1(&format!("Error: {:?}", e.list).into());
        }
        if let Some(Ok(data)) = async_files.read().as_deref() {
            set_files.set(data.list.clone());
        }
    });


    view! {
        <ConfigProvider>
            <main>
                <Header />
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
            </main>
        </ConfigProvider>
    }
}
