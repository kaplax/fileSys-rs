mod components;
mod utils;
mod container;
mod api;
mod hooks;
mod context;
mod router;
mod page;

use router::App;

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
