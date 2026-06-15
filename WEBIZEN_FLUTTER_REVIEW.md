# Review: `qualia-flutter-desktop` (earlier implementation)

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Source:** `C:\Projects\Local_LIbraries\EXPORTS\qualia-flutter-desktop`
**Status:** Reference review for migration into the Rust/Dioxus browser. **No code ported** — Flutter/Dart is consulted for *feature shape and UX*, then rebuilt in Dioxus.

> ⚠️ **Staleness caveat (per the user):** the QualiaDB backend has been updated since this Flutter app (it reached ~v0.0.10). The Flutter **FFI API reference** (`docs/flutter-api-reference.md`) reflects an older `qualia-core-db`; treat the *feature inventory* as authoritative but **re-verify each FFI signature against today's `qualia-client-core`** before wiring. Several calls here already exist (often renamed) as Tauri commands in `webizen-desktop`.

---

## 1. What it is

A complete **Flutter desktop** app (`flutter_rust_bridge` → Rust `qualia-core-db`), 65 Dart files: **17 screens, 22 widgets, 9 services**, a system tray, and a generated FFI bridge (`lib/src/rust`). It is the most *complete* prior expression of the product — essentially the same target as the Dioxus browser, built once already. The Dioxus app should reach **feature parity with this**, then exceed it (3D, §see vision doc).

> Note: this is the third reference implementation reviewed — after `legacy/` (SvelteKit) and the QualiaDB crates. Flutter is the richest for **UX/feature shape**; SvelteKit for the network/QDP design; the crates are the actual engine.

---

## 2. Screens → migration status

UI to rebuild in Dioxus; the backend column notes whether the capability already exists (mostly yes, in `qualia-client-core`).

| Flutter screen | Feature | Backend status | Dioxus action |
|---|---|---|---|
| `dashboard_screen` | Home / telemetry | exists (`system_telemetry`) | ✅ have a Dashboard; align |
| `chat_screen` + `chat_history_drawer` + `chat_environment_sheet` | Full chat w/ history + environment | exists (`chat_*` ×8) | **`qualia-chat`** feature (P0) |
| `llm_hub_screen` | Model mgmt / inference | exists (`model_lifecycle`, `inference_backend`) | surface (P0) |
| `ontology_hub_screen` | Ontology workbench | exists (`ontology_workbench`, `ontology_router`) | surface (P1) |
| `credential_manager_screen` | DID / VC | exists (`identifier`, `webizen_identifiers`, `key_vault`) | surface (P0) |
| `qapp_vault_screen` + `qualia_qapp_webview` | QApp store + host | exists (`qapp_registry`, `qapps_protocol`) | hypermedia format (`.q42app`) |
| `wallet_screen` | Multi-chain wallet | exists (`ilp_dispatcher`, wallet API) | surface (P1) |
| `address_book_screen` | Contacts / directory | exists (`social_connect`, `guardianship`) | surface (P1) — also the SocialWebNet peer source |
| `asset_library_screen` | Imported assets | exists (`resource_import`) | surface (P1) |
| `profile_screen` | User profile / "about me" | exists (`user_profile`) | ✅ Settings/About (Master Plan §4) |
| `settings_screen` | Settings | exists (`state`/config) | ✅ Settings (Master Plan §4) |
| `setup_wizard_screen` + `prerequisites_overlay` | Onboarding / prereqs | exists (`prerequisites`) | **new — needed** (first-run) |
| `spatial_physics_screen` | "3D" physics surface | **was WebView + JS** (`webview_flutter`, `JavaScriptMode.unrestricted`) | **replace with native `webizen-render`** — this is exactly what the new 3D engine is for |

**Headline:** the Flutter `spatial_physics_screen` rendered 3D via an embedded **WebView running JavaScript** — the very thing we're eliminating. The native Rust `Renderer`/`webizen-render` engine is its principled replacement. This validates the 3D engine direction concretely.

---

## 3. Widgets → the UX vocabulary to carry forward

The 22 widgets are the *distinctive human-centric UX* — most map to QualiaDB concepts and should be re-created in Dioxus (and, per the vision doc, **animated/3D-enhanced**):

