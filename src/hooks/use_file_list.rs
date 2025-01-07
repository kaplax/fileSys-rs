use leptos::prelude::*;
use web_sys::console::log_1;

use crate::{
    api::api::{get_file_list, FileInfo, FileListReq},
    utils::request::Req,
};

#[derive(Debug)]
pub struct UseFileList {
    pub loading: ReadSignal<bool>,
    pub files: ReadSignal<Vec<FileInfo>>,
}

pub fn use_file_list(path: String) -> UseFileList {
    let (loading, set_loading) = signal::<bool>(true);
    let (files, set_files) = signal::<Vec<FileInfo>>(vec![]);
    let async_files = LocalResource::new(move || {
        let value = path.clone();
        async move {
            get_file_list(Req {
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
            set_files.set(data.list.clone());
            set_loading.set(false);
        }
    });
    UseFileList { loading, files }
}
