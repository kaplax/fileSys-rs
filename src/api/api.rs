use crate::utils::request::{request, Req, RequestConfig};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileListResp {
    pub list: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct FileInfo {
    pub ext: String,
    pub is_dir: bool,
    pub last_modified_time: String,
    pub name: String,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct FileListReq {
    pub path: String,
}
pub async fn get_file_list(req: Req<FileListReq>) -> Result<FileListResp, FileListResp> {
    let res = request::<FileListReq, FileListResp>(
        "/api/fileList".to_string(),
        Some(RequestConfig {
            params: Some(req.params),
            host: None,
            method: None,
            body: None,
        }),
    )
    .await;

    match res {
        Ok(resp) => Ok(resp.data.unwrap_or_default()),
        Err(_) => Err(FileListResp::default()),
    }
}
