use crate::components::config_provider::ConfigProvider;
use crate::container::file_list::FileList;
use crate::container::header::Header;
use crate::context::AppContext;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let (path, set_path) = signal("".to_owned());
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
