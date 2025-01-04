use crate::container::header::Header;
use crate::{components::config_provider::ConfigProvider, utils::request::Resp};
use leptos::prelude::*;
use leptos::server_fn::request::browser::Request;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileList {
    list: Vec<File>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct File {
    ext: String,
    is_dir: bool,
    last_modified_time: String,
    name: String,
    size: i64,
}

#[component]
pub fn App() -> impl IntoView {
    let b = LocalResource::new(|| async move {
        let res = Request::get("http://192.168.1.91:3002/api/fileList?path=/".into())
            .send()
            .await;
        res.unwrap().json::<Resp<FileList>>().await
    });

    // let data = LocalResource::new(|| async move { run("/".into()).await });
    let (files, set_files) = signal::<Vec<File>>(vec![]);
    Effect::new(move || {
        if let Some(Ok(data)) = b.read().as_deref() {
            set_files.set(data.data.list.clone());
        }

        // match data {
        //     Some(data) => match data {
        //         Ok(data) => {
        //             let res: Resp<FileList> = serde_wasm_bindgen::from_value(data.clone()).unwrap();
        //             set_files.set(res.data.list);
        //         }
        //         Err(_) => (),
        //     },
        //     None => (),
        // }
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
