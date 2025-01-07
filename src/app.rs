use crate::components::config_provider::ConfigProvider;
use crate::container::file_list::FileList;
use crate::container::header::Header;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <main>
                <Header />
                <FileList path="/favor/movie/other".to_string() />
            </main>
        </ConfigProvider>
    }
}
