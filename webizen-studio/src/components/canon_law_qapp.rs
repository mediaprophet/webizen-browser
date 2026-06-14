use dioxus::prelude::*;

#[component]
pub fn CanonLawQapp() -> Element {
    let mut tradition = use_signal(|| "Roman Catholic".to_string());
    let mut code_or_source = use_signal(|| "Code of Canon Law 1983".to_string());
    let mut legal_domain = use_signal(|| "Marriage".to_string());
    let mut jurisdiction = use_signal(|| "Particular Church".to_string());
    let mut case_type = use_signal(|| String::new());
    let mut procedural_norm = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let traditions = ["Roman Catholic", "Eastern Catholic", "Eastern Orthodox", "Anglican", "Protestant Polity"];
    let codes = ["Code of Canon Law 1983", "Code of Canons of Eastern Churches", "Apostolic Constitutions", "Synodal Decisions", "Church Order"];
    let domains = ["Marriage", "Orders", "Sacraments", "Governance", "Penalties", "Temporal Goods", "Teaching Office", "Associations"];
    let jurisdictions = ["Universal", "Particular Church", "Diocese", "Religious Institute"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Canon Law" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Code / Source" }
                    select {
                        value: "{code_or_source}",
                        onchange: move |e| code_or_source.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in codes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Legal Domain" }
                    select {
                        value: "{legal_domain}",
                        onchange: move |e| legal_domain.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in domains { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Jurisdiction" }
                    select {
                        value: "{jurisdiction}",
                        onchange: move |e| jurisdiction.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in jurisdictions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Case Type" }
                    input {
                        r#type: "text",
                        value: "{case_type}",
                        oninput: move |e| case_type.set(e.value()),
                        placeholder: "e.g. nullity of marriage, dismissal from clerical state",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Procedural Norm" }
                    input {
                        r#type: "text",
                        value: "{procedural_norm}",
                        oninput: move |e| procedural_norm.set(e.value()),
                        placeholder: "e.g. Can. 1095, Can. 1671",
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #eba0ac;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{tradition} | {code_or_source} | {legal_domain} | {jurisdiction}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → canon law engine | jurisdictional sieve | procedural anchor" }
            }
        }
    }
}
