# Webizen — Master Implementation Plan

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Branch:** `0.0.2`
**Status:** Planning document. Supersedes the SvelteKit-era `legacy/devnotes/port_requirements.md` as the *execution* plan; that file remains the *feature spec*.

---

## 0. The One Non-Negotiable Constraint: NO Node.js. Pure Rust/Dioxus.

This project was restarted specifically to **eliminate Node.js**. The whole app — UI and backend — is Rust:

- **Frontend:** Dioxus → `wasm32-unknown-unknown`.
- **Backend / shell:** Tauri (Rust) + `qualia-core-db` / `qualia-client-core` (Rust) + `webizen-runtime` (Rust/wgpu).

**Hard rules:**
1. No `package.json`, `npm`/`pnpm`/`yarn`, Vite, `node_modules`, or any JS build step — ever.
2. No JS UI frameworks (no Svelte/React/Vue) and no JS application logic.
3. The `legacy/` folder is **reference-only**. We read it to understand *what a feature does and why*; we never import its SvelteKit/TypeScript code. Every feature is re-authored in Rust/Dioxus.
4. Every JavaScript library the legacy app used is replaced by a **Rust crate** (table below). The only JS permitted is the thin, unavoidable browser-API surface reached through `web-sys`/`wasm-bindgen` (e.g. `BarcodeDetector`, `WebSocket`, `Canvas`, WebGPU) — those are *browser APIs*, not Node and not bundled libraries.

### 0.1 Capability map: legacy dependency → **QualiaDB-native** (not external crates)

**Correction (2026-06-15):** an earlier draft of this plan wrongly proposed external Rust crates (`oxigraph`, `candle`, `llama-cpp-rs`, etc.). That is wrong. Per `QUALIA_DB_LOGIC_AUDIT.md`, QualiaDB has **collapsed hundreds of libraries into one engine** and the QApps are "UI thin-clients wrapping these massive internal logic compute units." The rule is therefore stronger than "no Node": **no external compute/engine crate either** — SPARQL, LLM, logic, RDF(-star), physics, crypto, and DLT all come from `qualia-core-db`. The browser/UI side only ever does *presentation* (Dioxus + `web-sys` browser APIs) and *transport* (Tauri `invoke` / WS to the daemon).

| Legacy dependency (JS or otherwise) | Purpose | Correct source — **QualiaDB module** (verified in `qualia-core-db/src`) |
|---|---|---|
| **llama.cpp / candle / GGUF-JS** | LLM inference | `llm_agent`, `gguf_bridge`, `gguf_sharder`, `ggml_quants`, `lora/`, `resident_model`, `directml_bridge`/`metal_bridge`/`npu_ffi`, `shaders/` — **internal wgpu LLM**. No candle, no llama.cpp. |
| **oxigraph / SPARQL-JS** | SPARQL / SPARQL-star | `sparql_library` (SPARQL 1.1, federated, `sparql_mm`, WS streaming) + `query_engine`/`query_compiler`. No oxigraph. |
| **rdflib / N3-JS** | RDF, RDF-star, N3, OWL, SHACL | `rdf_star`, `modalities/` (N3/OWL), `shacl_compiler`, `ontology_loader`. |
| **JSON-LD / format libs** | Document/container format | `q42_lex`/`q42_reader`/`q42_volume`/`q42_lexicon`, `yaml_ld_q42`, `cbor_compiler` — the **q42 format** (48-byte NQuin). See `WEBIZEN_HYPERMEDIA_FORMAT.md`. |
| **Babylon.js / three.js** | 3D / anatomy viewer | `webizen-runtime` wgpu + the `render` `Renderer` trait, with math/physics from QualiaDB `geometric_algebra/` (SIMD), `domains/physical`, `quantum_dft`, `ode_solver`, and diffusion `shaders/`. No Babylon/three. |
| **Whisper-JS / Tesseract.js / PDF.js** | Audio/OCR/PDF ingestion | `ingest`/`ingestion`, `dicom_ingest`, `specialized_libs` ML layers; PDF already routed via `qualia-client-core` `ingest_pdf`. |
| **OpenTimestamps-JS** | DLT / provenance anchoring | `provenance`, `fiduciary_crypto`, `zk_proofs`. |
| **Chart.js** | Charts | **Presentation only** — hand-rolled SVG in Dioxus (dashboard already does this). Data comes from QualiaDB queries. |
| **ZXing-js** | Barcode | Browser `BarcodeDetector`/`getUserMedia` via `web-sys` (W3C API, allowed); decoded value handed to QualiaDB. |
| **Shoelace (CDN web components)** | UI widgets | Decision §11.1 — keep (standards web components, no Node) or go native Dioxus. |

