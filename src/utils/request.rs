use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{RequestInit, Request, Response};


#[derive(Serialize, Deserialize)]
pub struct Resp<T> {
    pub data: T,
    pub message: String,
    pub success: bool,
}

#[wasm_bindgen]
pub async fn run(path: String) -> Result<JsValue, JsValue> {
    let opts = RequestInit::new();
    opts.set_method("GET");

    let url = format!("http://192.168.1.91:3002/api/fileList?path={}", path);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}

/// 请求工具, 用于发送请求，使用方式类似如下 TS 代码
/// ```ts
/// function request<P, R>(path: string, params: P): Promise<R>;
/// const data = await request<{a: number}, {data: string}>('msa-group/apig-website-oss', {params: {a: 1}});
/// ```
pub async fn request<P, R>(path: String, params: P) -> Result<R, String> {
  todo!();
}