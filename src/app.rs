use crate::components::config_provider::ConfigProvider;
use crate::container::file_list::FileList;
use crate::container::header::Header;
use crate::context::AppContext;
use crate::utils::url::get_url_params;
use leptos::prelude::*;
use leptos_use::use_window;

#[component]
pub fn App() -> impl IntoView {
    let window = use_window();
    let url = window.as_ref().unwrap().location().search().unwrap();
    let url_params = get_url_params(url.as_str());
    let path = url_params.unwrap().get("path").unwrap_or_default();
    let (path, set_path) = signal(path);
    provide_context(AppContext { path, set_path });
    view! {
        <ConfigProvider>
            <main>
                <Header />
                <FileList />
            </main>
        </ConfigProvider>
    }
}
