use dioxus::prelude::*;

#[component]
pub fn SoutheastAsianStudiesQapp() -> Element {
    let mut subregion = use_signal(|| "Mainland SEA".to_string());
    let mut period = use_signal(|| "ASEAN Era".to_string());
    let mut disciplinary_lens = use_signal(|| "History".to_string());
    let mut colonial_power = use_signal(|| "None".to_string());
    let mut religion_or_philosophy = use_signal(|| "Theravada Buddhism".to_string());
    let mut notes = use_signal(|| String::new());

    let subregions = ["Mainland SEA", "Insular SEA", "Vietnam", "Thailand", "Indonesia", "Philippines", "Myanmar", "Malaysia", "Cambodia", "Laos", "Timor-Leste"];
    let periods = ["Pre-Colonial", "Colonial", "Independence", "Cold War", "ASEAN Era", "Contemporary"];
    let lenses = ["History", "Political Economy", "Anthropology", "Literature", "Religion", "Environmental", "Gender Studies"];
    let colonial_powers = ["Dutch", "French", "British", "Spanish", "American", "Japanese", "None"];
    let religions = ["Theravada Buddhism", "Islam", "Hinduism", "Catholicism", "Animism", "Confucianism"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Southeast Asian Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subregion" }
                    select {
                        value: "{subregion}",
                        onchange: move |e| subregion.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in subregions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in periods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Disciplinary Lens" }
                    select {
                        value: "{disciplinary_lens}",
                        onchange: move |e| disciplinary_lens.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in lenses { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Colonial Power" }
                    select {
                        value: "{colonial_power}",
                        onchange: move |e| colonial_power.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in colonial_powers { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Religion / Philosophy" }
                    select {
                        value: "{religion_or_philosophy}",
                        onchange: move |e| religion_or_philosophy.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in religions { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #94e2d5;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{subregion} | {period} | {colonial_power} | {religion_or_philosophy}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → SEA region engine | colonial history sieve | religion anchor" }
            }
        }
    }
}
