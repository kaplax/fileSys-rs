use leptos::web_sys::UrlSearchParams;
use wasm_bindgen::JsValue;

pub fn get_url_params(params: &str) -> Result<UrlSearchParams, JsValue> {
    UrlSearchParams::new_with_str(params)
}
