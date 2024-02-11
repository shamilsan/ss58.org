use leptos::{html::Input, *};
use leptos_router::*;
use web_sys::KeyboardEvent;

use crate::utils;

const NETWORKS: [(&str, u16, bool); 4] = [
    ("Polkadot", 0, true),
    ("Kusama", 2, true),
    ("Substrate", 42, false),
    ("Vara", 137, true),
];

#[component]
pub(crate) fn Converter() -> impl IntoView {
    let (checkbox, set_checkbox) = create_signal(false);
    let checkbox_ref: NodeRef<Input> = create_node_ref();

    let params = use_params_map();
    let param_input = move || params.with(|params| params.get("input").cloned());

    let (input, _) = create_signal(param_input().unwrap_or_default());
    let input_ref: NodeRef<Input> = create_node_ref();

    let (error, set_error) = create_signal("".to_string());

    let (prefix, set_prefix) = create_signal(0_u16);
    let prefix_ref: NodeRef<Input> = create_node_ref();

    let (key_prefix, set_key_prefix) = create_signal(0_u16);
    let (public_key, set_public_key) = create_signal("".to_string());
    let (custom, set_custom) = create_signal("".to_string());
    let networks = NETWORKS.map(|_| create_signal("".to_string()));

    let convert = move || {
        set_error.set("".to_string());
        if let Some(element) = prefix_ref.get() {
            let value = element.value();
            set_prefix.set(value.parse().unwrap_or_default());
        }
        if let Some(element) = input_ref.get() {
            let value = element.value();
            let key = if value.starts_with("0x") {
                set_public_key.set("".to_string());
                value
            } else {
                let res = utils::address_to_key(&value);
                if let Err(err) = res {
                    set_error.set(err);
                    return;
                }
                let (prefix, key) = res.unwrap();
                set_key_prefix.set(prefix);
                set_public_key.set(key);
                public_key.get()
            };
            if checkbox.get() {
                let res = utils::key_to_address(prefix.get(), &key);
                if let Err(err) = res {
                    set_error.set(err);
                    return;
                }
                set_custom.set(res.unwrap());
            } else {
                set_custom.set("".to_string());
            }
            for (i, (_, set_network)) in networks.iter().enumerate() {
                let res = utils::key_to_address(NETWORKS[i].1, &key);
                if let Err(err) = res {
                    set_error.set(err);
                    return;
                }
                set_network.set(res.unwrap());
            }
        }
    };

    let on_alice = move |_| {
        if let Some(element) = input_ref.get() {
            element.set_value("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        }
        convert();
    };

    let clear = move || {
        let _ = input_ref.get().map(|element| element.set_value(""));
        let _ = checkbox_ref.get().map(|element| element.set_checked(false));
        let _ = prefix_ref.get().map(|element| element.set_value("0"));
        set_checkbox.set(false);
        set_error.set("".to_string());
        set_public_key.set("".to_string());
        set_custom.set("".to_string());
        for (_, set_network) in networks {
            set_network.set("".to_string());
        }
    };

    let on_keyup = move |e: KeyboardEvent| {
        if e.key() == "Enter" {
            convert();
        } else if e.key() == "Escape" {
            clear();
        }
    };

    view! {
        <>
            // Input
            <div class="field">
                <label class="label">"SS58 Address or Public Key"</label>
                <div class="control">
                    <input
                        class="input"
                        class:is-info=move || error.with(String::is_empty)
                        class:is-danger=move || !error.with(String::is_empty)
                        placeholder="e.g. 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
                        node_ref=input_ref
                        value=input
                        on:keyup=on_keyup
                    />
                </div>
                <p class="help is-danger" class:is-hidden=move || error.with(String::is_empty)>
                    {error}
                </p>
            </div>

            // Prefix
            <div class="field has-addons">
                <div class="columns is-flex is-vcentered">
                    <div class="column">
                        <span class=" control p-1">
                            <div class="pretty p-switch p-fill">
                                <input
                                    type="checkbox"
                                    node_ref=checkbox_ref
                                    checked=checkbox
                                    on:click=move |_| set_checkbox.update(|v| *v = !*v)
                                />
                                <div class="state p-primary">
                                    <label>"Custom Prefix"</label>
                                </div>
                            </div>
                        </span>
                    </div>
                    <div class="column">
                        <div class="control p-1">
                            <input
                                class="input is-info"
                                type="number"
                                min="0"
                                max="16383"
                                disabled=move || checkbox.with(|&v| !v)
                                node_ref=prefix_ref
                                value=prefix
                            />
                        </div>
                    </div>
                </div>
            </div>

            // Buttons
            <div class="buttons">
                <button class="button is-info is-primary" on:click=move |_| convert()>
                    <span class="icon">
                        <i class="fas fa-sync"></i>
                    </span>
                    <span>"Convert"</span>
                </button>
                <button class="button is-light" on:click=on_alice>
                    <span class="icon">
                        <i class="fas fa-user"></i>
                    </span>
                    <span>"Alice"</span>
                </button>
                <button class="button is-danger" on:click=move |_| clear()>
                    <span class="icon">
                        <i class="fas fa-times"></i>
                    </span>
                    <span>"Clear"</span>
                </button>
            </div>

            // Output
            <Address title="Public Key" prefix=key_prefix.into() value=public_key/>
            <Address title="Custom Prefix" prefix=prefix.into() value=custom/>
            {networks
                .into_iter()
                .enumerate()
                .map(|(i, (network, _))| {
                    view! {
                        <Address
                            title=NETWORKS[i].0
                            prefix=NETWORKS[i].1.into()
                            value=network
                            subscan=NETWORKS[i].2
                        />
                    }
                })
                .collect_view()}
        </>
    }
}

#[component]
fn Address(
    title: &'static str,
    prefix: MaybeSignal<u16>,
    value: ReadSignal<String>,
    #[prop(optional)] subscan: bool,
) -> impl IntoView {
    let address_ref: NodeRef<Input> = create_node_ref();

    let on_copy = move |_| {
        if let Some(element) = address_ref.get() {
            element.select();
            utils::copy(&element.value())
        }
    };

    let subscan_link = move || {
        format!(
            "https://{}.subscan.io/account/{}",
            title.to_lowercase(),
            value.get()
        )
    };

    view! {
        <div hidden=move || value.with(String::is_empty) class="field">
            <label class="label is-small is-family-monospace is-uppercase">
                {title} ": " {prefix}
            </label>
            <div class="field-body">
                <div class="field is-expanded">
                    <div class="field has-addons">
                        <div class="control is-expanded ">
                            <input
                                class="input is-info"
                                type="text"
                                readonly=true
                                node_ref=address_ref
                                value=value
                            />
                        </div>
                        <div class="control" hidden=!subscan>
                            <a class="button is-info is-outlined" href=subscan_link target="_blank">
                                <span class="icon" alt="Subscan">
                                    <i class="fas fa-search"></i>
                                </span>
                            </a>
                        </div>
                        <div class="control">
                            <button class="button is-info is-outlined" on:click=on_copy>
                                <span class="icon">
                                    <i class="fas fa-copy"></i>
                                </span>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
