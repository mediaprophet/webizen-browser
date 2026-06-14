use dioxus::prelude::*;

#[component]
pub fn GermanicLanguagesAndLiteraturesQapp() -> Element {
    let mut language = use_signal(|| "Modern German".to_string());
    let mut period = use_signal(|| "Post-WWII".to_string());
    let mut genre = use_signal(|| "Novel".to_string());
    let mut theoretical_lens = use_signal(|| "Frankfurt School".to_string());
    let mut author_or_text = use_signal(|| String::new());
    let mut notes = use_signal(|| String::new());

    let languages = ["Old High German", "Middle High German", "Modern German", "Old English", "Old Norse", "Dutch", "Gothic", "Yiddish", "Afrikaans"];
    let periods = ["Medieval", "Early Modern", "Enlightenment", "Romantic", "Weimar", "Nazi Era", "Post-WWII", "Contemporary"];
    let genres = ["Epic", "Lyric", "Drama", "Novel", "Expressionist", "Existentialist", "Contemporary"];
    let lenses = ["Philological", "Frankfurt School", "Feminist", "Postcolonial", "Trauma Studies"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Germanic Languages & Literatures" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Language" }
                    select {
                        value: "{language}",
                        onchange: move |e| language.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in languages { option { value: "{x}", "{x}" } }
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
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Genre" }
                    select {
                        value: "{genre}",
                        onchange: move |e| genre.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in genres { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Theoretical Lens" }
                    select {
                        value: "{theoretical_lens}",
                        onchange: move |e| theoretical_lens.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in lenses { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Author or Text" }
                input {
                    r#type: "text",
                    value: "{author_or_text}",
                    oninput: move |e| author_or_text.set(e.value()),
                    placeholder: "e.g. Goethe Faust, Kafka Metamorphosis, Toni Morrison",
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{language} | {period} | {genre} | {theoretical_lens}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → germanophone corpus | philological engine | literary sieve" }
            }
        }
    }
}
