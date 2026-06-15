# Webizen — Review Questions for the First 3D Human-Centric Internet Browser

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Audience:** reviewing agents (and humans) evaluating the design against the user's library of resources.
**Purpose:** a structured set of open questions — with context and expansion guidance — so other agents can (a) answer/critique them, and (b) hold *any other resource* in the library up against the project and report what it implies.

---

## 0. How to use this document

You are helping define the **first 3D-enabled, human-centric internet browser**, built in **Rust/Dioxus over the QualiaDB engine**. Before answering any question:

1. **Read the corpus** (the planning docs in this repo root, `WEBIZEN_*.md`):
   `WEBIZEN_MASTER_PLAN`, `_NETWORK`, `_HYPERMEDIA_FORMAT`, `_3D_ENGINE`, `_3D_NATIVE_VISION`, `_REALMS`, `_QUALIADB_GAPS`, `_REORG_NOTES`, `_FLUTTER_REVIEW`, `_APP_LIFECYCLE`, `_SWARM`, `_DESIGN_PHILOSOPHY`, plus `PROJECT_REVIEW`, `QUALIA_DB_LOGIC_AUDIT`.
2. **Respect the non-negotiable constraints** (§1). Suggestions that violate them are out of scope.
3. **Distinguish verified from assumed.** Much is grounded in `qualia-core-db` / `qualia-client-core` source; some is design. When you answer, say which you're relying on, and **re-verify against the live QualiaDB crates** — the backend evolves and parts of older references (esp. the Flutter FFI) are stale.
4. **Answer in the feedback format** (§ end).

---

## 1. Non-negotiable constraints (do not propose violating these)

- **No Node.js / no JS build chain.** Pure Rust/Dioxus → wasm + native. Browser APIs via `web-sys` are fine; JS *libraries* and Node are not.
- **No external compute/engine crates.** SPARQL, LLM, logic, physics, crypto, geometry, DNS all come from **QualiaDB**, not oxigraph/candle/llama/etc. The UI is a thin client; the engine computes.
- **Human-centric / dignity-first, WellFare threat model.** Consent-first, provenance-first, accessible (reduced-motion), calm-by-default. Some users are fleeing trafficking/DV — Sanctuary/Duress/Dead-Man's-Switch are core, not optional. Never optimise for attention capture.
- **Legacy is reference only.** `legacy/` (SvelteKit) and the Flutter export are *feature/UX references*; their code is not ported, it is re-authored in Rust/Dioxus.

---

## 2. How to evaluate ANY library resource (the rubric)

For each resource the user points you at, answer these six:

1. **Claim:** what is the resource's core idea, in one sentence?
2. **Fit:** which principle (`WEBIZEN_DESIGN_PHILOSOPHY` P1–P7) or layer (realm / network / 3D / hypermedia / swarm / engine) does it bear on?
3. **New capability?** does it imply a capability we have not planned, or contradict one we have?
4. **Grounding:** is the implied capability already in QualiaDB (cite the module), or a gap?
5. **Constraint check:** does adopting it violate §1? If so, is there a QualiaDB-native / Rust-native way to get the same value?
6. **Action:** concrete change to a named doc, or "no change — already covered by X."

> Especially valuable: resources that reveal a **missing engine capability**, a **human-centric risk we haven't named**, or a **representation metaphor** for a realm.

---

## 3. Domain question sets

Each question: **the question**, *Context* (what we believe + grounding), *Expand on* (angles + which resources/code to consult).

### A. Foundational philosophy & ethics
- **A1. What is the single test a design decision must pass to be "human-centric" here?**
  *Context:* `WEBIZEN_DESIGN_PHILOSOPHY` proposes "participant-steward, not resource to extract." We need a crisp, usable rubric.
  *Expand:* propose a 3–5 question checklist a designer applies per feature; reconcile with the WellFare dignity language ("inalienable", "self-determined", "care credits"); stress-test against a concrete feature (e.g. notifications).
- **A2. Where is the line between "calm, meaningful motion" and "engagement-bait"?**
  *Context:* P1 (attention is sacred) forbids capture but we want delight (animated icons).
  *Expand:* define measurable criteria (frequency, initiation, dismissibility, semantic necessity); map to a Calm/Standard/Off setting; consider the Stapp "attention is efficacious" framing — does drawing attention have a moral cost?
- **A3. Does the Stapp "Process 1 / Quantum Zeno + attention" argument give precise language for the omnibox-as-intention?**
  *Context:* we read the omnibox as "Process 1" (the chosen question). *This rests on the established thesis, not the transcript.*
  *Expand:* request/ingest the transcript; if it holds, articulate "the query is the act of mind that collapses possibility into a realm-view"; if not, revise P1/P2.

