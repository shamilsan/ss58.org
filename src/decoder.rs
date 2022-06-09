use base58::FromBase58;
use blake2::{Blake2b512, Digest};
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::copy;

const SS58_MIN_LEN: usize = 35;
const SS58_MAX_LEN: usize = 36;

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
pub struct Decoder {
    address: String,
    address_ref: NodeRef,
    error: String,
    prefix: u16,
    key: String,
    key_ref: NodeRef,
}

impl Component for Decoder {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Convert => {
                if let Some(address_input) = self.address_ref.cast::<HtmlInputElement>() {
                    self.address = address_input.value();
                }
                self.convert();
                true
            }
            Msg::Alice => {
                self.address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY".to_string();
                self.convert();
                true
            }
            Msg::Clear => {
                self.address.clear();
                self.error.clear();
                self.prefix = 0;
                self.key.clear();
                true
            }
            Msg::Copy => {
                if let Some(key_field) = self.key_ref.cast::<HtmlInputElement>() {
                    key_field.select();
                    copy(&key_field.value());
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div hidden={ !ctx.props().active }>
                <div class="field">
                    <label class="label">{ "SS58 Address" }</label>
                    { self.address_field() }
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
                    <label class="label">{ "Public Key" }</label>
                    <div class="field has-addons">
                        <div class="control">
                            <a class="button is-static">{ self.prefix.to_string() }</a>
                        </div>
                        <div class="control is-expanded">
                            <input class="input is-info" type="text" readonly=true
                                ref={ self.key_ref.clone() }
                                value={ self.key.clone() } />
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

impl Decoder {
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

    fn address_to_key(address: &str) -> Result<(u16, String), String> {
        let address = address
            .from_base58()
            .map_err(|e| format!("Base58 conversion error: {:?}", e))?;
        let len = address.len();
        if !(SS58_MIN_LEN..=SS58_MAX_LEN).contains(&len) {
            Err("SS58 address has wrong length".to_string())
        } else {
            // Verify the checksum
            let checksum = &address[len - 2..len];
            let mut hasher = Blake2b512::new();
            hasher.update(b"SS58PRE");
            hasher.update(&address[0..len - 2]);
            let checksum_calc = &hasher.finalize()[0..2];
            if checksum != checksum_calc {
                return Err(format!("Invalid checksum: input{checksum:?} not equal to calculated{checksum_calc:?}"));
            }

            // Get the key
            let key = format!("0x{}", hex::encode(&address[len - 34..len - 2]));

            // Get the network prefix
            let prefix_buf = &address[0..len - 34];
            let prefix = if prefix_buf.len() == 1 {
                prefix_buf[0] as u16
            } else {
                let prefix_hi = ((prefix_buf[1] & 0x3F) as u16) << 8;
                let prefix_lo = ((prefix_buf[0] << 2) | (prefix_buf[1] >> 6)) as u16;
                prefix_hi | prefix_lo
            };

            Ok((prefix, key))
        }
    }

    fn convert(&mut self) {
        match Self::address_to_key(&self.address) {
            Err(e) => self.error = e,
            Ok((prefix, key)) => {
                self.error.clear();
                self.prefix = prefix;
                self.key = key;
            }
        }
    }
}
