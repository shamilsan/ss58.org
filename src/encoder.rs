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
    Copy(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd)]
pub struct Network {
    prefix: u16,
    name: String,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub active: bool,
}

#[derive(Default)]
pub struct Encoder {
    checkbox_ref: NodeRef,
    prefix_ref: NodeRef,
    prefix: u16,
    custom_prefix: bool,
    key: String,
    key_ref: NodeRef,
    error: String,
    networks: Vec<Network>,
    addresses: Vec<(Network, String, NodeRef)>,
}

impl Component for Encoder {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            networks: vec![
                Network {
                    prefix: 0,
                    name: "Polkadot".to_string(),
                },
                Network {
                    prefix: 2,
                    name: "Kusama".to_string(),
                },
                Network {
                    prefix: 42,
                    name: "Substrate".to_string(),
                },
                Network {
                    prefix: 137,
                    name: "Vara".to_string(),
                },
            ],
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Prefix => {
                self.custom_prefix = !self.custom_prefix;
                if let Some(prefix_value) = self.prefix_ref.cast::<HtmlInputElement>() {
                    self.prefix = prefix_value.value().parse().unwrap_or_default();
                }

                if let Some(key_input) = self.key_ref.cast::<HtmlInputElement>() {
                    self.key = key_input.value();
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
                if let Some(prefix_select) = self.checkbox_ref.cast::<HtmlSelectElement>() {
                    self.prefix = 0;
                    prefix_select.set_value(&self.prefix.to_string());
                }
                self.key.clear();
                self.error.clear();
                self.addresses.clear();
                self.custom_prefix = false;
                true
            }
            Msg::Copy(index) => {
                if let Some(address_field) = self.addresses[index].2.cast::<HtmlInputElement>() {
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
            <div class="field has-addons">
                <span class="control p-1">
                    <div class="pretty p-switch p-fill">
                        <input checked={ self.custom_prefix } type="checkbox" ref={ self.checkbox_ref.clone() }
                            onclick={ ctx.link().callback(|_| Msg::Prefix) } />
                        <div class="state p-primary">
                            <label>{ "Custom Prefix"} </label>
                            // <label>Primary</label>
                        </div>
                    </div>

                </span>
                <div class="control p-1">
                    <input class="input is-info is-small" type="number" min="0" max="16383" disabled={ !self.custom_prefix }
                        ref={ self.prefix_ref.clone() } value={ self.prefix.to_string() } />
                </div>
            </div>

            <div class="buttons">
                <button class="button is-info is-primary" onclick={ ctx.link().callback(|_| Msg::Convert) }>
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

            { self.render_addresses(ctx) }
        </div>
        }
    }
}

impl Encoder {
    fn key_field(&self) -> Html {
        let class = classes!(
            "input",
            if self.error.is_empty() {
                "is-info"
            } else {
                "is-danger"
            }
        );
        html! {
            <div class="control">
                <input { class }
                    placeholder="e.g. 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
                    ref={ self.key_ref.clone() }
                    value={ self.key.clone() } />
            </div>
        }
    }

    fn error_help(&self) -> Html {
        let class = classes!(
            "help",
            "is-danger",
            self.error.is_empty().then_some("is-hidden")
        );
        html!(<p { class }>{ &self.error }</p>)
    }

    fn key_to_address(prefix: u16, key: &str) -> Result<String, String> {
        let formatted_key = if key.starts_with("0x") {
            &key[2..key.len()]
        } else {
            key
        };

        let raw_key = hex::decode(formatted_key);
        match raw_key {
            Err(e) => Err(format!("Hex decoding error: {e:?}")),
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
                        hasher.update([simple_prefix]);
                    } else {
                        hasher.update([prefix_hi]);
                        hasher.update([prefix_low]);
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
        self.addresses.clear();
        if self.custom_prefix {
            match Self::key_to_address(self.prefix, &self.key) {
                Err(e) => self.error = e,
                Ok(address) => self.addresses.insert(
                    0,
                    (
                        Network {
                            prefix: self.prefix,
                            name: "Custom".to_string(),
                        },
                        address,
                        NodeRef::default(),
                    ),
                ),
            }
        }
        for network in self.networks.iter() {
            match Self::key_to_address(network.prefix, &self.key) {
                Err(e) => self.error = e,
                Ok(address) => {
                    self.error.clear();
                    self.addresses
                        .push((network.clone(), address, NodeRef::default()));
                }
            }
        }
    }

    fn render_addresses(&self, ctx: &Context<Encoder>) -> Html {
        self.addresses.iter().enumerate().map(|(index, (network, address, node_ref))| {
            html!{
                <div class="field">
                    <p class="control is-small">
                        <span class="label is-static is-small is-family-monospace is-uppercase">
                            { format!("{}: {}", network.name.to_string(),network.prefix.to_string()) }
                        </span>
                    </p>
                    <div class="field-body">
                        <div class="field is-expanded">
                            <div class="field has-addons">
                                <div class="control is-expanded ">
                                    <input class="input is-info" type="text" readonly=true ref={ node_ref.clone() } value={
                                        address.clone() } />
                                </div>
                                <div class="control">
                                    <button class="button is-info is-outlined" onclick={ ctx.link().callback(move |_| Msg::Copy(index))
                                        }>
                                        <span class="icon"><i class="fas fa-copy"></i></span>
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                }

            }).collect::<Html>()
    }
}