- **Provenance / epistemics:** `chat_citation_chips`, `super_quin_provenance_chip`, `super_quin_inspector_sheet`, `sensitivity_badge` — every claim shows its NQuin source + sensitivity. *This is the soul of the product.*
- **Safety / consent (the Shield):** `shield_alert`, `vault_hud_bar`, `guardian_affirmation_chip`, `pending_affirmations_panel`, `axiom_bounds_sheet` — the HCAI/Duty-of-Care + Sanctuary consent surface (the "Shield overlay" the network doc calls for).
- **Chat richness:** `chat_graph_panel` (the DAG!), `markdown_message`, `latex_math_keyboard`, `chat_files_panel`, `chat_file_permissions_sheet`, `chat_image_attachment`, `chat_reaction_bar`, `chat_session_shares_sheet`, `chat_agent_outcome_sheet`, `chat_environment_bar`.
- **Knowledge:** `ontology_workbench_sheet`.
- **Social:** `add_friends_sheet`.
- **Dev:** `DevConsoleOverlay`.

### 3a. `qualia-chat` target shape (confirms the rename)
The Flutter chat is the reference for the consolidated **`qualia-chat`**: a chat surface with a **graph panel** (the `chat_graph` DAG), citation/provenance chips, file & image attachments with per-file permissions, reactions, session sharing, a sub-agent outcome sheet, and an environment bar. Build this as the one chat feature; fold the misnamed `chat_graph.rs` pane in.

---

## 4. Services → background capabilities

| Flutter service | Purpose | Notes |
|---|---|---|
| `chat_speech_service` | Voice / TTS | new in Dioxus (W3C SpeechSynthesis via `web-sys`) |
| `deep_link_service` | `qualia://` / `webizen://` deep links | maps to the Tauri scheme handlers (partly present) |
| `hardware_telemetry_service` + `system_telemetry_hub` | Live HW/VRAM HUD | `system_telemetry` exists; desktop already emits `hardware-telemetry` |
| `model_activation_service` | Async model load | `model_lifecycle` exists |
| `pending_affirmations_service` | HCAI/guardian affirmations queue | ties to the HCAI endpoint gap |
| `qapp_launcher` | Launch QApps | `qapp_registry`/`qapps_protocol` |
| `qpu_feature_service` | QPU unlock (UDHR affirmation) | `qpu_oracle` exists (commands present) |
| `update_checker` | Auto-update | desktop updater exists |

---

## 5. The third QApp manifest format (`qapp.json` + `x_qualia`)

`docs/qapp-vault-developer-guide.md` defines a **`qapp.json`** manifest (directory structure, `x_qualia` extension fields, launch context, local daemon API). This is a **third** QApp container shape, alongside `QappPackageManifest` (client-core) and the proposed `.q42app`. **Implication for the hypermedia format:** `.q42app` should *unify* these — reconcile `qapp.json` `x_qualia` fields + `QappPackageManifest` into the one q42-packaged, signed, UI-declarative format. Add `qapp.json` to the reconciliation list in `WEBIZEN_HYPERMEDIA_FORMAT.md`.

---

## 6. Net findings (added to the plan)

1. **The Flutter app is the feature-parity target.** Almost every screen's backend already exists in `qualia-client-core`; the Dioxus work is overwhelmingly **surfacing**, not building — consistent with the gap audit.
2. **Onboarding is a genuine new need** (`setup_wizard` + `prerequisites`) — not present in the Dioxus app yet.
3. **The 3D "spatial physics" screen was a WebView/JS hack** → the native `webizen-render` engine is its correct replacement (concrete validation of the 3D direction).
4. **Voice/TTS** (`chat_speech_service`) is a small new capability (W3C SpeechSynthesis).
5. **Three QApp manifest formats now exist** → the hypermedia `.q42app` must unify `qapp.json`/`x_qualia` + `QappPackageManifest`.
6. **The widget vocabulary (provenance chips, shield HUD, guardian/affirmation chips, chat-graph) is the human-centric identity of the product** — carry it forward and elevate it with native 3D + motion (see `WEBIZEN_3D_NATIVE_VISION.md`).

**Re-verification task before wiring:** diff `docs/flutter-api-reference.md` FFI names against today's `qualia-client-core` public API — names/signatures have drifted with the backend updates.
