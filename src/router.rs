use leptos::prelude::*;
use leptos_router::{components::*, path};

use crate::page::main::Main;
use crate::page::reader::Reader;

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <Routes fallback=|| view! { <div> "404" </div> }>
          <Route path=path!("/") view=Main />
          <Route path=path!("/reader") view=Reader />
        </Routes>
      </Router>
    }
}
