use dioxus::prelude::*;

#[component]
pub fn SociologyQapp() -> Element {
    let mut theoretical_tradition = use_signal(|| "Structural Functionalism".to_string());
    let mut level = use_signal(|| "Meso (Institutions)".to_string());
    let mut method = use_signal(|| "Survey / Quantitative".to_string());
    let mut social_phenomenon = use_signal(|| String::new());
    let mut stratification_axis = use_signal(|| "Class".to_string());
    let mut mobility_type = use_signal(|| "Intergenerational".to_string());
    let mut gini = use_signal(|| 0.35f64);
    let mut analysis_notes = use_signal(|| String::new());

    let traditions = [
        "Structural Functionalism (Parsons)", "Conflict Theory (Marx, Dahrendorf)",
        "Symbolic Interactionism (Mead, Blumer)", "Phenomenological Sociology (Schutz)",
        "Critical Theory (Frankfurt School)", "Feminist Sociology",
        "Postmodern Sociology (Baudrillard)", "Network Sociology",
        "Rational Choice / Exchange Theory",
        "Practice Theory (Bourdieu)", "World-Systems Theory (Wallerstein)",
    ];
    let levels = [
        "Micro (Individual / Interaction)", "Meso (Institutions)", "Macro (Society / Global)",
    ];
    let methods = [
        "Survey / Quantitative", "Ethnography", "Focus Groups",
        "In-Depth Interview", "Content Analysis", "Social Network Analysis",
        "Historical / Comparative", "Mixed Methods",
    ];
    let strat_axes = [
        "Class", "Race / Ethnicity", "Gender", "Age / Generation",
        "Sexuality", "Disability", "Religion", "Caste",
        "Intersectional (Multiple Axes)",
    ];
    let mobility_types = [
        "Intergenerational", "Intragenerational", "Structural",
        "Exchange", "Upward", "Downward", "Lateral",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Sociology" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Tradition" }
                    select {
                        value: "{theoretical_tradition}",
                        onchange: move |e| theoretical_tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Level of Analysis" }
                    select {
                        value: "{level}",
                        onchange: move |e| level.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in levels { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Research Method" }
                    select {
                        value: "{method}",
                        onchange: move |e| method.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in methods { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Stratification Axis" }
                    select {
                        value: "{stratification_axis}",
                        onchange: move |e| stratification_axis.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in strat_axes { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Social Mobility Type" }
                    select {
                        value: "{mobility_type}",
                        onchange: move |e| mobility_type.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in mobility_types { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Gini Coefficient: {gini:.2}" }
                    input {
                        type: "range", min: "0", max: "1", step: "0.01",
                        value: "{gini}",
                        oninput: move |e| gini.set(e.value().parse().unwrap_or(0.35)),
                        style: "width: 100%; margin-top: 10px;"
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Social Phenomenon / Research Question" }
                input {
                    type: "text", placeholder: "e.g. anomie in post-industrial cities, digital stratification…",
                    value: "{social_phenomenon}",
                    oninput: move |e| social_phenomenon.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Analysis Notes" }
                textarea {
                    value: "{analysis_notes}",
                    oninput: move |e| analysis_notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{level}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Stratification: {stratification_axis}" }
                span { style: "font-size: 0.8rem; color: #fab387; font-weight: bold;", "Gini: {gini:.2}" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → graph theory | statistical engine | social network sieve" }
            }
        }
    }
}
