use base58::ToBase58;
use blake2::{Blake2b512, Digest};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::copy;

const ADDRESS_TYPE: u8 = 42;

pub enum Msg {
    Convert,
    Alice,
    Clear,
    Copy,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active: bool,
}

#[derive(Default)]
pub struct Encoder {
    key: String,
    key_ref: NodeRef,
    error: String,
    address: String,
    address_ref: NodeRef,
}

impl Component for Encoder {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Convert => {
                if let Some(key_input) = self.key_ref.cast::<HtmlInputElement>() {
                    self.key = key_input.value();
                }
                self.convert();
                true
            }
            Msg::Alice => {
                self.key = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    .to_string();
                self.convert();
                true
            }
            Msg::Clear => {
                self.key.clear();
                self.error.clear();
                self.address.clear();
                true
            }
            Msg::Copy => {
                if let Some(address_field) = self.address_ref.cast::<HtmlInputElement>() {
                    address_field.select();
                    copy(&address_field.value());
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div hidden={ !ctx.props().active }>
                <div class="field">
                    <label class="label">{ "Public Key" }</label>
                    { self.key_field() }
                    { self.error_help() }
                </div>

                <div class="buttons">
                    <button class="button is-info is-primary"
                        onclick={ ctx.link().callback(|_| Msg::Convert) }>
                        <span class="icon"><i class="fas fa-sync"></i></span>
                        <span>{ "Convert" }</span>
                    </button>
                    <button class="button is-light" onclick={ ctx.link().callback(|_| Msg::Alice) }>
                        <span class="icon"><i class="fas fa-user"></i></span>
                        <span>{ "Alice" }</span>
                    </button>
                    <button class="button is-danger" onclick={ ctx.link().callback(|_| Msg::Clear) }>
                        <span class="icon"><i class="fas fa-times"></i></span>
                        <span>{ "Clear" }</span>
                    </button>
                </div>

                <div class="field">
                    <label class="label">{ "SS58 Address" }</label>
                    <div class="field has-addons">
                        <div class="control is-expanded">
                            <input class="input is-info" type="text" readonly=true
                                ref={ self.address_ref.clone() }
                                value={ self.address.clone() } />
                        </div>
                        <div class="control">
                            <button class="button is-info is-outlined" onclick={ ctx.link().callback(|_| Msg::Copy) }>
                                <span class="icon"><i class="fas fa-copy"></i></span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

impl Encoder {
    fn key_field(&self) -> Html {
        let mut class = Classes::from("input");
        if self.error.is_empty() {
            class.push("is-info");
        } else {
            class.push("is-danger");
        }

        html! {
            <div class="control">
                <input class={ class }
                    placeholder="e.g. 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    ref={ self.key_ref.clone() }
                    value={ self.key.clone() } />
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

    fn key_to_address(key: &str) -> Result<String, String> {
        let formatted_key = if key.starts_with("0x") {
            &key[2..key.len()]
        } else {
            key
        };

        let raw_key = hex::decode(formatted_key);
        match raw_key {
            Ok(mut raw_key) => {
                if raw_key.len() == 32 {
                    let mut hasher = Blake2b512::new();
                    hasher.update(b"SS58PRE");
                    hasher.update(&[ADDRESS_TYPE]);
                    hasher.update(&raw_key);
                    let checksum = hasher.finalize();

                    let mut raw_address: Vec<u8> = Vec::with_capacity(64);
                    raw_address.push(ADDRESS_TYPE);
                    raw_address.append(&mut raw_key);
                    raw_address.extend_from_slice(&checksum[0..2]);

                    Ok(raw_address[..].to_base58())
                } else {
                    Err(format!(
                        "Public key has wrong length: {} != 32",
                        raw_key.len()
                    ))
                }
            }
            Err(e) => Err(format!("Hex decoding error: {:?}", e)),
        }
    }

    fn convert(&mut self) {
        match Self::key_to_address(&self.key) {
            Ok(address) => {
                self.error.clear();
                self.address = address;
            }
            Err(e) => self.error = e,
        }
    }
}
