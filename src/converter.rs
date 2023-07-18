use leptos::{html::Input, *};

use crate::utils;

const NETWORKS: [(&str, u16); 4] = [
    ("Polkadot", 0),
    ("Kusama", 2),
    ("Substrate", 42),
    ("Vara", 137),
];

#[component]
pub(crate) fn Converter(cx: Scope) -> impl IntoView {
    let (checkbox, set_checkbox) = create_signal(cx, false);
    let checkbox_ref: NodeRef<Input> = create_node_ref(cx);

    let (input, _) = create_signal(cx, "".to_string());
    let input_ref: NodeRef<Input> = create_node_ref(cx);

    let (error, set_error) = create_signal(cx, "".to_string());

    let (prefix, set_prefix) = create_signal(cx, 0_u16);
    let prefix_ref: NodeRef<Input> = create_node_ref(cx);

    let (key_prefix, set_key_prefix) = create_signal(cx, 0_u16);
    let (public_key, set_public_key) = create_signal(cx, "".to_string());
    let (custom, set_custom) = create_signal(cx, "".to_string());
    let networks = NETWORKS.map(|_| create_signal(cx, "".to_string()));

    let convert = move || {
        set_error("".to_string());
        if let Some(element) = prefix_ref.get() {
            let value = element.value();
            set_prefix(value.parse().unwrap_or_default());
        }
        if let Some(element) = input_ref.get() {
            let value = element.value();
            let key = if value.starts_with("0x") {
                set_public_key("".to_string());
                value
            } else {
                let res = utils::address_to_key(&value);
                if let Err(err) = res {
                    set_error(err);
                    return;
                }
                let (prefix, key) = res.unwrap();
                set_key_prefix(prefix);
                set_public_key(key);
                public_key()
            };
            if checkbox() {
                let res = utils::key_to_address(prefix(), &key);
                if let Err(err) = res {
                    set_error(err);
                    return;
                }
                set_custom(res.unwrap());
            } else {
                set_custom("".to_string());
            }
            for (i, (_, set_network)) in networks.iter().enumerate() {
                let res = utils::key_to_address(NETWORKS[i].1, &key);
                if let Err(err) = res {
                    set_error(err);
                    return;
                }
                set_network(res.unwrap());
            }
        }
    };

    let on_alice = move |_| {
        if let Some(element) = input_ref.get() {
            element.set_value("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        }
        convert();
    };

    let on_clear = move |_| {
        if let Some(element) = input_ref.get() {
            element.set_value("");
        }
        if let Some(element) = checkbox_ref.get() {
            element.set_checked(false);
        }
        if let Some(element) = prefix_ref.get() {
            element.set_value("0");
        }
        set_checkbox(false);
        set_error("".to_string());
        set_public_key("".to_string());
        set_custom("".to_string());
        for (_, set_network) in networks {
            set_network("".to_string());
        }
    };

    view! { cx,
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
                <button class="button is-danger" on:click=on_clear>
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
                    view! { cx,
                        <Address title=NETWORKS[i].0 prefix=NETWORKS[i].1.into() value=network/>
                    }
                })
                .collect_view(cx)}
        </>
    }
}

#[component]
fn Address(cx: Scope, title: &'static str, prefix: MaybeSignal<u16>, value: ReadSignal<String>) -> impl IntoView {
    let address_ref: NodeRef<Input> = create_node_ref(cx);

    let on_copy = move |_| {
        if let Some(element) = address_ref.get() {
            element.select();
            utils::copy(&element.value())
        }
    };

    view! { cx,
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