> Allowed on the UI side because they are the *platform*, not engines: W3C browser APIs via `web-sys` (WebGPU, WebRTC, WebSocket, Canvas, localStorage/IndexedDB, `getUserMedia`, `BarcodeDetector`) and pure-presentation crates (Dioxus, SVG). Everything that *computes* goes to QualiaDB.

---

## 1. Where We Are Today (verified facts)

**Real and working (Rust, no Node):**
- `webizen-runtime` — genuine wgpu compute (diffusion kernel), native. Solid.
- `webizen-desktop` — Tauri shell: 79 commands, **system tray**, **local daemon on port 4242** (`start_local_daemon_with_options`), **localhost:8080 preview server**, `open` crate for launching URLs, auto-updater, sysinfo telemetry. Full `qualia-core-db` + `qualia-client-core` linked.
- `webizen-studio` — Dioxus → wasm: dashboard, 274 academic QApps (catalogue↔dispatcher consistent + tested), QApp Studio canvas, theme engine, `render` module (`Renderer` trait + Canvas2D backend), `endpoints` capability gate, `qapp_engine` QualiaDB contract (stub in browser / real Tauri command on desktop).
- `0.0.2` branch fixes: boot-panic safety, theme unification, panic hook, **header/boot-loader fix** (this session), CI consistency gate.

**Not yet real:**
- The wasm build still includes the in-app **browser pane** (should be excluded from the public wasm — see §3).
- No **Settings** page, no **About** page (native shell).
- No **browser-projection toggle** (surface the app in Chrome/Firefox/Edge).
- The 274 QApps are presentational; only 3 are wired to the engine contract (Anthropology/History/Economics pilots).
- None of the deep `port_requirements.md` features (LLM, Health Vault, Sanctuary Mode, Wallet, Cooperative, Anatomy) exist in Dioxus yet.
- The browser-side 3D is still CPU Canvas2D; the WebGPU `Renderer` backend is a stub-in-waiting.

---

## 2. Target Architecture — Three Run Profiles, One Rust Codebase

The user's "two/three functions" map cleanly onto **one shared Dioxus UI crate** compiled/served three ways:

```
                         ┌─────────────────────────────────────────────┐
                         │            webizen-studio (Dioxus)           │
                         │   QApps · 3D · dashboard · settings · about  │
                         │   capability-gated features (browser pane…)  │
                         └───────────────┬─────────────────────────────┘
                                         │ same wasm artifact
        ┌────────────────────────────────┼────────────────────────────────┐
        ▼                                 ▼                                 ▼
 (A) PUBLIC WASM                   (B) DESKTOP WEBVIEW              (C) BROWSER-PROJECTION
 GitHub Pages                      Tauri window                    System Chrome/FF/Edge
 - no browser pane                 - full features                 - opt-in toggle in (B)
 - no daemon (stub engine)         - full qualia-db                - desktop serves wasm at
 - 3D via WebGPU/Canvas2D          - browser pane ON                 http://127.0.0.1:8080
 - read-only / preview             - is_native_host()=true         - connects back to :4242
                                   - system tray, settings, about  - same is_native_host()=true
```

