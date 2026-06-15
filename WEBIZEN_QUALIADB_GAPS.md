# QualiaDB Library Gap Audit (for the Webizen browser)

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Method:** capability probes across all `C:\Projects\qualiaDB\crates` (`grep -rilE`, excluding `target/`), cross-referenced with `QUALIA_DB_LOGIC_AUDIT.md` and the feature set in `WEBIZEN_MASTER_PLAN.md`. Counts below are *files matched*, a coarse but honest presence signal.

> Scope note: "missing" means **missing from the QualiaDB engine crates**, where engine capability belongs (the no-external-engine rule). UI/presentation gaps are *not* engine gaps — they're Dioxus work. The one principled exception is **3D rendering**, which is inherently presentation and belongs in `webizen-render`, delegating math to QualiaDB.

---

## 1. Confirmed PRESENT (engine already has it — surface, don't rebuild)

| Capability | Signal | Modules |
|---|---|---|
| Internal LLM (llama replacement) | strong | `llm_agent`, `gguf_bridge`, `gguf_sharder`, `ggml_quants`, `lora/`, `resident_model`, `directml_bridge`/`metal_bridge`/`npu_ffi`, `shaders/` |
| SPARQL 1.1 **+ SPARQL-star / RDF-star** | 12 files | `sparql_library`, `query_engine`, `rdf_star`, `ingest` |
| Logic suite (N3/OWL/SHACL/deontic/epistemic/paraconsistent/ASP/LTL) | strong | `modalities/`, `shacl_compiler`, `deontic_logic`, `epistemic` |
| q42 format + CBOR-LD + YAML-LD | strong | `q42_*`, `cbor_compiler`, `yaml_ld_q42` |
| **QDP client resolution** (4-tier cascade) | 4 files | `qualia-client-core/dns_resolver` (`did:q42:`→NS→`/.well-known/QDP`→TXT) |
| **Front Door DIDs** | 6 files | `identifier`, `webizen_identifiers`, client `api`/`dns_resolver` |
| **SocialWebNet / WireGuard** (DID-keyed mesh, keys via DNSSEC) | yes | `daemon_swarm` (`SocialWebNetInterface`, `DnssecResolver`, `establish_wireguard_tunnel`) |
| Geometric algebra (SIMD, non-euclidean) | yes | `geometric_algebra/` |
| wgpu **compute** | 5 files | `shaders/`, diffusion; `webizen-runtime` |
| Crypto / provenance / ZK | strong | `fiduciary_crypto`, `zk_proofs`, `provenance`, `key_vault` |
| Wallet rails (lightning/cashu/monero/ecash) | 4 files | `ilp_dispatcher`, `fiduciary_crypto` (depth TBD — see §3) |
| Bitcoin / DLT anchoring | 3 files | `provenance`-adjacent (OTS-style) |
| Scientific compute (medical/financial/physics/chem/ML/stats) | strong | `specialized_libs/`, `domains/`, `solvers/`, `ode_solver`, `quantum_dft`, `qubo_compiler` |
| P2P / distribution | strong | `p2p`, `webtorrent_seeder`/`webtorrent_routes`, `daemon_swarm`, `acoustic_ble_mesh` |
| Nym mixnet | yes | `nym_adapter` |

---

## 1b. Client crates — these close several "gaps" (audited 2026-06-15)

The earlier audit was core-db-centric. `qualia-client-core` (the crate `webizen-desktop` actually calls via `api::*`) and its siblings already provide a lot of what the feature matrix needs — these are **present, surface them**, not build:

| Capability | Module(s) | Effect on plan |
|---|---|---|
| **LLM chat system** (sessions, inference, retrieval, relay, agents, files, ontology, graph) | `qualia-client-core/chat_{session,inference,retrieval,relay,agents,files,ontology,graph}`, `inference_backend`, `model_lifecycle`, `model_preferences` | **F1/F2 largely present** — downgrade from "build" to "surface in Dioxus". |
| **Anatomy representation context** | `anatomy_context` (`AnatomyGraphContext`, `build_anatomy_graph_context_json`, DICOM overlay spec) | **The concrete `SceneSource` for the 3D engine** — F10's data source exists; align `render::qualia::SemanticScene` to its JSON. |
| **QApp container system** (manifest, capability claims, clearance, sandbox domains, asset server) | `qapp_manifest` (`QappManifest`/`CapabilityClaims`/`CompiledCapability`), `qapp_registry` (`QappPackageManifest`), `qapps_protocol` (loopback asset server), `qapp_mcp`, `qapp_version` | **The hypermedia format already has a foundation** — `.q42app` must *extend* this, not invent (see corrected `WEBIZEN_HYPERMEDIA_FORMAT.md`). |
| **Social contacts / invites** | `social_connect` (`ConnectInvitePayload`, `ChatContact`, `generate/accept_connect_invite`, signed invites, DID-keyed contacts), `guardianship` | **F12 + the social-network peer source** — SocialWebNet/WireGuard peers can be provisioned from these DID contacts. |
| **Solid + OIDC micro-IdP** | `qualia-solid-bridge` (`oidc_micro_idp::oidc_routes`, `solid_proxy`, `ldp_translator`) | Identity/auth + Solid Pod; an OIDC IdP endpoint already exists. |
| **WellFare / health** (whole crate) | `wellfare-core` (`models`, `n3_rules`, `shapes`, `store`, `wasm`, `webizen`) | **F6/F7 have a dedicated crate** — health vault + sanctuary logic foundation. |
| **QPU dispatch / oracle / pipeline** | `qualia-client-core/qpu_{dispatcher,oracle,pipeline}` | quantum QApps already wired client-side. |
| **Resource import / ingestion** | `resource_import`, `engine/ingestion`, `q42_compress` | F3 ingestion plumbing. |
| **Compute extensions** | `qualia-extensions` (`pinn_extension`, `snn_extension`, `qpu_extension`, `webgpu_extension`) | PINN/SNN/QPU/WebGPU **compute** (see note in §2). |

