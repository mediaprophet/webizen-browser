use dioxus::prelude::*;

#[component]
pub fn NeuroscienceQapp() -> Element {
    let mut level = use_signal(|| "Systems".to_string());
    let mut technique = use_signal(|| "fMRI".to_string());
    let mut brain_region = use_signal(|| "Prefrontal Cortex".to_string());
    let mut neurotransmitter = use_signal(|| "Glutamate".to_string());
    let mut firing_rate_hz = use_signal(|| 40.0f64);
    let mut membrane_potential_mv = use_signal(|| -70.0f64);
    let mut research_notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #b4befe; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Neuroscience QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Level of Analysis" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| level.set(e.value()),
                        option { "Molecular" }
                        option { "Cellular" }
                        option { "Circuit" }
                        option { "Systems" }
                        option { "Cognitive" }
                        option { "Computational" }
                        option { "Clinical" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Technique" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| technique.set(e.value()),
                        option { "fMRI" }
                        option { "EEG" }
                        option { "Patch Clamp" }
                        option { "Two-Photon Imaging" }
                        option { "Optogenetics" }
                        option { "CRISPR" }
                        option { "Diffusion Tensor Imaging" }
                        option { "MEG" }
                        option { "PET" }
                        option { "Single-unit recording" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Brain Region" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| brain_region.set(e.value()),
                        option { "Prefrontal Cortex" }
                        option { "Hippocampus" }
                        option { "Amygdala" }
                        option { "Cerebellum" }
                        option { "Basal Ganglia" }
                        option { "Brainstem" }
                        option { "Thalamus" }
                        option { "Hypothalamus" }
                        option { "Motor Cortex" }
                        option { "Visual Cortex" }
                        option { "Corpus Callosum" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Neurotransmitter" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| neurotransmitter.set(e.value()),
                        option { "Glutamate" }
                        option { "GABA" }
                        option { "Dopamine" }
                        option { "Serotonin" }
                        option { "Acetylcholine" }
                        option { "Norepinephrine" }
                        option { "Endocannabinoids" }
                        option { "Glycine" }
                        option { "Oxytocin" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Firing Rate (Hz): {firing_rate_hz:.1}" }
                    input {
                        r#type: "range",
                        min: "0.0",
                        max: "500.0",
                        step: "0.5",
                        value: "{firing_rate_hz}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| firing_rate_hz.set(e.value().parse().unwrap_or(40.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Membrane Potential (mV)" }
                    input {
                        r#type: "number",
                        value: "{membrane_potential_mv}",
                        step: "0.1",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| membrane_potential_mv.set(e.value().parse().unwrap_or(-70.0)),
                    }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Research Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Experimental design, subject details, stimulation protocols, findings...",
                    oninput: move |e| research_notes.set(e.value()),
                    "{research_notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #b4befe; flex: 1;",
                h3 { style: "margin-top: 0; color: #b4befe; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Level:" }
                    div { style: "color: #cdd6f4;", "{level}" }
                    div { style: "color: #a6adc8;", "Technique:" }
                    div { style: "color: #cdd6f4;", "{technique}" }
                    div { style: "color: #a6adc8;", "Brain Region:" }
                    div { style: "color: #cdd6f4;", "{brain_region}" }
                    div { style: "color: #a6adc8;", "Neurotransmitter:" }
                    div { style: "color: #cdd6f4;", "{neurotransmitter}" }
                    div { style: "color: #a6adc8;", "Firing Rate:" }
                    div { style: "color: #cdd6f4;", "{firing_rate_hz:.1} Hz" }
                    div { style: "color: #a6adc8;", "V_m:" }
                    div { style: "color: #cdd6f4;", "{membrane_potential_mv:.1} mV" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → ODE Hodgkin-Huxley solver | graph connectome | neuro-symbolic sieve"
                }
            }
        }
    }
}
