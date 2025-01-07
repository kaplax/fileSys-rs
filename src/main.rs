mod app;
mod components;
mod utils;
mod container;
mod api;
mod hooks;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