### B. Realms / the engagement environment
- **B1. Is a realm exactly one context, or a context *query*?** (composability/nesting)
  *Context:* `WEBIZEN_REALMS` leans toward a context *query* (union/scope/nest). Grounded in the NQuin context vector (~71 files).
  *Expand:* design the `RealmBinding` type; how do nested project-realms inherit/override context; how does AS-OF time compose with realm scope.
- **B2. What is the *right set* of default realms, and are they the user's (personal/local/social/spatial/mind/real-world) or fewer/more?**
  *Expand:* test the taxonomy against real tasks; is "Local" distinct enough from "Personal"; should "Mind" and "Knowledge/disciplines" merge; consult any of the user's resources on cognitive/spatial models.
- **B3. What is each realm's *signature representation*, and how is realm-switch navigated without disorientation?**
  *Context:* the 3D vision gives metaphors (constellation/map/branching-worlds). Disorientation + reduced-motion are risks.
  *Expand:* propose camera/transition grammar; a "realm compass"; reduced-motion equivalents; how a colour-blind / low-vision user perceives realm boundaries.
- **B4. Should "Selfhood/Personhood zones" become real engine primitives over the context vector, or stay a UI grouping?**
  *Grounding:* the zones are UI language today, not named in code. Affects whether `qualia-core-db` gains a `realm`/`zone` concept.

