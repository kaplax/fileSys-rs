use gloo_net::http::{Method, Request};
use serde::{Deserialize, Serialize};
use web_sys::console::log_1;

#[derive(Serialize, Deserialize)]
pub struct Res<T> {
    pub data: Option<T>,
    pub message: String,
    pub success: bool,
}

pub struct Req<T>
where
    T: Serialize,
{
    pub params: T,
    // pub body: Option<T>,
}

#[derive(Debug)]
pub struct RequestConfig<T>
where
    T: Serialize,
{
    pub host: Option<String>,
    pub method: Option<Method>,
    pub params: Option<T>,
    pub body: Option<T>,
}

impl<T> Default for RequestConfig<T>
where
    T: Serialize,
{
    fn default() -> Self {
        Self {
            host: None,
            method: None,
            params: None,
            body: None,
        }
    }
}

impl<T> Res<T> {
    pub fn default() -> Self {
        Self {
            data: None,
            message: "".to_string(),
            success: false,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DefaultBody;

pub async fn request<P, R>(url: String, config: Option<RequestConfig<P>>) -> Result<Res<R>, Res<R>>
where
    P: Serialize + Default,
    R: for<'a> Deserialize<'a>,
{
    let conf = config.unwrap_or_default();
    let host = conf.host.unwrap_or("http://192.168.1.91:3002".to_string());
    let query_string = struct_to_query_string(conf.params);
    let method = conf.method.unwrap_or(Method::GET);

    let res = match method {
        Method::GET => {
            Request::get(&format!("{}{}?{}", host, url, query_string))
                .header("Content-Type", "application/json")
                .send()
                .await
        }
        _ => {
            let body = conf.body.unwrap_or_default();
            let body_value = serde_wasm_bindgen::to_value(&body).unwrap();
            let req_builder = Request::post(&format!("{}{}?{}", host, url, query_string))
                .header("Content-Type", "application/json")
                .body(body_value);
            req_builder.unwrap().send().await
        }
    };
    match res {
        Ok(res) => match res.json::<Res<R>>().await {
            Ok(resp) => Ok(resp),
            Err(_) => Err(Res::default()),
        },
        Err(_) => Err(Res::default()),
    }
}

pub fn struct_to_query_string<T: Serialize>(data: Option<T>) -> String {
    match data {
        Some(data) => match serde_qs::to_string(&data) {
            Ok(data) => data,
            Err(e) => {
                log_1(&format!("Error: {:?}", e).into());
                "".to_string()
            }
        },
        None => "".to_string(),
    }
}

// #[wasm_bindgen]
// pub async fn run(path: String) -> Result<JsValue, JsValue> {
//     let opts = RequestInit::new();
//     opts.set_method("GET");

//     let url = format!("http://192.168.1.91:3002/api/fileList?path={}", path);

//     let request = Request::new_with_str_and_init(&url, &opts)?;

//     request
//         .headers()
//         .set("Accept", "application/vnd.github.v3+json")?;

//     let window = web_sys::window().unwrap();
//     let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

//     // `resp_value` is a `Response` object.
//     // assert!(resp_value.is_instance_of::<Response>());
//     let resp: Response = resp_value.dyn_into().unwrap();

//     // Convert this other `Promise` into a rust `Future`.
//     let json = JsFuture::from(resp.json()?).await?;

//     // Send the JSON response back to JS.
//     Ok(json)
// }
