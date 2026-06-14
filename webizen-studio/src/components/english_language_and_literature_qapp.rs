use dioxus::prelude::*;

#[component]
pub fn EnglishLanguageAndLiteratureQapp() -> Element {
    let mut period = use_signal(|| "Victorian".to_string());
    let mut text = use_signal(|| String::new());
    let mut avg_sentence_len = use_signal(|| 0.0f64);
    let mut device = use_signal(|| "Metaphor".to_string());
    let mut genre = use_signal(|| "Novel".to_string());
    let mut author_query = use_signal(|| String::new());

    let periods = ["Old English (450–1150)", "Middle English (1150–1500)", "Early Modern (1500–1660)", "Restoration & 18th C.", "Romantic (1785–1830)", "Victorian (1830–1901)", "Modernism (1901–1945)", "Post-War (1945–1970)", "Contemporary (1970–)"];
    let devices = ["Metaphor", "Simile", "Alliteration", "Irony", "Synecdoche", "Anaphora", "Enjambment", "Stream of Consciousness", "Free Indirect Discourse"];
    let genres = ["Novel", "Short Story", "Poetry", "Drama", "Essay", "Life Writing", "Gothic", "Bildungsroman"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "English Language & Literature" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Literary Period" }
                    select {
                        value: "{period}",
                        onchange: move |e| period.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for p in periods { option { value: "{p}", "{p}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Genre" }
                    select {
                        value: "{genre}",
                        onchange: move |e| genre.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for g in genres { option { value: "{g}", "{g}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Literary Device Focus" }
                    select {
                        value: "{device}",
                        onchange: move |e| device.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px;",
                        for d in devices { option { value: "{d}", "{d}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Author / Text Search" }
                input {
                    type: "text",
                    placeholder: "e.g. George Eliot, Middlemarch, Emily Dickinson…",
                    value: "{author_query}",
                    oninput: move |e| author_query.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;"
                }
            }

            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;",
                    "Paste Text for Stylometric Analysis (avg sentence: {avg_sentence_len:.1} words)"
                }
                textarea {
                    value: "{text}",
                    oninput: move |e| {
                        let val = e.value();
                        let sentences: Vec<&str> = val.split(['.', '!', '?']).filter(|s| !s.trim().is_empty()).collect();
                        let total_words: usize = val.split_whitespace().count();
                        avg_sentence_len.set(if sentences.is_empty() { 0.0 } else { total_words as f64 / sentences.len() as f64 });
                        text.set(val);
                    },
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; font-family: Georgia, serif; line-height: 1.6; box-sizing: border-box; min-height: 100px;"
                }
            }

            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #89b4fa; display: flex; gap: 16px; flex-wrap: wrap;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Period: {period}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Genre: {genre}" }
                span { style: "font-size: 0.8rem; color: #a6adc8;", "Device: {device}" }
                span { style: "font-size: 0.8rem; color: #89b4fa;", "Avg sentence: {avg_sentence_len:.1}w" }
                div { style: "font-size: 0.75rem; color: #585b70; width: 100%;", "QualiaDB → stylometric engine | neuro-symbolic sieve" }
            }
        }
    }
}
