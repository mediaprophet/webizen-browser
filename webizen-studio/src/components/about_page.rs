use dioxus::prelude::*;
use serde::Deserialize;

#[cfg(target_arch = "wasm32")]
use serde_json::json;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

const PAGE_STYLE: &str =
    "width: 100%; height: 100%; overflow-y: auto; padding: 2rem 2rem 3rem;";
const PANEL_STYLE: &str = "background: var(--qualia-surface); border: 1px solid var(--qualia-border); border-radius: 18px; padding: 1.2rem; backdrop-filter: blur(22px); box-shadow: 0 10px 32px rgba(0,0,0,0.08);";

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn tauri_invoke(
        cmd: &str,
        args: js_sys::Object,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}

#[cfg(target_arch = "wasm32")]
async fn invoke_tauri_json<T>(cmd: &str, args: serde_json::Value) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let js_args = serde_wasm_bindgen::to_value(&args).map_err(|e| e.to_string())?;
    let value = tauri_invoke(cmd, js_args.into())
        .await
        .map_err(|e| format!("{e:?}"))?;
    serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
struct AgentConfigSnapshot {
    storage_path: String,
    daemon_host: String,
    daemon_port: u16,
    inference_backend: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
struct WalletStatusSnapshot {
    lightning_sats: u64,
    ilp_microcents: u64,
    nym_connected: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct RuntimeAboutState {
    daemon_status: String,
    config: Option<AgentConfigSnapshot>,
    wallet: Option<WalletStatusSnapshot>,
    identity_present: Option<bool>,
    status_note: String,
}

impl Default for RuntimeAboutState {
    fn default() -> Self {
        Self {
            daemon_status: "unknown".to_string(),
            config: None,
            wallet: None,
            identity_present: None,
            status_note: "Waiting for desktop runtime metadata…".to_string(),
        }
    }
}

#[component]
fn InfoPill(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "background: rgba(128,128,128,0.06); border: 1px solid var(--qualia-border); border-radius: 14px; padding: 0.9rem 1rem; min-height: 88px;",
            div {
                style: "font-size: 0.68rem; font-weight: 700; color: var(--qualia-text-muted); letter-spacing: 0.08em; text-transform: uppercase; margin-bottom: 0.45rem;",
                "{label}"
            }
            div {
                style: "font-size: 0.86rem; font-weight: 600; color: var(--qualia-text); line-height: 1.35;",
                "{value}"
            }
        }
    }
}

#[component]
fn CopyRow(label: &'static str, value: String) -> Element {
    rsx! {
        div {
            style: "display: grid; grid-template-columns: 120px 1fr; gap: 0.75rem; align-items: start; padding: 0.65rem 0; border-bottom: 1px solid rgba(255,255,255,0.05);",
            span {
                style: "font-size: 0.69rem; font-weight: 700; color: var(--qualia-text-muted); letter-spacing: 0.05em; text-transform: uppercase;",
                "{label}"
            }
            code {
                style: "font-size: 0.76rem; color: var(--qualia-text); background: rgba(128,128,128,0.08); border: 1px solid var(--qualia-border); border-radius: 10px; padding: 0.5rem 0.65rem; font-family: 'Consolas', 'SFMono-Regular', monospace; word-break: break-word;",
                "{value}"
            }
        }
    }
}

