use dioxus::prelude::*;

#[component]
pub fn PoliticalEconomyQapp() -> Element {
    let mut theoretical_school = use_signal(|| "Keynesian".to_string());
    let mut varieties_of_capitalism = use_signal(|| "Coordinated Market".to_string());
    let mut political_regime = use_signal(|| "Democracy".to_string());
    let mut key_variable = use_signal(|| "Trade Balance".to_string());
    let mut gdp_growth_pct = use_signal(|| 2.5f64);
    let mut trade_openness = use_signal(|| 0.5f64);
    let mut notes = use_signal(|| String::new());

    let schools = [
        "Classical (Smith/Ricardo)", "Marxian", "Keynesian", "Institutionalist",
        "Varieties of Capitalism", "World-Systems", "Feminist Political Economy",
        "Neoliberal Critique", "Ecological Political Economy",
    ];
    let voc = [
        "Liberal Market", "Coordinated Market", "Mixed", "State-Led", "Dependent",
    ];
    let regimes = ["Democracy", "Hybrid", "Autocracy"];
    let variables = [
        "Trade Balance", "Current Account", "Public Debt %GDP",
        "Wage Share", "Profit Share",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #eba0ac; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Political Economy" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical School" }
                    select {
                        value: "{theoretical_school}",
                        onchange: move |e| theoretical_school.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in schools { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Varieties of Capitalism" }
                    select {
                        value: "{varieties_of_capitalism}",
                        onchange: move |e| varieties_of_capitalism.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in voc { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Political Regime" }
                    select {
                        value: "{political_regime}",
                        onchange: move |e| political_regime.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in regimes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Key Variable" }
                    select {
                        value: "{key_variable}",
                        onchange: move |e| key_variable.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in variables { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "GDP Growth %: {gdp_growth_pct:.1}" }
                    input {
                        r#type: "number",
                        step: "0.1",
                        value: "{gdp_growth_pct}",
                        oninput: move |e| gdp_growth_pct.set(e.value().parse().unwrap_or(2.5)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Trade Openness (0.0–1.0): {trade_openness:.2}" }
                    input {
                        r#type: "range",
                        min: "0.0",
                        max: "1.0",
                        step: "0.01",
                        value: "{trade_openness}",
                        oninput: move |e| trade_openness.set(e.value().parse().unwrap_or(0.5)),
                        style: "width: 100%; margin-top: 4px;"
                    }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #eba0ac;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_school} | {varieties_of_capitalism} | {political_regime} | GDP growth: {gdp_growth_pct:.1}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → comparative political economy engine | regime classification sieve | institutional graph" }
            }
        }
    }
}
