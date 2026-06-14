use dioxus::prelude::*;

#[component]
pub fn CulturalStudiesQapp() -> Element {
    let mut theoretical_approach = use_signal(|| "Gramscian Hegemony".to_string());
    let mut cultural_form = use_signal(|| "Popular Media".to_string());
    let mut context = use_signal(|| String::new());
    let mut identity_axis = use_signal(|| "Class".to_string());
    let mut power_relation = use_signal(|| "Dominant / Subordinate".to_string());
    let mut discourse_fragment = use_signal(|| String::new());

    let approaches = [
        "Gramscian Hegemony", "Althusserian Ideology / ISA",
        "Foucauldian Discourse Analysis", "Birmingham CCCS",
        "Feminist Cultural Studies", "Postcolonial (Said, Bhabha, Spivak)",
        "Black Cultural Studies (Hall, Gilroy)", "Queer Theory (Butler)",
        "Affect Theory (Massumi)", "Semiotics (Barthes)",
        "Media Ecology (McLuhan)", "Postmodernism (Jameson)",
    ];
    let forms = [
        "Popular Media", "Social Media / Digital Culture", "Fashion",
        "Sport", "Music / Subculture", "Film & Television",
        "Advertising", "News / Journalism", "Everyday Life / Practice",
        "Ritual / Ceremony", "Architecture / Built Environment",
        "Food Culture", "Youth / Subculture",
    ];
    let id_axes = [
        "Class", "Race / Ethnicity", "Gender", "Sexuality",
        "Religion", "Nationality", "Age / Generation",
        "Disability", "Multiple / Intersecting",
    ];
    let power_rels = [
        "Dominant / Subordinate", "Hegemony / Counter-Hegemony",
        "Discourse / Counter-Discourse", "Centre / Margin",
        "Global / Local (Glocal)", "Coloniser / Colonised",
        "Normative / Deviant",
    ];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #f5c2e7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Cultural Studies" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Approach" }
                    select {
                        value: "{theoretical_approach}",
                        onchange: move |e| theoretical_approach.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in approaches { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Form / Object" }
                    select {
                        value: "{cultural_form}",
                        onchange: move |e| cultural_form.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in forms { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Identity Axis" }
                    select {
                        value: "{identity_axis}",
                        onchange: move |e| identity_axis.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in id_axes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Power Relation" }
                    select {
                        value: "{power_relation}",
                        onchange: move |e| power_relation.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for x in power_rels { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Cultural Context / Site" }
                input {
                    type: "text", placeholder: "e.g. post-Thatcher UK, TikTok algorithm culture, Afrobeats…",
                    value: "{context}",
                    oninput: move |e| context.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Discourse Fragment / Text for Analysis" }
                textarea {
                    value: "{discourse_fragment}",
                    oninput: move |e| discourse_fragment.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 80px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #f5c2e7; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_approach}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{cultural_form}" }
                span { style: "font-size: 0.8rem; color: #f5c2e7;", "{power_relation}" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → discourse analysis sieve | neuro-symbolic | knowledge graph" }
            }
        }
    }
}
