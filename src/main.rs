use js_sys::Date;
use leptos::prelude::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use converter::Converter;

mod converter;
mod utils;

#[component]
fn App() -> impl IntoView {
    let year = Date::new_0().get_full_year();

    view! {
        <div class="container">
            <div class="box">
                <Router>
                    <Routes fallback=|| "Not found.">
                        <Route path=path!("/") view=Converter/>
                    </Routes>
                </Router>
            </div>

            <div class="content is-small has-text-centered">
                "Version: " <strong>{env!("CARGO_PKG_VERSION")}</strong> " • Source: " <strong>
                    <a href="https://github.com/shamilsan/ss58.org" target="_blank">
                        "GitHub"
                    </a>
                </strong> " • © 2021–" {year} " Shamil"
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
