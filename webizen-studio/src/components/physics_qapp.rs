use dioxus::prelude::*;

#[component]
pub fn PhysicsQapp() -> Element {
    let mut subdiscipline = use_signal(|| "Classical Mechanics".to_string());
    let mut formalism = use_signal(|| "Lagrangian".to_string());
    let mut energy_j = use_signal(|| 1.0f64);
    let mut momentum_kgms = use_signal(|| 0.0f64);
    let mut wavelength_nm = use_signal(|| 500.0f64);
    let mut notes = use_signal(|| String::new());

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",
            h2 { style: "margin: 0; color: #89b4fa; border-bottom: 1px solid #313244; padding-bottom: 8px;", "Physics QApp" }

            div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 12px;",
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Subdiscipline" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| subdiscipline.set(e.value()),
                        option { "Classical Mechanics" }
                        option { "Electromagnetism" }
                        option { "Thermodynamics" }
                        option { "Quantum Mechanics" }
                        option { "Special Relativity" }
                        option { "General Relativity" }
                        option { "Particle Physics" }
                        option { "Condensed Matter" }
                        option { "Optics" }
                        option { "Nuclear" }
                        option { "Plasma Physics" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Formalism" }
                    select {
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        onchange: move |e| formalism.set(e.value()),
                        option { "Newtonian" }
                        option { "Lagrangian" }
                        option { "Hamiltonian" }
                        option { "Path Integral" }
                        option { "Statistical Mechanical" }
                        option { "Quantum Field Theory" }
                        option { "Geometric / Differential" }
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Energy (J)" }
                    input {
                        r#type: "number",
                        value: "{energy_j}",
                        step: "0.001",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| energy_j.set(e.value().parse().unwrap_or(1.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Momentum (kg·m/s)" }
                    input {
                        r#type: "number",
                        value: "{momentum_kgms}",
                        step: "0.001",
                        style: "width: 100%; padding: 6px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                        oninput: move |e| momentum_kgms.set(e.value().parse().unwrap_or(0.0)),
                    }
                }
                div {
                    label { style: "font-size: 0.8rem; color: #a6adc8;", "Wavelength (nm): {wavelength_nm:.1}" }
                    input {
                        r#type: "range",
                        min: "1",
                        max: "10000",
                        step: "1",
                        value: "{wavelength_nm}",
                        style: "width: 100%; margin-top: 8px;",
                        oninput: move |e| wavelength_nm.set(e.value().parse().unwrap_or(500.0)),
                    }
                }
                div {
                    style: "background: #181825; border: 1px solid #45475a; border-radius: 4px; padding: 8px; margin-top: 4px;",
                    div { style: "font-size: 0.75rem; color: #a6adc8;", "Planck's Constant" }
                    div { style: "font-size: 0.85rem; color: #f9e2af; margin-top: 4px;", "h = 6.626 × 10⁻³⁴ J·s" }
                    div { style: "font-size: 0.75rem; color: #a6adc8; margin-top: 4px;", "c = 2.998 × 10⁸ m/s" }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Notes" }
                textarea {
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; min-height: 60px; box-sizing: border-box; resize: vertical;",
                    placeholder: "Equations of motion, boundary conditions, approximations, symmetries...",
                    oninput: move |e| notes.set(e.value()),
                    "{notes}"
                }
            }

            div {
                style: "background: #11111b; padding: 16px; border-radius: 8px; border-left: 4px solid #89b4fa; flex: 1;",
                h3 { style: "margin-top: 0; color: #89b4fa; font-size: 0.9rem;", "Output / Analysis" }
                div { style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; font-size: 0.8rem;",
                    div { style: "color: #a6adc8;", "Subdiscipline:" }
                    div { style: "color: #cdd6f4;", "{subdiscipline}" }
                    div { style: "color: #a6adc8;", "Formalism:" }
                    div { style: "color: #cdd6f4;", "{formalism}" }
                    div { style: "color: #a6adc8;", "Energy:" }
                    div { style: "color: #cdd6f4;", "{energy_j:.4} J" }
                    div { style: "color: #a6adc8;", "Momentum:" }
                    div { style: "color: #cdd6f4;", "{momentum_kgms:.4} kg·m/s" }
                    div { style: "color: #a6adc8;", "Wavelength:" }
                    div { style: "color: #cdd6f4;", "{wavelength_nm:.1} nm" }
                }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 12px; border-top: 1px solid #313244; padding-top: 8px;",
                    "QualiaDB → ODE numerical solver | quantum DFT engine | Allen Interval Algebra"
                }
            }
        }
    }
}