#[component]
pub fn AboutPage() -> Element {
    let host_surface = match crate::endpoints::current_host_surface() {
        crate::endpoints::HostSurface::DesktopWebview => "Desktop webview".to_string(),
        crate::endpoints::HostSurface::PublicWeb => "Public web".to_string(),
    };
    let browser_pane_status = if crate::endpoints::supports_browser_pane() {
        "available".to_string()
    } else {
        "disabled on this surface".to_string()
    };
    let desktop_surface =
        crate::endpoints::current_host_surface() == crate::endpoints::HostSurface::DesktopWebview;

    let runtime = use_signal(RuntimeAboutState::default);
    let load_started = use_signal(|| false);
    #[cfg(not(target_arch = "wasm32"))]
    let _ = load_started;

    use_effect(move || {
        #[cfg(target_arch = "wasm32")]
        {
            if !desktop_surface || load_started() {
                return;
            }

            load_started.set(true);
            let mut runtime = runtime;

            spawn(async move {
                let daemon_status = invoke_tauri_json::<String>("daemon_status", json!({}))
                    .await
                    .unwrap_or_else(|err| format!("Unavailable ({err})"));
                let config =
                    invoke_tauri_json::<AgentConfigSnapshot>("get_config", json!({})).await.ok();
                let wallet =
                    invoke_tauri_json::<WalletStatusSnapshot>("get_wallet_status", json!({}))
                        .await
                        .ok();
                let identity = invoke_tauri_json::<Option<serde_json::Value>>(
                    "read_identity",
                    json!({}),
                )
                .await
                .ok()
                .flatten();

                runtime.set(RuntimeAboutState {
                    daemon_status,
                    config,
                    wallet,
                    identity_present: Some(identity.is_some()),
                    status_note: "Desktop runtime metadata loaded from native commands.".to_string(),
                });
            });
        }
    });

    let runtime_snapshot = runtime();
    let daemon_endpoint = runtime_snapshot
        .config
        .as_ref()
        .map(|config| format!("{}:{}", config.daemon_host, config.daemon_port))
        .unwrap_or_else(|| "127.0.0.1:4242".to_string());
    let storage_path = runtime_snapshot
        .config
        .as_ref()
        .map(|config| config.storage_path.clone())
        .unwrap_or_else(|| "Desktop config unavailable on this surface".to_string());
    let inference_backend = runtime_snapshot
        .config
        .as_ref()
        .map(|config| config.inference_backend.clone())
        .unwrap_or_else(|| "unknown".to_string());
    let identity_status = match runtime_snapshot.identity_present {
        Some(true) => "Configured".to_string(),
        Some(false) => "Not configured".to_string(),
        None => "Unknown".to_string(),
    };
    let wallet_summary = runtime_snapshot
        .wallet
        .as_ref()
        .map(|wallet| {
            format!(
                "{} sats / {} ILP mc / Nym {}",
                wallet.lightning_sats,
                wallet.ilp_microcents,
                if wallet.nym_connected {
                    "connected"
                } else {
                    "offline"
                }
            )
        })
        .unwrap_or_else(|| "Wallet status unavailable".to_string());

    rsx! {
        div { style: PAGE_STYLE,
            div {
                style: "display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem; margin-bottom: 1.5rem; flex-wrap: wrap;",
                div {
                    h1 {
                        style: "margin: 0 0 0.25rem 0; font-size: 1.45rem; font-weight: 700; color: var(--qualia-text); letter-spacing: -0.02em;",
                        "About Webizen"
                    }
                    p {
                        style: "margin: 0; max-width: 44rem; font-size: 0.8rem; color: var(--qualia-text-muted); line-height: 1.55;",
                        "This surface now mixes project context with live runtime state, so it doubles as a quick support snapshot for the current host."
                    }
                }
                span {
                    style: "font-size: 0.68rem; font-weight: 700; color: var(--qualia-accent); background: var(--qualia-accent-glow); border: 1px solid var(--qualia-border); border-radius: 999px; padding: 0.28rem 0.62rem; letter-spacing: 0.06em; text-transform: uppercase;",
                    if desktop_surface { "Runtime-backed" } else { "Public Web" }
                }
            }

            div {
                class: "panel-card",
                style: "{PANEL_STYLE} margin-bottom: 1rem;",
                div {
                    style: "display: grid; grid-template-columns: minmax(0, 1.4fr) minmax(260px, 0.9fr); gap: 1rem; align-items: stretch;",
                    div {
                        style: "display: flex; flex-direction: column; justify-content: space-between; gap: 0.9rem;",
                        div {
                            span {
                                style: "display: inline-flex; align-items: center; gap: 0.35rem; font-size: 0.66rem; font-weight: 700; color: #fb923c; background: rgba(251,146,60,0.12); border: 1px solid rgba(251,146,60,0.24); border-radius: 999px; padding: 0.2rem 0.48rem; letter-spacing: 0.08em; text-transform: uppercase;",
                                "Local-First Workspace"
                            }
                            h2 {
                                style: "margin: 0.7rem 0 0.35rem 0; font-size: 1.25rem; font-weight: 700; color: var(--qualia-text); letter-spacing: -0.02em;",
                                "Webizen Studio"
                            }
                            p {
                                style: "margin: 0; font-size: 0.78rem; color: var(--qualia-text-muted); line-height: 1.62;",
                                "Webizen is a graph-native environment for working with knowledge, tools, and personal computing systems in one place. For this 0.0.3 slice, the desktop shell now exposes enough runtime state to make the about page useful during setup, screenshots, and debugging."
                            }
                        }
                        div {
                            style: "display: flex; gap: 0.5rem; flex-wrap: wrap;",
                            span {
                                style: "font-size: 0.7rem; color: var(--qualia-text); background: rgba(128,128,128,0.08); border: 1px solid var(--qualia-border); border-radius: 999px; padding: 0.24rem 0.58rem;",
                                "Host-aware"
                            }
                            span {
                                style: "font-size: 0.7rem; color: var(--qualia-text); background: rgba(128,128,128,0.08); border: 1px solid var(--qualia-border); border-radius: 999px; padding: 0.24rem 0.58rem;",
                                "Desktop config"
                            }
                            span {
                                style: "font-size: 0.7rem; color: var(--qualia-text); background: rgba(128,128,128,0.08); border: 1px solid var(--qualia-border); border-radius: 999px; padding: 0.24rem 0.58rem;",
                                "0.0.3"
                            }
                        }
                    }

                    div {
                        style: "background: linear-gradient(160deg, rgba(251,146,60,0.16) 0%, rgba(255,255,255,0.04) 100%); border: 1px solid var(--qualia-border); border-radius: 16px; padding: 1rem; display: flex; flex-direction: column; gap: 0.7rem; justify-content: center;",
                        div {
                            style: "font-size: 0.71rem; font-weight: 700; color: var(--qualia-text-muted); letter-spacing: 0.08em; text-transform: uppercase;",
                            "Runtime Snapshot"
                        }
                        div {
                            style: "font-size: 1rem; font-weight: 700; color: var(--qualia-text);",
                            "{runtime_snapshot.daemon_status.clone()}"
                        }
                        p {
                            style: "margin: 0; font-size: 0.73rem; color: var(--qualia-text-muted); line-height: 1.55;",
                            if desktop_surface {
                                "{runtime_snapshot.status_note.clone()}"
                            } else {
                                "The public web build intentionally avoids native commands, so runtime metadata is limited here."
                            }
                        }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 0.85rem; margin-bottom: 1rem;",
                InfoPill { label: "Studio Version", value: "0.0.3".to_string() }
                InfoPill { label: "Host Surface", value: host_surface }
                InfoPill { label: "Browser Pane", value: browser_pane_status }
                InfoPill { label: "Inference", value: inference_backend.clone() }
            }

            div {
                style: "display: grid; grid-template-columns: minmax(0, 1.15fr) minmax(260px, 0.85fr); gap: 1rem;",
                div {
                    class: "panel-card",
                    style: PANEL_STYLE,
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between; gap: 0.75rem; margin-bottom: 0.85rem; flex-wrap: wrap;",
                        div {
                            h2 {
                                style: "margin: 0 0 0.2rem 0; font-size: 0.94rem; font-weight: 650; color: var(--qualia-text);",
                                "Runtime State"
                            }
                            p {
                                style: "margin: 0; font-size: 0.73rem; color: var(--qualia-text-muted); line-height: 1.5;",
                                "Live values from the current desktop session when native commands are available."
                            }
                        }
                        span {
                            style: "font-size: 0.64rem; font-weight: 700; letter-spacing: 0.06em; text-transform: uppercase; color: var(--qualia-accent); background: var(--qualia-accent-glow); border: 1px solid var(--qualia-border); border-radius: 999px; padding: 0.18rem 0.45rem;",
                            if desktop_surface { "Live" } else { "Limited" }
                        }
                    }
                    div {
                        style: "display: flex; flex-direction: column;",
                        CopyRow { label: "Daemon", value: runtime_snapshot.daemon_status.clone() }
                        CopyRow { label: "Endpoint", value: daemon_endpoint }
                        CopyRow { label: "Storage", value: storage_path }
                        CopyRow { label: "Identity", value: identity_status }
                        CopyRow { label: "Wallet", value: wallet_summary }
                    }
                }

                div {
                    class: "panel-card",
                    style: PANEL_STYLE,
                    h2 {
                        style: "margin: 0 0 0.5rem 0; font-size: 0.94rem; font-weight: 650; color: var(--qualia-text);",
                        "Why Webizen"
                    }
                    p {
                        style: "margin: 0 0 0.8rem 0; font-size: 0.74rem; color: var(--qualia-text-muted); line-height: 1.6;",
                        "The project aims to make personal computing feel more legible and more accountable. Rather than treating tools, data, and automation as disconnected islands, Webizen frames them as related surfaces inside one governed environment."
                    }
                    div {
                        style: "display: flex; flex-direction: column; gap: 0.6rem;",
                        div {
                            style: "padding: 0.75rem 0.8rem; background: rgba(128,128,128,0.05); border: 1px solid var(--qualia-border); border-radius: 12px;",
                            div {
                                style: "font-size: 0.76rem; font-weight: 600; color: var(--qualia-text); margin-bottom: 0.2rem;",
                                "Local-first posture"
                            }
                            p {
                                style: "margin: 0; font-size: 0.7rem; color: var(--qualia-text-muted); line-height: 1.5;",
                                "Keep work close to the user while still leaving room for external services when they are explicitly chosen."
                            }
                        }
                        div {
                            style: "padding: 0.75rem 0.8rem; background: rgba(128,128,128,0.05); border: 1px solid var(--qualia-border); border-radius: 12px;",
                            div {
                                style: "font-size: 0.76rem; font-weight: 600; color: var(--qualia-text); margin-bottom: 0.2rem;",
                                "Provenance-aware tools"
                            }
                            p {
                                style: "margin: 0; font-size: 0.7rem; color: var(--qualia-text-muted); line-height: 1.5;",
                                "Show where outputs come from and preserve enough structure that interfaces can explain themselves."
                            }
                        }
                        div {
                            style: "padding: 0.75rem 0.8rem; background: rgba(128,128,128,0.05); border: 1px solid var(--qualia-border); border-radius: 12px;",
                            div {
                                style: "font-size: 0.76rem; font-weight: 600; color: var(--qualia-text); margin-bottom: 0.2rem;",
                                "Composable surfaces"
                            }
                            p {
                                style: "margin: 0; font-size: 0.7rem; color: var(--qualia-text-muted); line-height: 1.5;",
                                "Let dashboards, agents, and knowledge views coexist without forcing them into one rigid application model."
                            }
                        }
                    }
                }
            }
        }
    }
}