- **(A) Public WASM** = "webizen wasm". Excludes the browser pane and any daemon-only feature; uses the deterministic engine **stub**. This is the Pages demo and the embeddable surface. The 3D (`webizen-runtime` → WebGPU `Renderer` backend) lands here, so yes — "webizen wasm" becomes the QApps + 3D surface, **without** an embedded browser.
- **(B) Desktop webview** = the local install, **and this *is* the new browser** — not merely a Studio host. The QualiaDB daemon baked into it is the local agent; the user's machine becomes a globally discoverable hub via QDP DNS + Front Door DIDs, with all inbound contact funnelled through one HCAI-agreement chokepoint. The Tauri shell bundles the *same* wasm (`distDir: ../webizen-studio/dist`) but the webview exposes `window.__TAURI__`, so `is_native_host()` is true and the full feature set unlocks: real QualiaDB, the browser pane, system tray, settings, about, the port-4242 relay, **and the socially-defined network layer** (QDP / Front Door / HCAI / Nym / WebRTC / WebTorrent). See **`WEBIZEN_NETWORK.md`** — much of this (`resolve_qdp_did`, `generate_front_door`, `toggle_nym_relay`, semantic handshake) is *already* live as Tauri commands; the gaps are the `did.json` frontdoor server, the HCAI endpoint, and the Dioxus UI.
- **(C) Browser projection** = an opt-in toggle in the desktop install that serves the wasm from the desktop's own localhost server (`:8080`) and opens it in the user's system browser. That browser tab talks back to the desktop daemon on `:4242`. Same `is_native_host()` capability path (detect daemon reachability instead of `__TAURI__` — see §5.3).

**Key insight:** capability gating is *runtime*, not a second build. One wasm artifact serves all three; features light up based on `endpoints::is_native_host()` / daemon reachability. This avoids a CI matrix and keeps "no Node" trivially true.

---

## 3. Scoping the WASM build (remove the browser from public wasm)

Today `Route::BrowserRoute` + `WebBrowserPane` ship in every build. For the public wasm they should be **absent or inert**.

**Plan:**
1. Gate the browser pane behind a runtime capability: render the "Browser" nav item and route only when `endpoints::is_native_host()` is true. In the public demo it simply isn't shown.
2. For a hard exclusion (smaller wasm, zero browser code in the public artifact), add a cargo feature `browser-pane` (default off) and `#[cfg(feature = "browser-pane")]` the route/module; the desktop build enables it. Recommended **after** the runtime gate, as a size optimization, not a correctness need.
3. The in-app browser pane itself (when present in desktop) is a Tauri webview/child window concern, not a Dioxus DOM `<iframe>`, to keep it real on native.

Decision default: **runtime gate now** (one artifact), **cargo feature later** if wasm size matters.

---

## 4. The Native Desktop (local install) — fill the gaps

All of these are Rust (Dioxus components + Tauri commands). None need Node.

### 4.1 Settings page (Dioxus route `/settings`)
- General: node name, theme (wire to existing `theme_engine`), data directory.
- Engine: daemon port (default 4242), preview port (8080), auto-start daemon toggle.
- Privacy: Nym relay toggle (command exists: `toggle_nym_relay`), telemetry on/off.
- **Browser projection toggle** (§4.4).
- Hardware: GPU backend selection (the `hardware-config` QApp is a stub to promote here).
- Persisted via existing `AgentConfig` (`get_config`/`save_config` commands already exist).

### 4.2 About page (Dioxus route `/about`)
- App name/version (from `tauri.conf.json` / `CARGO_PKG_VERSION`).
- Author: Timothy Charles Holborn; license CC BY-NC-ND 4.0; links.
- Build info: commit hash, qualia-core-db version, wgpu adapter in use.
- "About me" (user identity): surfaces the local `did:q42:` identity, Ed25519 key fingerprint, VC count (commands exist: `read_identity`, `load_identity`).

