use yew::{html, Component, Context, Html};

pub enum Message {
    Convert,
}

pub struct App {}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Convert => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="form">
                    <button onclick={ ctx.link().callback(|_| Message::Convert) }>
                        { "Convert" }
                    </button>
                </div>

                <p align="center">
                    { "Under construction" }
                </p>

                <p class="footer">
                    { "Version: " }
                    { env!("CARGO_PKG_VERSION") }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
