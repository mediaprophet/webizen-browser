use dioxus::prelude::*;

#[component]
pub fn EuropeanStudiesQapp() -> Element {
    let mut subregion = use_signal(|| "Western".to_string());
    let mut period = use_signal(|| "Contemporary EU".to_string());
    let mut analytical_lens = use_signal(|| "Political Economy".to_string());
    let mut primary_source_type = use_signal(|| "Treaty".to_string());
    let mut country_focus = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box; overflow-y: auto;",
            h2 { style: "margin: 0; color: #f38ba8; border-bottom: 1px solid #313244; padding-bottom: 8px;", "European Studies QApp" }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Sub-region" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| subregion.set(e.value()),
                    option { "Northern" }
                    option { "Southern" }
                    option { "Eastern" }
                    option { selected: true, "Western" }
                    option { "Central" }
                    option { "Balkan" }
                    option { "Nordic" }
                    option { "EU as Institution" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Period" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| period.set(e.value()),
                    option { "Ancient Rome" }
                    option { "Medieval" }
                    option { "Early Modern" }
                    option { "Enlightenment" }
                    option { "19th C." }
                    option { "Interwar" }
                    option { "Cold War" }
                    option { "Post-1989" }
                    option { selected: true, "Contemporary EU" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Analytical Lens" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| analytical_lens.set(e.value()),
                    option { selected: true, "Political Economy" }
                    option { "Cultural History" }
                    option { "Security Studies" }
                    option { "Migration" }
                    option { "Populism" }
                    option { "Environmental Policy" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Primary Source Type" }
                select {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    onchange: move |e| primary_source_type.set(e.value()),
                    option { selected: true, "Treaty" }
                    option { "Directive" }
                    option { "Literature" }
                    option { "Film" }
                    option { "Oral History" }
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Country Focus" }
                input {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box;",
                    placeholder: "e.g. Germany, France, Poland...",
                    oninput: move |e| country_focus.set(e.value()),
                }
            }

            div { style: "display: flex; flex-direction: column; gap: 8px;",
                label { style: "font-size: 0.85rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "background: #313244; color: #cdd6f4; border: 1px solid #45475a; border-radius: 6px; padding: 6px 8px; font-family: monospace; width: 100%; box-sizing: border-box; min-height: 60px; resize: vertical;",
                    placeholder: "Additional notes...",
                    oninput: move |e| notes.set(e.value()),
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #f38ba8; flex: 1;",
                h3 { style: "margin-top: 0; color: #f38ba8; font-size: 0.9rem;", "QualiaDB Output" }
                div { style: "font-size: 0.8rem; color: #a6adc8; display: flex; flex-direction: column; gap: 4px;",
                    div { "Sub-region: {subregion()}" }
                    div { "Period: {period()}" }
                    div { "Lens: {analytical_lens()}" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 8px;", "QualiaDB → Allen Interval | knowledge graph | policy sieve" }
            }
        }
    }
}