### 4.3 System tray — already present; extend
- Existing items: Open Studio, Settings, Logs, localhost preview, Revoke Sessions, Daemon Status, Quit.
- Add: **"Open in system browser"** (triggers §4.4), **"Projection: on/off"** indicator.

### 4.4 Browser-projection toggle (the headline new capability)
Goal: user flips a switch → parts of the app appear in their real browser (Chrome/FF/Edge).

Mechanism (all Rust, reuses existing pieces):
1. Desktop already runs a localhost preview server (`:8080`) and has the `open` crate.
2. Serve the **studio wasm bundle** (the same `dist/`) from that server with permissive-localhost CORS.
3. On toggle ON: `open::that("http://127.0.0.1:8080/")` launches the default browser; persist the preference.
4. That browser tab loads the wasm; `is_native_host()` must return true there too — but `window.__TAURI__` is absent in a real browser. So detect the daemon instead: **probe `ws://127.0.0.1:4242`** (or a `GET /healthz` on 8080) and treat reachability as "native host present." (`endpoints.rs` gets a second detection path.)
5. The **:4242 daemon** must accept the browser origin (CORS / `Access-Control-Allow-Origin: http://127.0.0.1:8080`, WS origin check) — a small change in the `qualia-core-db` daemon or a thin Tauri-side reverse proxy.
6. Security: bind to `127.0.0.1` only; the projection is loopback-only; never `0.0.0.0`. Document this.

**This is the "local relay for port 4242 for the wasm" the user described** — the desktop is the relay; the wasm (in webview *or* system browser) is the client.

---

## 5. Capability detection (`endpoints.rs`) — generalize

Current `is_native_host()` only checks `window.__TAURI__`. Generalize to:
1. `__TAURI__` present → **webview host** (full trust, direct `invoke`).
2. else probe daemon (`:4242` WS open or `:8080 /healthz` 200) → **projected host** (use HTTP/WS relay instead of `invoke`).
3. else → **public demo** (engine stub only).

`qapp_engine::analyze` and friends choose transport accordingly: `invoke` in (1), `fetch`/WS to `:8080`/`:4242` in (2), stub in (3).

---

## 6. Feature Port Matrix (legacy spec → Rust/Dioxus)

Re-implementations of `port_requirements.md`, each pure-Rust. Priority: **P0** = MVP for a credible local install; **P1** = strong; **P2** = later.

UI is always a Dioxus thin-client; the "engine" column names the **QualiaDB module** that actually computes.

