use base58::ToBase58;
use blake2::{Blake2b512, Digest};
use web_sys::{HtmlInputElement, HtmlSelectElement};
use yew::prelude::*;

use crate::copy;

pub enum Msg {
    Prefix,
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
    select_ref: NodeRef,
    prefix_ref: NodeRef,
    prefix: u16,
    custom_prefix: bool,
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
            Msg::Prefix => {
                if let Some(prefix_select) = self.select_ref.cast::<HtmlSelectElement>() {
                    let prefix = prefix_select.value();
                    if prefix == "custom" {
                        self.custom_prefix = true;
                    } else {
                        self.custom_prefix = false;
                        self.prefix = prefix_select.value().parse().unwrap_or_default();
                    }
                }
                true
            }
            Msg::Convert => {
                if let Some(prefix_value) = self.prefix_ref.cast::<HtmlInputElement>() {
                    self.prefix = prefix_value.value().parse().unwrap_or_default();
                }
                if let Some(key_input) = self.key_ref.cast::<HtmlInputElement>() {
                    self.key = key_input.value();
                }
                self.convert();
                true
            }
            Msg::Alice => {
                if let Some(prefix_value) = self.prefix_ref.cast::<HtmlInputElement>() {
                    self.prefix = prefix_value.value().parse().unwrap_or_default();
                }
                self.key = "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    .to_string();
                self.convert();
                true
            }
            Msg::Clear => {
                if let Some(prefix_select) = self.select_ref.cast::<HtmlSelectElement>() {
                    self.prefix = 0;
                    prefix_select.set_value(&self.prefix.to_string());
                }
                self.key.clear();
                self.error.clear();
                self.address.clear();
                self.custom_prefix = false;
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
                    <label class="label">{ "Network Prefix" }</label>
                    <div class="field has-addons">
                        <div class="control">
                            <div class="select is-info">
                                <select ref={ self.select_ref.clone() } onchange={ ctx.link().callback(|_| Msg::Prefix) }>
                                    <option value="0" selected=true>{ "Polkadot" }</option>
                                    <option value="2">{ "Kusama" }</option>
                                    <option value="42">{ "Substrate" }</option>
                                    <option value="137">{ "Vara" }</option>
                                    <option value="custom">{ "Custom" }</option>
                                </select>
                            </div>
                        </div>
                        <div class="control is-expanded">
                            <input class="input is-info" type="number" min="0" max="16383"
                                disabled={ !self.custom_prefix } ref={ self.prefix_ref.clone() } value={ self.prefix.to_string() }/>
                        </div>
                    </div>
                </div>

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

    fn key_to_address(prefix: u16, key: &str) -> Result<String, String> {
        let formatted_key = if key.starts_with("0x") {
            &key[2..key.len()]
        } else {
            key
        };

        let raw_key = hex::decode(formatted_key);
        match raw_key {
            Err(e) => Err(format!("Hex decoding error: {:?}", e)),
            Ok(mut raw_key) => {
                if raw_key.len() != 32 {
                    Err(format!(
                        "Public key has wrong length: {} != 32",
                        raw_key.len()
                    ))
                } else {
                    let mut hasher = Blake2b512::new();
                    hasher.update(b"SS58PRE");
                    let simple_prefix: u8 = (prefix & 0x3F) as _;
                    let full_prefix = 0x4000 | ((prefix >> 8) & 0x3F) | ((prefix & 0xFF) << 6);
                    let prefix_hi: u8 = (full_prefix >> 8) as _;
                    let prefix_low: u8 = (full_prefix & 0xFF) as _;
                    if prefix == simple_prefix as u16 {
                        hasher.update(&[simple_prefix]);
                    } else {
                        hasher.update(&[prefix_hi]);
                        hasher.update(&[prefix_low]);
                    }
                    hasher.update(&raw_key);
                    let checksum = hasher.finalize();

                    let mut raw_address: Vec<u8> = Vec::with_capacity(64);
                    if prefix == simple_prefix as u16 {
                        raw_address.push(simple_prefix);
                    } else {
                        raw_address.push(prefix_hi);
                        raw_address.push(prefix_low);
                    }
                    raw_address.append(&mut raw_key);
                    raw_address.extend_from_slice(&checksum[0..2]);

                    Ok(raw_address[..].to_base58())
                }
            }
        }
    }

    fn convert(&mut self) {
        match Self::key_to_address(self.prefix, &self.key) {
            Err(e) => self.error = e,
            Ok(address) => {
                self.error.clear();
                self.address = address;
            }
        }
    }
}
