use dioxus::prelude::*;

#[component]
pub fn BioinformaticsQapp() -> Element {
    let mut analysis_type = use_signal(|| "Sequence Alignment".to_string());
    let mut algorithm = use_signal(|| "BLAST".to_string());
    let mut data_type = use_signal(|| "DNA".to_string());
    let mut seq_length_range = use_signal(|| "<10k bp".to_string());
    let mut accuracy = use_signal(|| 90u32);
    let mut notes = use_signal(|| String::new());

    let analysis_types = [
        "Sequence Alignment", "Genome Assembly", "Phylogenetics",
        "Protein Structure", "Gene Expression", "Variant Calling", "Metagenomics",
    ];
    let algorithms = [
        "BLAST", "Smith-Waterman", "Needleman-Wunsch",
        "Hidden Markov Model", "Random Forest", "Deep Learning",
    ];
    let data_types = ["DNA", "RNA", "Protein", "Epigenomic", "Metagenomic"];
    let length_ranges = ["<10k bp", "10k–100k bp", "100k–1M bp", "1M–10M bp", ">10M bp"];

    rsx! {
        div {
            style: "padding: 20px; background: #1e1e2e; color: #cdd6f4; border-radius: 12px; font-family: monospace; display: flex; flex-direction: column; gap: 16px; height: 100%; box-sizing: border-box;",

            h2 {
                style: "margin: 0; color: #94e2d5; border-bottom: 1px solid #313244; padding-bottom: 8px;",
                "Bioinformatics"
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Analysis Type" }
                select {
                    value: "{analysis_type}",
                    onchange: move |e| analysis_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in analysis_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Algorithm" }
                select {
                    value: "{algorithm}",
                    onchange: move |e| algorithm.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in algorithms { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Data Type" }
                select {
                    value: "{data_type}",
                    onchange: move |e| data_type.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in data_types { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Sequence Length Range" }
                select {
                    value: "{seq_length_range}",
                    onchange: move |e| seq_length_range.set(e.value()),
                    style: "width: 100%; padding: 8px; background: #181825; border: 1px solid #45475a; color: #cdd6f4; border-radius: 4px; margin-top: 4px; box-sizing: border-box;",
                    for x in length_ranges { option { value: "{x}", "{x}" } }
                }
            }

            div {
                label { style: "font-size: 0.8rem; color: #a6adc8;", "Accuracy: {accuracy}%" }
                input {
                    r#type: "range", min: "0", max: "100",
                    value: "{accuracy}",
                    oninput: move |e| accuracy.set(e.value().parse().unwrap_or(90)),
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
                span { style: "font-size: 0.8rem; color: #a6adc8;", "{analysis_type} | {algorithm} | {data_type} | {seq_length_range} | acc {accuracy}%" }
                div { style: "font-size: 0.75rem; color: #585b70; margin-top: 6px;", "QualiaDB → knowledge engine | sieve | anchor" }
            }
        }
    }
}
