use dioxus::prelude::*;

#[component]
pub fn QueerStudiesQapp() -> Element {
    let mut theoretical_framework = use_signal(|| "Queer Theory (Butler)".to_string());
    let mut identity_focus = use_signal(|| "Queer".to_string());
    let mut method = use_signal(|| "Discourse Analysis".to_string());
    let mut historical_period = use_signal(|| "Contemporary".to_string());
    let mut legal_context = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let frameworks = [
        "Queer Theory (Butler)", "Queer of Colour Critique", "Homonationalism Critique",
        "Trans* Studies", "Intersectional Queer", "Crip Queer",
        "Diasporic Queer", "Queer Phenomenology",
    ];
    let identities = [
        "Gay", "Lesbian", "Bisexual", "Trans", "Non-Binary",
        "Asexual", "Queer", "Two-Spirit", "Multiple",
    ];
    let methods = [
        "Discourse Analysis", "Autoethnography", "Archival",
        "Oral History", "Textual Analysis", "Policy Analysis",
    ];
    let periods = [
        "Pre-Stonewall", "Stonewall Era", "AIDS Crisis",
        "Post-1990s Theory", "Contemporary",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Queer Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Framework" }
                    select {
                        value: "{theoretical_framework}",
                        onchange: move |e| theoretical_framework.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in frameworks { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Identity Focus" }
                    select {
                        value: "{identity_focus}",
                        onchange: move |e| identity_focus.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in identities { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Method" }
                    select {
                        value: "{method}",
                        onchange: move |e| method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historical Period" }
                    select {
                        value: "{historical_period}",
                        onchange: move |e| historical_period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in periods { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Legal Context" }
                input {
                    r#type: "text",
                    value: "{legal_context}",
                    oninput: move |e| legal_context.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f38ba8;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_framework} | {identity_focus} | {method} | {historical_period}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → queer theory engine | intersectional sieve | performativity graph" }
            }
        }
    }
}