### C. 3D engine, rendering & motion
- **C1. `WgpuRenderer`: WebGPU-first with WebGL fallback, and how does it share the wgpu device with QualiaDB compute (`webizen-runtime`, `qualia-extensions/webgpu_extension`)?**
  *Context:* render is a confirmed gap (compute exists, render doesn't). The engine + `Renderer` trait exist (`webizen-studio/src/render`).
  *Expand:* device-sharing design (avoid CPU round-trips); WebGL fallback scope; how `geometric_algebra` (native SIMD) feeds the renderer while wasm uses the f64 path.
- **C2. What belongs in `render::motion` (the animation dev-kit), and where does it live (`webizen-render` crate)?**
  *Context:* proposed Timeline/Spring/Anim/IconScene, reduced-motion-gated, semantic-driven.
  *Expand:* API sketch; how `qapp_engine` events drive motion targets; the cheap SVG/CSS path vs. mesh path; performance budget (off-screen idle, battery).
- **C3. Is the Mind realm (knowledge/epistemic graph you navigate) the flagship 3D experience rather than anatomy?**
  *Context:* the design philosophy (P4) argues yes; anatomy has a ready data source (`anatomy_context`).
  *Expand:* compare effort/impact; what graph-layout algorithm in Rust; how SPARQL-star/`rdf_star` results map to a navigable `SemanticScene`.

### D. Hypermedia QApp format & editability
- **D1. How do we unify the THREE existing manifest shapes into one `.q42app`?** (`qapp.json`+`x_qualia`, `QappPackageManifest`, the proposed `.q42app`)
  *Context:* `WEBIZEN_HYPERMEDIA_FORMAT` must *extend*, not invent. Field reconciliation needed.
  *Expand:* produce a unified schema mapping each existing field; what's declarative-UI vs. `webizen_bytecode`; the `qapp:` vocabulary/namespace.
- **D2. How much UI is safely declarable before a QApp needs code, so "every QApp is editable by non-programmers" holds?**
  *Context:* the editability rule (`WEBIZEN_APP_LIFECYCLE`). Panes + bindings now; bytecode for advanced.
  *Expand:* define the declarative pane/binding grammar; the fork + re-sign flow; how the 274 disciplines export to editable `.q42app`.
- **D3. What's the sandbox/permission model for third-party `.q42app`s, beyond the existing clearance + `permitted_domains[8]` + `extension_bus`?**
  *Expand:* capability-verb gating (sparql/rule/llm/solve/render3d); consent prompts; what a malicious `.q42app` could attempt and how it's contained.

### E. Network / socially-defined internet
- **E1. Where does the HCAI negotiation endpoint live and how is it built?** (the highest-leverage gap — 0 matches in the engine)
  *Context:* the single inbound chokepoint the whole "browser" thesis depends on. Legacy put it in `webai/hcai_agreement.rs`; the gap audit leans engine-side for reuse.
  *Expand:* engine (`qualia-core-db`) vs shell (`webizen-desktop`); the agreement NQuin schema; "structural minimisation is the only authorization"; enforcement via `deontic_logic`; consult the HCAI-ANP standard in the qualiaDB standards folder.
- **E2. What's the plan to *serve* (not just resolve) QDP — own `did.json` + `/.well-known/QDP` + `_qdp` provisioning?**
  *Context:* resolution exists (4-tier `dns_resolver`); serving is "the only genuinely new surface" per legacy. The QDP IETF draft specifies it.
  *Expand:* the serving surface; `did:web` over HTTP vs `did:q42:` over NS; DNS-AID compatibility; relationship to the desktop's localhost server.
- **E3. SocialWebNet/WireGuard: what's the OS datapath integration, per platform?**
  *Context:* the *logic* exists (`daemon_swarm`: `establish_wireguard_tunnel`, keys via DNSSEC). The datapath (kernel `wg` / wintun / userspace `boringtun`) is the open item.
  *Expand:* per-OS plan (Windows/macOS/Linux); userspace vs kernel; how Front-Door-DID peers map to tunnel config; the Tauri command surface.
- **E4. How does the omnibox become a true protocol router + "Process 1" surface?**
  *Expand:* `qdp://`/`did:q42:`/`webizen://`/`qualia://`/`https://` handling; rendering Front-Door resolution; switching realms; the Shield/consent affordance always one action away.

### F. Swarm / collaborative compute
- **F1. What is the trust model for swarm peers — SocialWebNet/WireGuard (trusted) only, or opt-in semi-trusted pools?**
  *Context:* `WEBIZEN_SWARM`; the machinery exists (`daemon_swarm` WorkerCells, `ambient_orchestration`, `crdt` consensus). Recommend trusted-only first.
  *Expand:* threat model (a malicious peer returning wrong results); verification (re-compute quorum / `zk_proofs` / trust+provenance); consult any library resources on verifiable/confidential computing.
- **F2. How are swarm contributions attributed and (optionally) credited?**
  *Context:* ties to cooperative obligation/attribution (`port_requirements` §4).
  *Expand:* contribution provenance; µ-units vs care-credits vs provenance-only; the "lend my compute" revocable control + budget (power/network conditions).

### G. Engine gaps & verification
- **G1. For each confirmed gap (HCAI endpoint, 3D render, glTF, Whisper, OCR) — build in QualiaDB or in the browser layer?**
  *Context:* render is presentation (→ browser); HCAI is engine-or-shell; Whisper/OCR are engine (ingestion).
  *Expand:* per-gap home + priority; what "minimal viable" looks like for each.
- **G2. Which "present" capabilities are actually stubbed vs. complete?** (file-match presence ≠ completeness)
  *Expand:* function-level read of `chat_*`, `anatomy_context`, `qapp_registry`, wallet rails, OTS anchoring; report which are wired vs. scaffold.

### H. App lifecycle, mobile & desktop
- **H1. Is wasm-in-webview the default desktop personal-app runtime, with native Rust as an export step?**
  *Expand:* confirm; define the native-export packaging (Tauri-per-app vs shared host); when an app "graduates" to native.
- **H2. What is the one-click (or CLI) "export to mobile" build, from `.q42app` → Android/iOS PWA?**
  *Context:* `qualia-mobile-harness` is the template (OPFS, `android_/ios_pwa_edge`, minimal-512 engine).
  *Expand:* templating the build; replacing the harness's inline `html5-qrcode` JS with `web-sys BarcodeDetector`; offline/sync story (CRDT).

### I. Migration, reorg & naming
- **I1. Execution order for the reorg (chat rename, `webizen-render` crate, qualia-chat consolidation) without breaking the build?**
  *Context:* `WEBIZEN_REORG_NOTES`; the `chat_graph.rs → neuro_symbolic_chat.rs` rename is the safe first step.
- **I2. Re-verify the Flutter feature parity list against today's `qualia-client-core` — which FFI names/signatures drifted?**
  *Context:* the Flutter api-reference predates backend updates.

### J. Safety, accessibility, performance
- **J1. Does every realm/motion/feature have a reduced-motion + screen-reader + keyboard equivalent, and a Sanctuary-mode-safe (shoulder-surf-safe) variant?**
- **J2. What are the memory/perf budgets (512 MB WorkerCells, mobile RAM, off-screen idle) and how are they enforced in the UI?**

---

## 4. Cross-cutting / meta questions

- **M1. What is the *one* demo that proves "a 3D human-centric internet" in 90 seconds?** (the thing to build first for conviction)
- **M2. Where could this design be *capturing* rather than *serving* the user, despite our intentions?** (red-team P1/P6)
- **M3. What have we over-engineered relative to a credible first release?** (cut list)
- **M4. What's missing entirely that none of the docs mention?** (the unknown unknowns — the highest-value find)
- **M5. Which decisions are reversible vs. one-way doors?** (sequence the one-way doors for the most deliberation)

---

## 5. Feedback format (for reviewing agents)

For each question (or resource) you address, return:

```
[ID or resource name]
Verdict:    agree / disagree / refine / blocked-on
Grounding:  verified-in-code (cite module) | designed/assumed | external-resource
Answer:     (concise; name trade-offs, not just a pick)
Constraint check: passes §1 | violates §1 because … (+ native alternative)
Doc impact: edit <DOC>.md §X  |  new doc  |  no change (covered by X)
Confidence: high / medium / low  + what would raise it
```

And, if reviewing a **library resource**, additionally fill the §2 six-point rubric.

> Bias to: naming a missing capability, a human-centric risk, or a sharper representation metaphor — over restating what the corpus already says. Cite QualiaDB modules when you claim something exists; flag when you couldn't verify.
