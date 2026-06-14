use dioxus::prelude::*;

#[component]
pub fn DeafStudiesQapp() -> Element {
    let mut identity_model = use_signal(|| "Deaf Culture".to_string());
    let mut sign_language = use_signal(|| "ASL".to_string());
    let mut communication_mode = use_signal(|| "Sign Language".to_string());
    let mut educational_setting = use_signal(|| "Residential School".to_string());
    let mut community_access = use_signal(|| 65u32);
    let mut notes = use_signal(|| String::new());

    let identity_models = ["Deaf Culture", "Hearing Loss/Disability", "Bilingual-Bicultural", "Deafblind", "Late-Deafened", "Oral Deaf"];
    let sign_languages = ["ASL", "BSL", "Auslan", "LSF", "DGS", "ISL", "Other"];
    let comm_modes = ["Sign Language", "Oral", "Cued Speech", "Total Communication", "Written"];
    let edu_settings = ["Residential School", "Mainstream", "Bilingual", "Home"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 { style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Deaf Studies" }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Identity Model" }
                select {
                    value: "{identity_model}",
                    onchange: move |e| identity_model.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in identity_models { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Sign Language" }
                select {
                    value: "{sign_language}",
                    onchange: move |e| sign_language.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in sign_languages { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Communication Mode" }
                select {
                    value: "{communication_mode}",
                    onchange: move |e| communication_mode.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in comm_modes { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Educational Setting" }
                select {
                    value: "{educational_setting}",
                    onchange: move |e| educational_setting.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in edu_settings { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Community Access: {community_access}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{community_access}",
                    oninput: move |e| community_access.set(e.value().parse().unwrap_or(65)),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{identity_model} | {sign_language} | {communication_mode} | {educational_setting} | Access: {community_access}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
