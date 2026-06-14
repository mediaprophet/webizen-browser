use dioxus::prelude::*;

#[component]
pub fn AestheticsQapp() -> Element {
    let mut aesthetic_theory = use_signal(|| "Kantian Disinterestedness".to_string());
    let mut aesthetic_property = use_signal(|| "Beauty".to_string());
    let mut art_form = use_signal(|| "Visual".to_string());
    let mut contemplative_mode = use_signal(|| "Contemplative".to_string());
    let mut formal_or_contextual = use_signal(|| "Formalist".to_string());
    let mut notes = use_signal(|| String::new());

    let theories = ["Kantian Disinterestedness", "Hegelian Spirit", "Schopenhauer Will", "Dewey Experience", "Adorno Negative Dialectics", "Danto Artworld", "Carroll Cognitive", "Dickie Institutional", "Phenomenological", "Analytic"];
    let properties = ["Beauty", "Sublimity", "Ugliness", "Grace", "Elegance", "Camp", "Kitsch", "Authenticity", "Originality", "Expression"];
    let art_forms = ["Visual", "Music", "Literature", "Film", "Architecture", "Dance", "Digital", "Nature"];
    let modes = ["Contemplative", "Critical", "Participatory", "Relational"];
    let formal_contextual = ["Formalist", "Contextualist", "Mixed"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Aesthetics" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Aesthetic Theory" }
                    select {
                        value: "{aesthetic_theory}",
                        onchange: move |e| aesthetic_theory.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in theories { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Aesthetic Property" }
                    select {
                        value: "{aesthetic_property}",
                        onchange: move |e| aesthetic_property.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in properties { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Art Form" }
                    select {
                        value: "{art_form}",
                        onchange: move |e| art_form.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in art_forms { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Contemplative Mode" }
                    select {
                        value: "{contemplative_mode}",
                        onchange: move |e| contemplative_mode.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in modes { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Formal or Contextual" }
                    select {
                        value: "{formal_or_contextual}",
                        onchange: move |e| formal_or_contextual.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in formal_contextual { option { value: "{x}", "{x}" } }
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
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #cba6f7;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{aesthetic_theory} | {aesthetic_property} | {art_form} | {formal_or_contextual}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → aesthetics engine | philosophical sieve | art theory anchor" }
            }
        }
    }
}
