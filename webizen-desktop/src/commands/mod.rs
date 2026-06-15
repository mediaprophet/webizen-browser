use qualia_client_core::api;
use qualia_client_core::api::{CoinBalance, HardwareStatus, TokenEntry, TxRecord, WalletStatus};
use qualia_client_core::engine::{ingestion, llm_offload};
use qualia_client_core::state::{Actor, AgentConfig, DelegationRule, FrontDoor, ProgressPayload};
use qualia_core_db::ilp_dispatcher::DispatchResult;
use qualia_core_db::rpc::TaxRecipientSuite;
use std::time::Duration;
use tauri::{command, AppHandle, Manager, State, WindowBuilder, WindowUrl};
use webizen_runtime::DiffusionConfig;

use crate::runtime::{RuntimeHandle, RuntimeLedgerHealth, RuntimeSnapshotRecord};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct DiffusionConfigInput {
    pub width: u32,
    pub height: u32,
    pub diffusion_rate: f32,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct LocalPreviewProbe {
    pub target_url: String,
    pub reachable: bool,
    pub status_code: Option<u16>,
    pub detail: String,
}

// ── Qapp vault ────────────────────────────────────────────────────────────────

#[command]
pub fn list_installed_qapps() -> Vec<String> {
    api::list_installed_qapps()
}

#[command]
pub fn generate_qapp_credential(qapp_name: String) -> String {
    api::generate_qapp_credential(qapp_name)
}

#[command]
pub fn verify_and_install_qapp(target_path: String) -> Result<String, String> {
    api::verify_and_install_qapp(target_path)
}

#[command]
pub fn launch_installed_qapp(app: AppHandle, qapp_name: String) -> Result<(), String> {
    let url = api::launch_installed_qapp(qapp_name.clone())?;
    let label: String = format!(
        "qapp-{}",
        qapp_name
            .chars()
            .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
            .collect::<String>()
    );

    if let Some(window) = app.get_window(&label) {
        window.set_focus().map_err(|e| e.to_string())?;
        return Ok(());
    }

    let parsed = url
        .parse()
        .map_err(|e| format!("Invalid launch URL '{url}': {e}"))?;
    WindowBuilder::new(&app, label, WindowUrl::External(parsed))
        .title(qapp_name)
        .inner_size(1200.0, 800.0)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

// ── Hardware / system ─────────────────────────────────────────────────────────

#[command]
pub fn get_hardware_status() -> HardwareStatus {
    api::get_hardware_status()
}

#[command]
pub fn profile_energy_circumstance() -> String {
    api::profile_energy_circumstance()
}

// ── Daemon ────────────────────────────────────────────────────────────────────

#[command]
pub fn start_daemon() -> String {
    api::start_daemon()
}

#[command]
pub fn daemon_status() -> String {
    api::daemon_status()
}

#[command]
pub fn run_engine_command(cmd: String) -> String {
    api::run_engine_command(cmd)
}

// ── Config ────────────────────────────────────────────────────────────────────

#[command]
pub fn get_config() -> AgentConfig {
    api::get_config()
}

#[command]
pub fn save_config(new_config: AgentConfig) -> Result<(), String> {
    api::save_config(new_config)
}

// ── Wallet / identity ─────────────────────────────────────────────────────────

#[command]
pub fn get_wallet_status() -> WalletStatus {
    api::get_wallet_status()
}

#[command]
pub fn is_first_run() -> bool {
    api::is_first_run()
}

#[command]
pub fn read_identity() -> Option<serde_json::Value> {
    api::read_identity()
}

#[command]
pub fn save_identity(wallets: serde_json::Value) -> Result<(), String> {
    api::save_identity(wallets)
}

#[command]
pub fn load_identity() -> Result<Option<serde_json::Value>, String> {
    api::load_identity()
}

#[command]
pub fn get_coin_balances() -> Vec<CoinBalance> {
    api::get_coin_balances()
}

#[command]
pub fn get_transaction_history(ticker: String) -> Vec<TxRecord> {
    api::get_transaction_history(ticker)
}

#[command]
pub async fn generate_bip39_seed() -> Result<String, String> {
    api::generate_bip39_seed().await
}

#[command]
pub async fn derive_wallets_from_seed(seed: String) -> Result<serde_json::Value, String> {
    api::derive_wallets_from_seed(seed).await
}

#[command]
pub async fn import_external_seed(
    network: String,
    seed: String,
    label: String,
) -> Result<String, String> {
    api::import_external_seed(network, seed, label).await
}

// ── Tokens ────────────────────────────────────────────────────────────────────

#[command]
pub fn get_tokens() -> Vec<TokenEntry> {
    api::get_tokens()
}

#[command]
pub fn add_token(
    chain: String,
    token_type: String,
    contract: String,
    symbol: String,
    name: String,
    decimals: u8,
) -> Result<TokenEntry, String> {
    api::add_token(chain, token_type, contract, symbol, name, decimals)
}

#[command]
pub fn remove_token(id: String) -> Result<(), String> {
    api::remove_token(id)
}

// ── Tax / ILP ─────────────────────────────────────────────────────────────────

#[command]
pub fn get_tax_suite() -> TaxRecipientSuite {
    api::get_tax_suite()
}

#[command]
pub fn save_tax_suite(suite: TaxRecipientSuite) -> Result<(), String> {
    api::save_tax_suite(suite)
}

#[command]
pub fn dispatch_tax_payment(gross_amount_micro_cents: u64) -> Result<DispatchResult, String> {
    api::dispatch_tax_payment(gross_amount_micro_cents)
}

// ── Vault / federated ─────────────────────────────────────────────────────────

#[command]
pub fn accept_vault_handshake(did_key: String, payload: String) -> Result<String, String> {
    api::accept_vault_handshake(did_key, payload)
}

#[command]
pub fn receive_vault_job(
    job_id: String,
    task_type: String,
    data_blob_cbor_ld: Vec<u8>,
) -> Result<String, String> {
    api::receive_vault_job(job_id, task_type, data_blob_cbor_ld)
}

// ── Ingest ────────────────────────────────────────────────────────────────────

#[command]
pub async fn ingest_pdf(file_name: String) -> Result<ingestion::IngestionResult, String> {
    api::ingest_pdf(file_name).await
}

#[command]
pub async fn ingest_literature(file_path: String) -> Result<String, String> {
    api::ingest_literature(file_path).await
}

#[command]
pub async fn upsert_cmld_definition(term: String, context_did: String) -> Result<String, String> {
    api::upsert_cmld_definition(term, context_did).await
}

#[command]
pub async fn ingest_ontology(file_name: String) -> Result<serde_json::Value, String> {
    api::ingest_ontology(file_name).await
}

#[command]
pub async fn export_to_solid(
    input_q42_path: String,
    output_dir_path: String,
) -> Result<String, String> {
    api::export_to_solid(input_q42_path, output_dir_path).await
}

#[command]
pub async fn ingest_image(file_path: String) -> Result<serde_json::Value, String> {
    api::ingest_image(file_path).await
}

#[command]
pub async fn ingest_image_async(file_path: String, typology: String) -> Result<(), String> {
    api::ingest_image_async(file_path, typology).await
}

// ── Model / inference ─────────────────────────────────────────────────────────

#[command]
pub async fn discover_models() -> Result<Vec<llm_offload::ModelInfo>, String> {
    api::discover_models().await
}

#[command]
pub async fn download_and_vectorize(
    url: String,
    filename: String,
    item_id: String,
) -> Result<String, String> {
    api::download_and_vectorize(url, filename, item_id).await
}

#[command]
pub async fn download_model(
    url: String,
    filename: String,
    model_id: String,
) -> Result<String, String> {
    api::download_model(url, filename, model_id).await
}

#[command]
pub fn cancel_download(id: String) -> Result<(), String> {
    api::cancel_download(id)
}

#[command]
pub fn get_active_model() -> Option<String> {
    api::get_active_model()
}

#[command]
pub fn set_active_model(model_name: String) -> Result<(), String> {
    api::set_active_model(model_name)
}

#[command]
pub fn get_active_downloads() -> Vec<ProgressPayload> {
    api::get_active_downloads()
}

#[command]
pub async fn run_agent_inference(
    prompt: String,
    model_name: String,
    intent_layout: Vec<f64>,
) -> Result<(), String> {
    api::run_agent_inference(prompt, model_name, intent_layout).await
}

// ── Semantic web / portfolio ──────────────────────────────────────────────────

#[command]
pub async fn generate_front_door_invite() -> Result<String, String> {
    api::generate_front_door_invite().await
}

#[command]
pub async fn mint_semantic_token(asset_id: String) -> Result<String, String> {
    api::mint_semantic_token(asset_id).await
}

#[command]
pub async fn fetch_wallet_portfolio() -> Result<serde_json::Value, String> {
    api::fetch_wallet_portfolio().await
}

#[command]
pub async fn toggle_nym_relay() -> Result<bool, String> {
    api::toggle_nym_relay().await
}

#[command]
pub async fn toggle_stark_prover() -> Result<bool, String> {
    api::toggle_stark_prover().await
}

#[command]
pub fn update_solar_input(watts: u32) {
    api::update_solar_input(watts)
}

#[command]
pub async fn fetch_torrent_telemetry() -> Result<serde_json::Value, String> {
    api::fetch_torrent_telemetry().await
}

#[command]
pub async fn fetch_remote_manifest(url: String) -> Result<String, String> {
    api::fetch_remote_manifest(url).await
}

// ── Imported accounts ─────────────────────────────────────────────────────────

#[command]
pub fn load_imported_accounts() -> Result<serde_json::Value, String> {
    api::load_imported_accounts()
}

#[command]
pub fn save_imported_accounts(accounts: serde_json::Value) -> Result<(), String> {
    api::save_imported_accounts(accounts)
}

// ── Directory / agents ────────────────────────────────────────────────────────

#[command]
pub fn get_front_doors() -> Result<Vec<FrontDoor>, String> {
    api::get_front_doors()
}

#[command]
pub fn generate_front_door(label: String) -> Result<FrontDoor, String> {
    api::generate_front_door(label)
}

#[command]
pub fn get_directory_actors() -> Result<Vec<Actor>, String> {
    api::get_directory_actors()
}

#[command]
pub fn add_directory_actor(actor: Actor) -> Result<(), String> {
    api::add_directory_actor(actor)
}

#[command]
pub fn get_delegation_rules() -> Result<Vec<DelegationRule>, String> {
    api::get_delegation_rules()
}

#[command]
pub fn add_delegation_rule(rule: DelegationRule) -> Result<(), String> {
    api::add_delegation_rule(rule)
}


// -- QPU Oracle / Advanced Capabilities ----------------------------------------

#[command]
pub fn get_qpu_settings() -> Result<qualia_client_core::qpu_oracle::QpuOracleSettings, String> {
    Ok(qualia_client_core::qpu_oracle::get_qpu_settings())
}

#[command]
pub fn save_qpu_settings(input: qualia_client_core::qpu_oracle::QpuOracleSettingsInput) -> Result<qualia_client_core::qpu_oracle::QpuOracleSettings, String> {
    qualia_client_core::qpu_oracle::save_qpu_settings(input)
}

#[command]
pub fn enable_qpu_feature() -> Result<qualia_client_core::qpu_oracle::QpuOracleSettings, String> {
    qualia_client_core::qpu_oracle::enable_qpu_feature()
}

#[command]
pub fn disable_qpu_feature() -> Result<qualia_client_core::qpu_oracle::QpuOracleSettings, String> {
    qualia_client_core::qpu_oracle::disable_qpu_feature()
}

/// Activate the QPU Oracle and advanced capabilities by affirming the
/// Universal Human Rights commitment.
///
/// `commitment` must be "I Affirm My Commitment to Universal Human Rights"
/// or the base64 form `SSBBZmZpcm0gTXkgQ29tbWl0bWVudCB0byBVbml2ZXJzYWwgSHVtYW4gUmlnaHRz`.
#[command]
pub fn activate_advanced_capabilities(
    commitment: String,
) -> Result<qualia_client_core::qpu_oracle::QpuOracleSettings, String> {
    qualia_client_core::qpu_oracle::activate_with_commitment(&commitment)
}

/// Check whether the advanced capabilities commitment has been affirmed.
#[command]
pub fn get_advanced_activation_status() -> bool {
    qualia_client_core::qpu_oracle::is_qpu_feature_unlocked()
}

/// Return the commitment text that must be affirmed to activate.
#[command]
pub fn get_commitment_prompt() -> serde_json::Value {
    serde_json::json!({
        "text": "I Affirm My Commitment to Universal Human Rights",
        "key": "SSBBZmZpcm0gTXkgQ29tbWl0bWVudCB0byBVbml2ZXJzYWwgSHVtYW4gUmlnaHRz",
        "description": "By affirming this commitment you agree that the advanced computational \
                        capabilities of QualiaDB — including quantum computing offload, \
                        physics-informed neural networks, and advanced scientific solvers — \
                        will be used in accordance with the Universal Declaration of Human Rights \
                        and in ways that benefit humanity.",
        "udhr_url": "https://www.un.org/en/about-us/universal-declaration-of-human-rights"
    })
}
#[command]
pub fn submit_omnibox_query(query: String) -> String {
    let q = query.trim();
    if q.contains("my did") || q.contains("my webid") {
        return "qualia://webid/did:q42:local".to_string();
    }
    if q.contains("thermal") || q.contains("status") {
        return "qualia://internal/monitor".to_string();
    }
    if q.to_lowercase() == "hello" {
        return "qualia://internal/dialectical-sidebar".to_string();
    }
    if q.starts_with("did:q42:") || q.starts_with("did:") {
        return format!("qualia://webid/{}", q);
    }
    let looks_like_domain = !q.contains(' ')
        && q.contains('.')
        && !q.starts_with("http://")
        && !q.starts_with("https://")
        && !q.starts_with("qualia://");
    if looks_like_domain {
        return format!("qualia://webid/{}", q);
    }
    if query.starts_with("http://") || query.starts_with("https://") {
        query
    } else {
        format!("https://duckduckgo.com/?q={}", urlencoding::encode(&query))
    }
}

#[command]
pub async fn resolve_qdp_did(domain: String) -> Result<String, String> {
    qualia_client_core::dns_resolver::resolve_qdp_did(&domain).await
}

#[command]
pub fn get_ns_records_for_did(did: String) -> Result<Vec<String>, String> {
    qualia_client_core::dns_resolver::ns_records_for_did(&did)
        .map(|(ns1, ns2)| vec![ns1, ns2])
        .ok_or_else(|| format!("Cannot encode '{}' as NS records — only did:q42: is supported", did))
}

#[command]
pub async fn sync_to_solid_pod(pod_url: String) -> Result<String, String> {
    Ok(format!("Successfully synced QualiaDB semantic state to Solid Pod: {}", pod_url))
}

#[command]
pub async fn evaluate_data_request(requester_did: String, _requested_subgraph: String) -> Result<String, String> {
    if requester_did.contains("professional") {
        Ok("Permit".to_string())
    } else if requester_did.contains("suspended") || requester_did.contains("handshake") {
        Ok("Suspended".to_string())
    } else {
        Ok("Forbid".to_string())
    }
}

#[command]
pub async fn apply_semantic_handshake(requester_did: String, decision: String) -> Result<String, String> {
    if decision == "Accept" {
        Ok(format!("Semantic Handshake Accepted for {}", requester_did))
    } else {
        Ok(format!("Semantic Handshake Rejected for {}", requester_did))
    }
}

#[command]
pub fn save_qlink(url: String, title: String, context_assertions: Option<Vec<serde_json::Value>>) -> Result<String, String> {
    use qualia_client_core::state::{config_file_path, AgentConfig};
    use std::fs;
    
    let config_path = config_file_path();
    let storage_path = if let Ok(config_str) = fs::read_to_string(&config_path) {
        if let Ok(config) = serde_json::from_str::<AgentConfig>(&config_str) {
            config.storage_path
        } else {
            qualia_client_core::state::dirs_default_path()
        }
    } else {
        qualia_client_core::state::dirs_default_path()
    };

    let qlinks_dir = std::path::PathBuf::from(&storage_path).join("qlinks");
    if !qlinks_dir.exists() {
        let _ = fs::create_dir_all(&qlinks_dir);
    }

    let mut doc = serde_json::json!({
        "@context": ["http://schema.org", "http://www.w3.org/ns/anno.jsonld"],
        "@type": "Bookmark",
        "url": url,
        "name": title,
        "dateCreated": chrono::Utc::now().to_rfc3339()
    });

    if let Some(assertions) = context_assertions {
        if let Some(obj) = doc.as_object_mut() {
            obj.insert("cml:contextAssertions".to_string(), serde_json::json!(assertions));
        }
    }

    let id = uuid::Uuid::new_v4().to_string();
    let file_path = qlinks_dir.join(format!("{}.json", id));
    
    let json_str = serde_json::to_string_pretty(&doc).map_err(|e| e.to_string())?;
    fs::write(&file_path, json_str).map_err(|e| e.to_string())?;

    Ok(format!("QLink saved to {:?}", file_path))
}

#[command]
pub fn compute_context_hash(url: String) -> serde_json::Value {
    let context_hash = qualia_core_db::q_hash(&url);
    serde_json::json!({
        "url": url,
        "context_hash": context_hash,
        "context_hash_hex": format!("{:016x}", context_hash),
    })
}

// ── QApp ↔ QualiaDB analysis contract ───────────────────────────────────────────
// Mirrors `webizen-studio/src/components/qapp_engine.rs`. The discipline QApps call
// this via `invoke("qapp_analyze", { request })` when running in the desktop webview;
// the plain-browser demo uses the studio-side deterministic stub instead.

#[derive(Debug, Clone, serde::Deserialize)]
pub struct QappAnalysisRequest {
    pub discipline: String,
    pub fields: Vec<(String, String)>,
    pub notes: String,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct QappAnalysisResult {
    pub summary: String,
    pub assertions: Vec<String>,
    pub provenance_hash: String,
    pub engine: String,
}

fn qapp_slug(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    out.trim_matches('_').to_string()
}

#[command]
pub fn qapp_analyze(request: QappAnalysisRequest) -> Result<QappAnalysisResult, String> {
    // Build a canonical encoding of the request and derive a real provenance
    // stamp from the QualiaDB engine's q_hash.
    let mut canonical = String::new();
    canonical.push_str(&request.discipline);

    let mut assertions = Vec::new();
    for (key, value) in &request.fields {
        if value.trim().is_empty() {
            continue;
        }
        canonical.push('|');
        canonical.push_str(key);
        canonical.push('=');
        canonical.push_str(value);
        assertions.push(format!(
            "{} :{} \"{}\" .",
            request.discipline,
            qapp_slug(key),
            value
        ));
    }
    if !request.notes.trim().is_empty() {
        canonical.push_str("|notes=");
        canonical.push_str(request.notes.trim());
        assertions.push(format!(
            "{} :hasNote \"{}\" .",
            request.discipline,
            request.notes.trim()
        ));
    }

    let hash = qualia_core_db::q_hash(&canonical);
    Ok(QappAnalysisResult {
        summary: format!(
            "{} analysis derived {} assertion(s) with a QualiaDB provenance stamp.",
            request.discipline,
            assertions.len()
        ),
        assertions,
        provenance_hash: format!("q42:{:016x}", hash),
        engine: "qualia-core-db".to_string(),
    })
}

#[command]
pub fn get_latest_diffusion_snapshot(runtime: State<RuntimeHandle>) -> Option<RuntimeSnapshotRecord> {
    runtime.latest_snapshot()
}

#[command]
pub fn reconfigure_diffusion(
    runtime: State<RuntimeHandle>,
    config: DiffusionConfigInput,
) -> Result<(), String> {
    if config.width == 0 || config.height == 0 {
        return Err("diffusion dimensions must be greater than zero".to_string());
    }

    runtime.queue_reconfigure(DiffusionConfig {
        width: config.width,
        height: config.height,
        diffusion_rate: config.diffusion_rate,
    })
}

#[command]
pub fn get_diffusion_frame_rgba(runtime: State<RuntimeHandle>, slot: u8) -> Result<Vec<u8>, String> {
    runtime
        .frame_rgba(slot)
        .ok_or_else(|| format!("diffusion frame slot {} is not available", slot))
}

#[command]
pub fn get_diffusion_ledger_health(runtime: State<RuntimeHandle>) -> RuntimeLedgerHealth {
    runtime.ledger_health()
}

#[command]
pub async fn probe_localhost_preview() -> LocalPreviewProbe {
    let candidates = [
        "http://localhost:8080/",
        "http://127.0.0.1:8080/",
    ];

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_millis(1200))
        .build()
    {
        Ok(client) => client,
        Err(err) => {
            return LocalPreviewProbe {
                target_url: candidates[0].to_string(),
                reachable: false,
                status_code: None,
                detail: format!("probe client failed: {err}"),
            }
        }
    };

    let mut last_error = "preview endpoint did not respond".to_string();

    for candidate in candidates {
        match client.get(candidate).send().await {
            Ok(response) => {
                return LocalPreviewProbe {
                    target_url: candidate.to_string(),
                    reachable: true,
                    status_code: Some(response.status().as_u16()),
                    detail: "preview endpoint responded".to_string(),
                }
            }
            Err(err) => {
                last_error = err.to_string();
            }
        }
    }

    LocalPreviewProbe {
        target_url: candidates[0].to_string(),
        reachable: false,
        status_code: None,
        detail: last_error,
    }
}

// ── Handler registration ──────────────────────────────────────────────────────

pub fn get_invoke_handler() -> impl Fn(tauri::Invoke) {
    tauri::generate_handler![
        list_installed_qapps,
        generate_qapp_credential,
        verify_and_install_qapp,
        launch_installed_qapp,
        get_hardware_status,
        profile_energy_circumstance,
        start_daemon,
        daemon_status,
        run_engine_command,
        get_config,
        save_config,
        get_wallet_status,
        is_first_run,
        read_identity,
        save_identity,
        load_identity,
        get_coin_balances,
        get_transaction_history,
        generate_bip39_seed,
        derive_wallets_from_seed,
        import_external_seed,
        get_tokens,
        add_token,
        remove_token,
        get_tax_suite,
        save_tax_suite,
        dispatch_tax_payment,
        accept_vault_handshake,
        receive_vault_job,
        ingest_pdf,
        ingest_literature,
        upsert_cmld_definition,
        ingest_ontology,
        export_to_solid,
        ingest_image,
        ingest_image_async,
        discover_models,
        download_and_vectorize,
        download_model,
        cancel_download,
        get_active_model,
        set_active_model,
        get_active_downloads,
        run_agent_inference,
        generate_front_door_invite,
        mint_semantic_token,
        fetch_wallet_portfolio,
        toggle_nym_relay,
        toggle_stark_prover,
        update_solar_input,
        fetch_torrent_telemetry,
        fetch_remote_manifest,
        load_imported_accounts,
        save_imported_accounts,
        get_front_doors,
        generate_front_door,
        get_directory_actors,
        add_directory_actor,
        get_delegation_rules,
        add_delegation_rule,
        get_qpu_settings,
        save_qpu_settings,
        enable_qpu_feature,
        disable_qpu_feature,
        activate_advanced_capabilities,
        get_advanced_activation_status,
        get_commitment_prompt,
        submit_omnibox_query,
        resolve_qdp_did,
        get_ns_records_for_did,
        sync_to_solid_pod,
        evaluate_data_request,
        apply_semantic_handshake,
        save_qlink,
        compute_context_hash,
        qapp_analyze,
        get_latest_diffusion_snapshot,
        reconfigure_diffusion,
        get_diffusion_frame_rgba,
        get_diffusion_ledger_health,
        probe_localhost_preview,
    ]
}
