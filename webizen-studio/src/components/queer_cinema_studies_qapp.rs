use dioxus::prelude::*;

#[component]
pub fn QueerCinemaStudiesQapp() -> Element {
    let mut queer_cinema_category = use_signal(|| "New Queer Cinema".to_string());
    let mut director_tradition = use_signal(|| "Fassbinder".to_string());
    let mut theoretical_lens = use_signal(|| "Queer Theory".to_string());
    let mut subversion_index = use_signal(|| 50u32);
    let mut visibility_politics = use_signal(|| 50u32);
    let mut notes = use_signal(|| String::new());

    let categories = ["New Queer Cinema", "Camp", "Homoerotic", "Trans Cinema", "Queer Horror", "Queer Comedy", "Queer Documentary"];
    let directors = ["Fassbinder", "Almodóvar", "Waters", "Haynes", "Campion", "Wachowskis"];
    let lenses = ["Queer Theory", "Psychoanalysis", "Affect Theory", "Phenomenology", "Camp Aesthetics"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Queer Cinema Studies" }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Queer Cinema Category" }
                select {
                    value: "{queer_cinema_category}",
                    onchange: move |e| queer_cinema_category.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in categories { option { value: "{x}", "{x}" } }
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Director Tradition" }
                select {
                    value: "{director_tradition}",
                    onchange: move |e| director_tradition.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in directors { option { value: "{x}", "{x}" } }
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
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Subversion Index: {subversion_index}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{subversion_index}",
                    oninput: move |e| subversion_index.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Visibility Politics: {visibility_politics}" }
                input {
                    r#type: "range", min: "0", max: "100", value: "{visibility_politics}",
                    oninput: move |e| visibility_politics.set(e.value().parse().unwrap_or(50)),
                    style: "width: 100%; margin-top: 4px;"
                }
            }
            div {
                style: "flex: 1; display: flex; flex-direction: column;",
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    value: "{notes}",
                    oninput: move |e| notes.set(e.value()),
                    style: "flex: 1; width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; resize: none; box-sizing: border-box; min-height: 60px;"
                }
            }
            div {
                style: "background: #11111b; padding: 12px 16px; border-radius: 8px; border-left: 4px solid #94e2d5;",
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{queer_cinema_category} | {director_tradition} | Subversion: {subversion_index} | Visibility: {visibility_politics}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → queer cinema engine | camp sieve | visibility anchor" }
            }
        }
    }
}
