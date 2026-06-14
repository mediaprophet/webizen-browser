use dioxus::prelude::*;

#[component]
pub fn AnimationAndDigitalArtsQapp() -> Element {
    let mut medium = use_signal(|| "2D Animation".to_string());
    let mut technique = use_signal(|| "Hand-Drawn".to_string());
    let mut tradition = use_signal(|| "Studio Ghibli".to_string());
    let mut frame_rate_fps = use_signal(|| 24u32);
    let mut resolution = use_signal(|| "HD".to_string());
    let mut pipeline_stage = use_signal(|| "Animation".to_string());
    let mut notes = use_signal(|| String::new());

    let mediums = ["2D Animation", "3D CGI", "Stop Motion", "Motion Graphics", "VFX", "Interactive Digital Art", "Generative Art", "NFT Art", "AR", "VR"];
    let techniques = ["Hand-Drawn", "Vector", "3D Modelling", "Procedural", "Particle Systems", "Neural Style Transfer"];
    let traditions = ["Disney", "Anime", "Studio Ghibli", "Soviet Animation", "Zagreb School", "Contemporary Indie"];
    let resolutions = ["SD", "HD", "4K", "8K"];
    let stages = ["Concept", "Storyboard", "Layout", "Animation", "Compositing", "Render"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #cba6f7; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Animation & Digital Arts" }

            div {
                style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Medium" }
                    select {
                        value: "{medium}",
                        onchange: move |e| medium.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in mediums { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Technique" }
                    select {
                        value: "{technique}",
                        onchange: move |e| technique.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in techniques { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Tradition" }
                    select {
                        value: "{tradition}",
                        onchange: move |e| tradition.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in traditions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Resolution" }
                    select {
                        value: "{resolution}",
                        onchange: move |e| resolution.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in resolutions { option { value: "{x}", "{x}" } }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Pipeline Stage" }
                    select {
                        value: "{pipeline_stage}",
                        onchange: move |e| pipeline_stage.set(e.value()),
                        style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        for x in stages { option { value: "{x}", "{x}" } }
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Frame Rate (fps): {frame_rate_fps}" }
                input {
                    r#type: "range",
                    min: "6",
                    max: "120",
                    value: "{frame_rate_fps}",
                    oninput: move |e| frame_rate_fps.set(e.value().parse().unwrap_or(24)),
                    style: "width: 100%; margin-top: 4px;"
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{medium} | {technique} | {tradition} | {frame_rate_fps}fps | {pipeline_stage}" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → animation engine | pipeline sieve | digital arts anchor" }
            }
        }
    }
}
