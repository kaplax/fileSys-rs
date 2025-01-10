use leptos::prelude::{ReadSignal, WriteSignal};

#[derive(Clone)]
pub struct AppContext {
  pub path: ReadSignal<String>,
  pub set_path: WriteSignal<String>,
}