| # | Feature (legacy source) | Dioxus UI + **QualiaDB module** | Prio |
|---|---|---|---|
| F1 | LLM inference + streaming chat | chat UI ← `llm_agent` + `gguf_bridge`/`lora`/`resident_model` + wgpu `shaders/` (internal LLM); stream via Tauri events | P0 |
| F2 | Chat session history / branching | session list ← `wal`/`temporal_graph` (provenance-stamped turns) | P0 |
| F3 | Universal ingestion (PDF/lit/image/web/audio) | drop-zone UI ← `ingest`/`ingestion`, `dicom_ingest`, `cbor_compiler` | P0 |
| F4 | SPARQL / SPARQL-star + graph browser | query playground + SVG graph ← `sparql_library`, `query_engine`, `rdf_star` | P0 |
| F5 | Full DID / VC management | credential UI ← `webizen_identifiers`, `identifier`, `key_vault`, `fiduciary_crypto` | P0 |
| F6 | Health Vault (records, meds, FHIR, SHACL) | vault UI ← `clinical_engine`, `comorbidity_eval`, `shacl_compiler`, `domains` | P1 |
| F7 | Sanctuary Mode (duress, dead-man switch, anchoring) | UI ← `key_vault`, `fiduciary_crypto`, `zk_proofs`, `provenance` (DLT anchor) | P1 |
| F8 | Multi-chain wallet ("care credits") | wallet UI ← `ilp_dispatcher`, `fiduciary_crypto`, `nym_adapter` | P1 |
| F9 | Voice/video calls (WebRTC) | call overlay ← `web-sys` WebRTC (W3C) + `webizen_sync`/`nym_adapter` signalling | P1 |
| F10 | 3D Anatomy viewer + health↔anatomy map | wgpu `Renderer` ← `geometric_algebra` (SIMD math) + `modalities` N3 organ-intensity rules over the health graph | P1 |
| F11 | Cooperative projects/obligation/ledger | project UI + SVG charts ← `deontic_logic`, `modalities`, `provenance` | P2 |
| F12 | Verified directory & contacts (FOAF/ODRL) | contact grid ← `agency`, `webizen_identifiers`, `deontic_logic` | P2 |
| F13 | QApp manifest v2 + hypermedia container | ← `extension_manifest`, `extension_bus`, `q42_*`, `webtorrent_seeder` (see `WEBIZEN_HYPERMEDIA_FORMAT.md`) | P2 |
| F14 | Tripwire / synthesis (logic rules) | dashboards ← `modalities` (paraconsistent/ASP/deontic), `neuro_symbolic_sieve` | P2 |
| F15 | Wire remaining 271 QApps to `qapp_engine` | mechanical: `EnginePanel` per discipline (pattern proven) | P1 |
| F16 | **Browser network layer** (QDP / Front Door / HCAI / Nym / VerifiedComms / browser pane) | omnibox router + Shield overlay ← `resolver`/`dns_resolver`, `webizen_identifiers`, `webizen_server`, `deontic_logic`, `nym_adapter`, `p2p`. **Much already live** — see `WEBIZEN_NETWORK.md` | P0 (this *is* the browser) |

---

## 7. QualiaDB Engine Surface (already exists — to be *exposed*, not rebuilt)

These capabilities are **already implemented** in `qualia-core-db` (sibling repo). The work is to surface them through Tauri commands + Dioxus UI, not to add engines:
- `sparql_library` / `query_engine` / `rdf_star` → F4 query & graph browsing.
- `modalities/` (N3, OWL, deontic, epistemic, paraconsistent, ASP, LTL) → F6/F10/F11/F14 rules.
- `llm_agent` + `gguf_bridge` + `lora` + wgpu `shaders/` → F1 inference (no external LLM).
- `geometric_algebra/` (SIMD) + `domains/physical` + `quantum_dft` + `ode_solver` → F10 / 3D engine math & physics.
- `provenance` / `fiduciary_crypto` / `zk_proofs` → F5/F7 anchoring & credentials.
- `webtorrent_seeder` / `webtorrent_routes` / `p2p` / `crdt` → distribution & sync.
- `q42_*` / `yaml_ld_q42` / `cbor_compiler` → the hypermedia container format (§ below).

If a *new* engine primitive is genuinely missing, it is added in `qualia-core-db`, never reimplemented UI-side. The studio↔engine **contract already exists** (`qapp_engine` + `qapp_analyze` Tauri command); new capabilities extend that command surface rather than inventing new transport.

---

## 8. Multi-Agent Execution Plan

The work parallelizes along clean seams. Suggested agent swimlanes (each is mostly independent after the Phase-0 scaffolding lands):

- **Agent A — Shell & Architecture:** §3 wasm scoping, §4 settings/about/tray, §4.4 projection toggle, §5 capability detection. *Owns `main.rs`, `endpoints.rs`, `webizen-desktop`.*
- **Agent B — 3D/Render:** WebGPU `Renderer` backend behind the existing trait; promote physics sim to GPU; foundation for F10 anatomy. *Owns `src/render`, `webizen-runtime`.*
- **Agent C — Engine Contract & QApps:** F15 (wire the 271 remaining QApps via `EnginePanel`), expand `qapp_engine`/`qapp_analyze` for richer returns. *Owns `qapp_engine.rs`, `*_qapp.rs`, desktop commands.*
- **Agent D — Ingestion & LLM (F1–F5):** candle chat, ingestion pipeline, SPARQL UI, DID/VC. *Owns new `components/{chat,ingest,sparql,identity}` + desktop commands.*
- **Agent E — Health/Safety/Wallet (F6–F9):** later phase; depends on engine work.

