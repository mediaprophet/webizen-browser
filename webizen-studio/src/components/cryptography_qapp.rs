use dioxus::prelude::*;

#[component]
pub fn CryptographyQapp() -> Element {
    let mut paradigm = use_signal(|| "Symmetric".to_string());
    let mut algorithm = use_signal(|| "AES".to_string());
    let mut application = use_signal(|| "Secure Communication".to_string());
    let mut key_size = use_signal(|| "256".to_string());
    let mut security_level = use_signal(|| 80u32);
    let mut notes = use_signal(|| String::new());

    let paradigms = [
        "Symmetric", "Asymmetric", "Hash Function",
        "Zero-Knowledge", "Post-Quantum", "Homomorphic Encryption",
    ];
    let algorithms = ["AES", "RSA", "Elliptic Curve", "SHA-256", "ChaCha20", "Kyber", "BLAKE3"];
    let applications = [
        "Secure Communication", "Digital Signature", "Key Exchange",
        "Authentication", "Blockchain", "Privacy-Preserving Computation",
    ];
    let key_sizes = ["128", "256", "512", "1024", "2048", "4096"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #f9e2af; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Cryptography"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cryptographic Paradigm" }
                select {
                    value: "{paradigm}",
                    onchange: move |e| paradigm.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in paradigms { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Algorithm" }
                select {
                    value: "{algorithm}",
                    onchange: move |e| algorithm.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in algorithms { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Application" }
                select {
                    value: "{application}",
                    onchange: move |e| application.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in applications { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Key Size (bits)" }
                select {
                    value: "{key_size}",
                    onchange: move |e| key_size.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in key_sizes { option { value: "{x}", "{x} bits" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Security Level: {security_level}" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{security_level}",
                    oninput: move |e| security_level.set(e.value().parse().unwrap_or(80)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f9e2af;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{paradigm} | {algorithm} | {application} | {key_size}-bit | sec {security_level}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
