use base58::FromBase58;
use js_sys::Date;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const SS58_LEN: usize = 35;
pub enum Message {
    Convert,
    Alice,
    Clear,
    Copy,
}

#[derive(Default)]
pub struct App {
    address: String,
    address_ref: NodeRef,
    error: String,
    key: String,
    key_ref: NodeRef,
    year: u32,
}

impl App {
    fn address_field(&self) -> Html {
        let mut class = Classes::from("input");
        if self.error.is_empty() {
            class.push("is-info");
        } else {
            class.push("is-danger");
        }

        html! {
            <div class="control">
                <input class={ class }
                    placeholder="e.g. 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
                    ref={ self.address_ref.clone() }
                    value={ self.address.clone() } />
            </div>
        }
    }

    fn error_help(&self) -> Html {
        let mut class = classes!("help", "is-danger");
        if self.error.is_empty() {
            class.push("is-hidden");
        }
        html! {
            <p class={ class }>{ &self.error }</p>
        }
    }

    fn address_to_key(address: &str) -> Result<String, String> {
        address
            .from_base58()
            .map_err(|e| format!("Base58 conversion error: {:?}", e))
            .and_then(|address| {
                let len = address.len();
                if len == SS58_LEN {
                    let public_key = &address[1..33];
                    let hex_public_key = hex::encode(public_key);
                    Ok(format!("0x{}", &hex_public_key))
                } else {
                    Err("SS58 address has wrong length".to_string())
                }
            })
    }

    fn convert(&mut self) {
        match Self::address_to_key(&self.address) {
            Ok(key) => {
                self.error.clear();
                self.key = key;
            }
            Err(e) => self.error = e,
        }
    }
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            year: Date::new_0().get_full_year(),
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Convert => {
                if let Some(address_input) = self.address_ref.cast::<HtmlInputElement>() {
                    self.address = address_input.value();
                }
                self.convert();
                true
            }
            Message::Alice => {
                self.address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string();
                self.convert();
                true
            }
            Message::Clear => {
                self.address.clear();
                self.error.clear();
                self.key.clear();
                true
            }
            Message::Copy => {
                if let Some(key_field) = self.key_ref.cast::<HtmlInputElement>() {
                    let key = key_field.value();
                    key_field.select();
                    gloo_utils::document()
                        .default_view()
                        .and_then(|win| win.navigator().clipboard())
                        .map(|clipboard| clipboard.write_text(&key));
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>

            <div class="box">
                <div class="tabs is-centered is-boxed">
                    <ul>
                        <li class="is-active"><a>
                        <span>{ "Address " }</span>
                        <span class="icon"><i class="fas fa-arrow-circle-right" aria-hidden="true"></i></span>
                        <span>{ " Key" }</span>
                        </a></li>
                    </ul>
                </div>
                <div class="field">
                    <label class="label">{ "SS58 Address" }</label>
                    { self.address_field() }
                    { self.error_help() }
                </div>

                <div class="buttons">
                    <button class="button is-info is-primary"
                        onclick={ ctx.link().callback(|_| Message::Convert) }>
                        <span class="icon"><i class="fas fa-sync"></i></span>
                        <span>{ "Convert" }</span>
                    </button>
                    <button class="button is-light" onclick={ ctx.link().callback(|_| Message::Alice) }>
                        <span class="icon"><i class="fas fa-user"></i></span>
                        <span>{ "Alice" }</span>
                    </button>
                    <button class="button is-danger" onclick={ ctx.link().callback(|_| Message::Clear) }>
                        <span class="icon"><i class="fas fa-times"></i></span>
                        <span>{ "Clear" }</span>
                    </button>
                </div>

                <div class="field">
                    <label class="label">{ "Public Key" }</label>
                    <div class="field has-addons">
                        <div class="control is-expanded">
                            <input class="input is-info" type="text" readonly=true
                                ref={ self.key_ref.clone() }
                                value={ self.key.clone() } />
                        </div>
                        <div class="control">
                            <button class="button is-info is-outlined" onclick={ ctx.link().callback(|_| Message::Copy) }>
                                <span class="icon"><i class="fas fa-copy"></i></span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>

            <div class="content is-small has-text-centered">
                { "Version: " }<strong>{ env!("CARGO_PKG_VERSION") }</strong>
                { " • Source: " }
                <strong>
                    <a href="https://github.com/shamilsan/ss58.org" target="_blank">{ "GitHub" }</a>
                </strong>
                { " • © 2021–" }{ self.year }{ " Shamil" }
            </div>

            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
