use dioxus::prelude::*;

#[component]
pub fn CriticalFilmStudiesQapp() -> Element {
    let mut theoretical_approach = use_signal(|| "Psychoanalytic".to_string());
    let mut film_form = use_signal(|| "Narrative".to_string());
    let mut gaze_type = use_signal(|| "Male Gaze".to_string());
    let mut ideology_index = use_signal(|| 50u32);
    let mut spectator_position = use_signal(|| "Dominant".to_string());
    let mut notes = use_signal(|| String::new());

    let theoretical_approaches = ["Psychoanalytic", "Feminist", "Postcolonial", "Queer Theory", "Affect Theory", "Genre Theory", "Auteur"];
    let film_forms = ["Narrative", "Documentary", "Experimental", "Animation", "Essay Film", "Found Footage"];
    let gaze_types = ["Male Gaze", "Imperial Gaze", "Tourist Gaze", "Surveillant"];
    let spectator_positions = ["Dominant", "Negotiated", "Oppositional"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #fab387; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Critical Film Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Approach" }
                select {
                    value: "{theoretical_approach}", onchange: move |e| theoretical_approach.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in theoretical_approaches { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Film Form" }
                select {
                    value: "{film_form}", onchange: move |e| film_form.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in film_forms { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Gaze Type" }
                select {
                    value: "{gaze_type}", onchange: move |e| gaze_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in gaze_types { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Ideology Index: {ideology_index}" }
                input { r#type: "range", min: "0", max: "100", value: "{ideology_index}",
                    oninput: move |e| ideology_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;" }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Spectator Position" }
                select {
                    value: "{spectator_position}", onchange: move |e| spectator_position.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in spectator_positions { option { value: "{x}", "{x}" } }
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea { value: "{notes}", oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;" }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #fab387;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{theoretical_approach} | {film_form} | {gaze_type} | {spectator_position} | ideology: {ideology_index}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → film studies engine | discourse sieve | anchor" }
            }
        }
    }
}
