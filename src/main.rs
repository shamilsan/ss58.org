use base58::FromBase58;
use yew::{html, Component, Context, Html};

pub enum Message {
    Convert,
    Alice,
    Clear,
}

pub struct App {
    address: String,
    key: String,
}

impl Component for App {
    type Message = Message;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            address: String::new(),
            key: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Message::Convert => {
                self.address.clear();
                self.key = "Not implemented yet, coming soom".to_string();
                true
            }
            Message::Alice => {
                self.address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string();
                self.convert();
                true
            }
            Message::Clear => {
                self.address.clear();
                self.key.clear();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
                <div class="box">
                    <div class="tabs is-centered is-boxed is-large">
                        <ul>
                            <li class="is-active"><a>{ "SS58 → Key"}</a></li>
                        </ul>
                    </div>
                    <div class="field">
                        <label class="label">{ "SS58 Address" }</label>
                        <div class="control">
                            <input class="input is-info" type="text" placeholder="e.g. 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY" value={ self.address.clone() } />
                        </div>
                    </div>
                    <div class="field">
                        <label class="label">{ "Public Key" }</label>
                        <div class="control">
                            <input class="input is-info" type="text" disabled=true value={ self.key.clone() }/>
                        </div>
                    </div>
                    <div class="buttons">
                        <button class="button is-info is-primary" onclick={ ctx.link().callback(|_| Message::Convert) }>
                            <span class="icon">
                                <i class="fas fa-sync"></i>
                            </span>
                            <span>{ "Convert" }</span>
                        </button>
                        <button class="button is-light" onclick={ ctx.link().callback(|_| Message::Alice) }>
                            <span class="icon">
                                <i class="fas fa-user"></i>
                            </span>
                            <span>{ "Alice" }</span>
                        </button>
                        <button class="button is-danger" onclick={ ctx.link().callback(|_| Message::Clear) }>
                            <span class="icon">
                                <i class="fas fa-times"></i>
                            </span>
                            <span>{ "Clear" }</span>
                        </button>
                    </div>
                    <div class="content has-text-centered">
                        { "Version: " }
                        <strong>{ env!("CARGO_PKG_VERSION") }</strong>
                        { " | Source: " }
                        <a href="https://github.com/shamilsan/ss58.org" target="_blank">{ "GitHub" }</a>
                        { " | © " }
                        { 2022 }
                        { " Shamil" }
                    </div>
                </div>
        }
    }
}

impl App {
    fn convert(&mut self) {
        let res = self.address.from_base58();
        match res {
            Ok(key) => {
                let len = key.len();
                if len == 35 {
                    let public_key = &key[1..33];
                    let hex_public_key = hex::encode(public_key);
                    self.key = format!("0x{}", &hex_public_key);
                } else {
                    self.key = "Error: Invalid SS58 address, wrong length".to_string();
                }
            }
            Err(_) => self.key = "Error: Invalid SS58 address".to_string(),
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