Coordination rules: each agent works on its own files/worktree; shared touch-points (`main.rs` routes, `mod.rs`, desktop `get_invoke_handler`) are append-only and reconciled by Agent A. The catalogue↔dispatcher test + `cargo fmt --check` + wasm `cargo check` gate every merge. **No agent may add a Node/JS dependency** (CI grep gate, §10).

> Operationally: spin these up as parallel sub-agents (worktree isolation) once Phase 0/1 below is merged, so they branch from a stable base.

---

## 9. Phased Roadmap

**Phase 0 — Stabilize & scope (days)**
- Merge `0.0.2` (boot fix, header fix, theming, contract, tests) to `main`; redeploy Pages.
- Runtime-gate the browser pane out of the public wasm (§3).
- Add `/settings` + `/about` Dioxus routes (§4.1–4.2) — even thin versions.

**Phase 1 — Local install becomes real (1–2 wks)**
- Browser-projection toggle end-to-end (§4.4) + generalized capability detection (§5).
- WebGPU `Renderer` backend (§Agent B) so the demo benchmarks real GPU.
- F15: wire all 274 QApps to the engine contract (mechanical, parallelizable).

**Phase 2 — Knowledge & AI (2–4 wks)**
- F1 LLM (candle) + F2 chat history; F3 ingestion; F4 SPARQL; F5 DID/VC.

**Phase 3 — Health, Safety, Cooperative, Anatomy (engine-gated)**
- F6–F11 as QualiaDB engine features land.

**Phase 4 — Comms, Wallet, P2P**
- F8, F9, F12–F14.

---

## 10. Guardrails (CI)

- **No-Node gate:** CI step fails if `package.json`, `node_modules`, `*.svelte`, or a JS bundler config appears outside `legacy/`.
- **No-new-JS-lib gate:** fail if a new CDN `<script>`/`document::Script` is added without sign-off (protects the Shoelace decision).
- Existing gates kept: catalogue↔dispatcher consistency, `cargo fmt --check`, wasm `cargo check`, informational clippy.

---

## 11. Open Decisions (need the user)

1. **Shoelace.** Keep the CDN web-components (standards-based, no Node, but third-party JS in the page) or invest in native Dioxus components for a zero-third-party-JS guarantee? *Recommendation: keep for now (it's web components, not Node), revisit if "absolutely no JS in the page" is required.*
2. **Browser-pane exclusion:** runtime-gate only, or also a `browser-pane` cargo feature for a slimmer public wasm?
3. **Projection scope:** project the *whole* app to the system browser, or only selected QApps/panels?
4. **Hypermedia container format:** ratify the `.q42app` design in `WEBIZEN_HYPERMEDIA_FORMAT.md` (q42-native, WebTorrent-distributable).

> Resolved (no longer open): SPARQL and LLM are **QualiaDB-native** (`sparql_library`, `llm_agent`) — no oxigraph, candle, or llama.cpp.

---

## 12. Immediate Next Actions

1. Land the header/boot-loader fix (done this session, on `0.0.2`) and merge `0.0.2` → `main`.
2. Build Phase 0: browser-pane runtime gate + `/settings` + `/about` skeletons.
3. Stand up the WebGPU `Renderer` backend (unblocks real 3D in the demo).
4. Kick off Agent C to wire the remaining QApps to the engine contract.
5. Confirm the §11 decisions, then fan out Agents A–E in worktrees.

*Everything in this plan is Rust/Dioxus. The `legacy/` SvelteKit app is consulted for behavior and intent only; not one line of its Node/TypeScript is carried forward.*
