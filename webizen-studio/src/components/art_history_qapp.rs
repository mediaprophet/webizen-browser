use dioxus::prelude::*;

#[component]
pub fn ArtHistoryQapp() -> Element {
    let mut period = use_signal(|| "Renaissance".to_string());
    let mut attribution_certainty = use_signal(|| 75u32);
    let mut iconographic_query = use_signal(|| String::new());
    let mut movement_start = use_signal(|| 1400i32);
    let mut movement_end = use_signal(|| 1600i32);
    let mut style_tag = use_signal(|| "Chiaroscuro".to_string());

    let periods = ["Ancient", "Medieval", "Renaissance", "Baroque", "Neoclassicism", "Romanticism", "Impressionism", "Modernism", "Contemporary"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Art History" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Art Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for p in periods {
                            option { value: "{p}", "{p}" }
                        }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Style / Technique" }
                    input {
                        type: "text",
                        value: "{style_tag}",
                        oninput: move |e| style_tag.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Iconographic Query (QualiaDB Semantic Search)" }
                input {
                    type: "text",
                    placeholder: "e.g. Madonna and Child, vanitas symbols, heroic nude…",
                    value: "{iconographic_query}",
                    oninput: move |e| iconographic_query.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Movement Start (CE)" }
                    input {
                        type: "number",
                        value: "{movement_start}",
                        oninput: move |e| movement_start.set(e.value().parse().unwrap_or(1400)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Movement End (CE)" }
                    input {
                        type: "number",
                        value: "{movement_end}",
                        oninput: move |e| movement_end.set(e.value().parse().unwrap_or(1600)),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Attribution Certainty: {attribution_certainty}%" }
                input {
                    type: "range", min: "0", max: "100",
                    value: "{attribution_certainty}",
                    oninput: move |e| attribution_certainty.set(e.value().parse().unwrap_or(75)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #cba6f7; flex: 1;",
                h3 { style: "margin-top: 0; color: #cba6f7; font-size: 0.9rem;", "Allen Interval — Movement Span" }
                div {
                    style: "font-size: 0.85rem; color: #a6adc8; margin-bottom: 8px;",
                    "Period: {period} | Duration: {movement_end() - movement_start()} years"
                }
                div {
                    style: "position: relative; height: 24px; background: #181825; border-radius: 4px; overflow: hidden;",
                    div {
                        style: "position: absolute; top: 0; left: 5%; right: 5%; height: 100%; background: linear-gradient(90deg, #cba6f7, #89b4fa); border-radius: 4px; display: flex; align-items: center; justify-content: center; font-size: 0.75rem; color: #1e1e2e; font-weight: bold;",
                        "{period}: {movement_start}–{movement_end}"
                    }
                }
                div {
                    style: "margin-top: 12px; font-size: 0.8rem; color: #6c7086;",
                    "Attribution confidence (epistemic): {attribution_certainty}% | Style: {style_tag}"
                }
            }
        }
    }
}
