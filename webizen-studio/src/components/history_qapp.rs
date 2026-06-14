use dioxus::prelude::*;

#[component]
pub fn HistoryQapp() -> Element {
    let mut period = use_signal(|| "Modern (1500–1900)".to_string());
    let mut region = use_signal(|| "Europe".to_string());
    let mut historiographic_school = use_signal(|| "Social History (Annales)".to_string());
    let mut primary_source = use_signal(|| String::new());
    let mut event_start = use_signal(|| 1789i32);
    let mut event_end = use_signal(|| 1799i32);
    let mut causal_factor = use_signal(|| "Economic".to_string());
    let mut research_notes = use_signal(|| String::new());

    let periods = [
        "Ancient (pre-500 BCE)", "Classical Antiquity (500 BCE–500 CE)",
        "Late Antiquity (200–700)", "Medieval (500–1500)",
        "Early Modern (1500–1800)", "Modern (1500–1900)",
        "19th Century", "20th Century", "Contemporary (1945–)",
        "Deep History / World History",
    ];
    let regions = [
        "Europe", "East Asia", "South Asia", "Southeast Asia",
        "Middle East / MENA", "Sub-Saharan Africa", "North Africa",
        "North America", "Latin America / Caribbean",
        "Oceania / Pacific", "Central Asia", "Global / Transnational",
    ];
    let schools = [
        "Social History (Annales)", "Political / Diplomatic History",
        "Economic History", "Cultural History", "Intellectual History",
        "Microhistory", "World / Global History", "Postcolonial History",
        "Women's & Gender History", "Environmental History",
        "History from Below", "Quantitative / Cliometrics",
    ];
    let causes = [
        "Economic", "Political", "Religious / Ideological",
        "Military / Strategic", "Environmental / Climatic",
        "Demographic", "Technological", "Cultural",
        "Structural / Systemic", "Contingent / Individual Agency",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "History — Historiographic Workbench" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historical Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in periods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Region / Geography" }
                    select {
                        value: "{region}",
                        onchange: move |e| region.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in regions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Historiographic School" }
                    select {
                        value: "{historiographic_school}",
                        onchange: move |e| historiographic_school.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in schools { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Event Start (CE)" }
                    input {
                        type: "number",
                        value: "{event_start}",
                        oninput: move |e| event_start.set(e.value().parse().unwrap_or(1789)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Event End (CE)" }
                    input {
                        type: "number",
                        value: "{event_end}",
                        oninput: move |e| event_end.set(e.value().parse().unwrap_or(1799)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Primary Causal Factor" }
                    select {
                        value: "{causal_factor}",
                        onchange: move |e| causal_factor.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in causes { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Primary Source Reference" }
                input {
                    type: "text", placeholder: "e.g. Cahiers de doléances, Domesday Book, CO 137/…",
                    value: "{primary_source}",
                    oninput: move |e| primary_source.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Research & Interpretation Notes" }
                textarea {
                    value: "{research_notes}",
                    oninput: move |e| research_notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #cba6f7; font-weight: bold;", "{event_start}–{event_end}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{region}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{historiographic_school}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Cause: {causal_factor}" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → Allen Interval Algebra | historiography_mapper | knowledge graph" }
            }
        }
    }
}
