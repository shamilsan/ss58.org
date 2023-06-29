mod decoder;
mod encoder;

use js_sys::Date;
use yew::prelude::*;

use decoder::Decoder;
use encoder::Encoder;

pub enum Msg {
    ToEncoder,
    ToDecoder,
}

#[derive(PartialEq)]
enum Mode {
    DecoderMode,
    EncoderMode,
}

pub struct App {
    mode: Mode,
    year: u32,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            mode: Mode::EncoderMode,
            year: Date::new_0().get_full_year(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToEncoder => {
                self.mode = Mode::EncoderMode;
                true
            }
            Msg::ToDecoder => {
                self.mode = Mode::DecoderMode;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let encoder_active = self.mode == Mode::EncoderMode;
        let decoder_active = self.mode == Mode::DecoderMode;
        html! {
            <div class="container">
                <div class="box">
                    <div class="tabs is-centered is-boxed">
                        <ul>
                            <li class={ if encoder_active { "is-active" } else { "" } }>
                                <a onclick={ ctx.link().callback(|_| Msg::ToEncoder) }>
                                    <span>{ " Key" }</span>
                                    <span class="icon"><i class="fas fa-arrow-circle-right" aria-hidden="true"></i></span>
                                    <span>{ "Address " }</span>
                                </a>
                            </li>
                            <li class={ if decoder_active { "is-active" } else { "" } }>
                                <a onclick={ ctx.link().callback(|_| Msg::ToDecoder) }>
                                    <span>{ "Address " }</span>
                                    <span class="icon"><i class="fas fa-arrow-circle-right" aria-hidden="true"></i></span>
                                    <span>{ " Key" }</span>
                                </a>
                            </li>
                        </ul>
                    </div>
                    <Encoder active={ encoder_active } />
                    <Decoder active={ decoder_active } />
                </div>

                <div class="content is-small has-text-centered">
                    { "Version: " }<strong>{ env!("CARGO_PKG_VERSION") }</strong>
                    { " • Source: " }
                    <strong>
                        <a href="https://github.com/gear-tech/ss58.org" target="_blank">{ "GitHub" }</a>
                    </strong>
                    { " • © 2021–" }{ self.year }{ " Gear Technologies Inc." }
                </div>
            </div>
        }
    }
}

pub fn copy(text: &str) {
    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.default_view())
        .and_then(|win| win.navigator().clipboard())
        .map(|clipboard| clipboard.write_text(text));
}

fn main() {
    yew::Renderer::<App>::new().render();
}
