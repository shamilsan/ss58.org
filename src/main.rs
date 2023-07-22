use js_sys::Date;
use leptos::*;
use leptos_router::*;

mod converter;
use converter::Converter;

mod utils;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let year = Date::new_0().get_full_year();

    view! { cx,
        <div class="container">
            <div class="box">
                <Router>
                    <Routes>
                        <Route path="/" view=Converter/>
                        <Route path=":input" view=Converter/>
                    </Routes>
                </Router>
            </div>

            <div class="content is-small has-text-centered">
                "Version: " <strong>{env!("CARGO_PKG_VERSION")}</strong> " • Source: " <strong>
                    <a href="https://github.com/gear-tech/ss58.org" target="_blank">
                        "GitHub"
                    </a>
                </strong> " • © 2021–" {year} " Gear Technologies Inc."
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(App);
}
