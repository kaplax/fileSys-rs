use std::collections::HashMap;
use leptos::web_sys::{UrlSearchParams, window};
use wasm_bindgen::JsValue;
use leptos::logging::log;

pub fn get_url_params(params: &str) -> Result<UrlSearchParams, JsValue> {
    UrlSearchParams::new_with_str(params)
}

pub fn set_url_params(params: &HashMap<String, String>) {
    let window = window().unwrap();
    let search_params = get_url_params(window.location().search().unwrap_or_default().as_str());
    match search_params {
        Ok(search_params) => {
            for (key, value) in params {
                let key = key.as_str();
                let value = value.as_str();
                if let Some(_) = params.get(key) {
                    search_params.set(key, value);
                }
            }
            let pathname = window.location().pathname().unwrap_or_default();
            let pathname_with_params = format!("{}?{}", pathname, search_params.to_string());
            let _ = window.history().unwrap().push_state_with_url(&JsValue::null(), "", Some(&pathname_with_params.as_str()));
        }
        Err(e) => {
            log!("Error: {:?}", e);
        }
    }
}
