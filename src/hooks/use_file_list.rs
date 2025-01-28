use leptos::prelude::*;
use web_sys::console::log_1;

use crate::{
    api::api::{self, FileInfo, FileListReq},
    utils::request::Req,
};

#[derive(Debug)]
pub struct UseFileList {
    pub loading: ReadSignal<bool>,
    pub files: ReadSignal<Vec<FileInfo>>,
}

pub fn use_file_list(path: ReadSignal<String>) -> UseFileList {
    let (loading, set_loading) = signal::<bool>(true);
    let (files, set_files) = signal::<Vec<FileInfo>>(vec![]);

    let async_files = LocalResource::new(move || {
        let value = path.read().to_string();
        async move {
            api::get_file_list(Req {
                params: FileListReq { path: value },
                // body: None,
            })
            .await
        }
    });

    Effect::new(move || {
        if let Some(Err(e)) = async_files.read().as_deref() {
            log_1(&format!("Error: {:?}", e.list).into());
            set_loading.set(false);
        }
        if let Some(Ok(data)) = async_files.read().as_deref() {
            let mut sorted_list = data.list.clone();
            sorted_list.sort_by(|a, b| {
                alphanumeric_sort::compare_str(&a.name, &b.name)
            });
            set_files.set(sorted_list);
            set_loading.set(false);
        }
    });
    
    UseFileList { loading, files }
}
