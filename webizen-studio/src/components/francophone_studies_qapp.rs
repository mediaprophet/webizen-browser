use dioxus::prelude::*;

#[component]
pub fn FrancophonieStudiesQapp() -> Element {
    let mut region = use_signal(|| "France".to_string());
    let mut period = use_signal(|| "Postcolonial".to_string());
    let mut disciplinary_lens = use_signal(|| "Literature".to_string());
    let mut theoretical_lens = use_signal(|| "Postcolonial".to_string());
    let mut language_register = use_signal(|| "Standard French".to_string());
    let mut notes = use_signal(|| String::new());

    let regions = ["France", "Belgium", "Switzerland", "Quebec", "West Africa", "North Africa (Maghreb)", "Sub-Saharan Francophone", "Caribbean", "Pacific", "Indian Ocean"];
    let periods = ["Colonial", "Decolonisation", "Postcolonial", "Contemporary"];
    let lenses = ["Literature", "History", "Linguistics", "Film Studies", "Political Economy", "Cultural Studies"];
    let theoretical_lenses = ["Négritude", "Creolisation", "Francophonie Institutions", "Postcolonial", "Migrant Literature"];
    let registers = ["Standard French", "Regional Variety", "Creole", "Verlan", "Code-Switching"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Francophonie Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Region" }
                    select {
                        value: "{region}",
                        onchange: move |e| region.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in regions { option { value: "{x}", "{x}" } }
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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                    select {
                        value: "{theoretical_lens}",
                        onchange: move |e| theoretical_lens.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in theoretical_lenses { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Language Register" }
                    select {
                        value: "{language_register}",
                        onchange: move |e| language_register.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in registers { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{region} | {period} | {disciplinary_lens} | {theoretical_lens} | {language_register}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → francophone corpus | creolisation engine | postcolonial sieve" }
            }
        }
    }
}