---

## 2. Confirmed MISSING (0 file matches — genuine gaps to build)

| Gap | Probe | Where it should be built | Blocks |
|---|---|---|---|
| **3D rendering** (render pipeline / vertex+fragment shaders / surface config) | 0 | **`webizen-render`** (new) — *presentation*, not engine; uses QualiaDB `geometric_algebra` for math. NB: `qualia-extensions/webgpu_extension` is GPU **compute** (fluid/EM physics), not render — the gap holds; the renderer can *share* its wgpu device. | 3D anatomy (F10), GPU benchmark, physics viz |
| **glTF / `.glb` asset loading** | 0 | `webizen-render` (a codec) | anatomy atlas, model import |
| **HCAI negotiation endpoint** (inbound agreement server) | 0 | new `qualia-core-db` surface or `webizen-desktop/webai/` (legacy located it here); enforce via `deontic_logic` | the entire inbound gatekeeper (F16) — *this is the chokepoint the browser is built around* |
| **Audio transcription (Whisper)** | 0 | `qualia-core-db` (neural layer / `llm_agent` multimodal) | ingestion F3 (audio) |
| **OCR** (image→text) | 0 | `qualia-core-db` ingestion (or `dicom_ingest`-style) | ingestion F3 (image-OCR) |

---

## 3. PARTIAL / needs verification (present but thin or unconfirmed depth)

| Capability | Signal | Concern |
|---|---|---|
| **QDP *serving*** (publishing one's own `did.json` + `/.well-known/QDP` + `_qdp` provisioning) | 3 files | Resolver *fetches* QDP well; the **outbound frontdoor server** is the "only genuinely new surface" per legacy notes. Verify a node can publish its own QDP/`did.json`. |
| **WebRTC** | 5 refs | Used for P2P SuperBlock *streaming* (CLI benches) + "reserved for future" telemetry. A full **browser WebRTC ingress** for calls + HCAI sessions (DTLS-SRTP) is not confirmed — likely needs a `webrtc` dep + a desktop surface. |
| **Multi-chain wallet depth** | 4 files | Rails referenced; the WellFare HCW phases (blind-proxy treasury, anon credentials, Lightning channel state) depth is unverified. |
| **Bitcoin/OTS anchoring** | 3 files | Present in some form; confirm it produces downloadable `.ots` proofs (Sanctuary Mode F7 needs this). |
| **`did:web` handling** | passthrough only | `dns_resolver` passes `did:web:`/`did:key:` through but `encode_did_for_ns("did:web:…")` returns `None` (test-confirmed) — only `did:q42:` is NS-encodable. Fine, but the frontdoor must serve `did:web` over HTTP, not NS. |

---

## 4. Implications for the plan (revised after the client-crate audit)

1. **3D engine work is validated.** Rendering exists nowhere in QualiaDB — even `qualia-extensions/webgpu_extension` is compute. `webizen-render` (scene-graph + `Renderer` + future `WgpuRenderer`) is the correct, non-duplicative home; math delegates to `geometric_algebra`, physics to the solvers, and it owns only *presentation*. Its concrete first data source is **`anatomy_context`** — align `render::qualia::SemanticScene` to `AnatomyGraphContext`.
2. **Less to build than first thought.** The client crates already cover **LLM chat (F1/F2)**, **anatomy data (F10)**, the **QApp container foundation**, **social contacts (F12)**, **Solid/OIDC**, and a whole **WellFare/health crate (F6/F7)**. These move from "build" to "**surface in Dioxus**".
3. **The hypermedia format must extend, not invent.** `qapp_manifest`/`qapp_registry`/`qapps_protocol` already give a manifest, capability claims, clearance/sandbox domains, chat handoff, and an asset server. `.q42app` formalises + q42-packages + signs + UI-declares on top (corrected in `WEBIZEN_HYPERMEDIA_FORMAT.md`).
4. **The HCAI negotiation endpoint is the highest-leverage true gap** (0 matches) — the single inbound door the browser thesis depends on (F16). Build next to `deontic_logic`, mirroring legacy `webai/hcai_agreement.rs`.
5. **Frontdoor serving** (own `did.json` + `/.well-known/QDP`) and **WebRTC ingress** are the two transport surfaces to confirm/build; **WireGuard datapath** (`WEBIZEN_NETWORK.md` §6.5) — logic exists, confirm OS datapath.
6. **Whisper audio + OCR** are real engine gaps but P2 (not MVP-blocking).

**Revised build order:** *surface* already-present client capabilities (chat, anatomy, contacts, QApp manifest) in Dioxus → HCAI endpoint → QDP/`did.json` serving → WireGuard command surface + datapath → WebRTC ingress → `webizen-render` `WgpuRenderer` → (later) Whisper/OCR.

---

## 5. Method caveat

File-match counts are a presence heuristic, not proof of completeness — a capability can be present-but-stubbed or absent-but-named-differently. The §3 "needs verification" items specifically warrant a function-level read before committing to build vs. surface. The §2 zeros are high-confidence (multiple synonyms probed) but a deeper read of `qualia-core-db` is the right confirmation before scheduling the HCAI/WebRTC/render work.
