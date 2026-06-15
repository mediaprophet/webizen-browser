# Webizen Hypermedia QApp Format (`.q42app`)

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Design proposal for ratification (Master Plan §11.4)
**Grounding:** extends existing `qualia-core-db` **and `qualia-client-core`** modules. No new serialization stack, no Node, no external engine.

> ✅ **Correction (2026-06-15, after the client-crate audit):** an existing QApp container system was found in `qualia-client-core` — `qapp_manifest` (`QappManifest`, `CapabilityClaims`, `CompiledCapability` with clearance + `permitted_domains[8]` sandbox), `qapp_registry` (`QappPackageManifest`: capability, engine bindings, `health_path`/`query_path`/`websocket_path`, intent, `supports_launch_from_chat`/`return_to_chat`, PINN-model requirements), and `qapps_protocol` (a loopback HTTP server serving qapp assets from `{storage}/Qapps/`). `.q42app` is therefore **an extension of this manifest**, not a greenfield format — it adds: q42-volume packaging, a *declarative Dioxus-pane UI section*, author-DID signing, WebTorrent distribution, and the capability-verb bindings below. Field names below should be reconciled with `QappPackageManifest` during ratification.

---

## 1. Why a format at all

Today the **built-in** QApps are hardcoded Dioxus components compiled into the studio binary (the existing `qapp_manifest`/`qapp_registry` system handles *installed HTML/WebView* qapps separately, but not the 274 disciplines). Neither path lets a discipline (or a third party) ship a **declarative, signed, distributable** QApp. The audit (`QUALIA_DB_LOGIC_AUDIT.md`) is explicit: QApps are **thin UI clients over QualiaDB compute**. So a QApp's *essence* is not code — it is:

1. a **declarative UI** (which panes, how arranged, what they bind to),
2. a set of **capability bindings** (which QualiaDB queries / rules / models / solvers it invokes),
3. its **ontologies & SHACL shapes**,
4. **provenance + a signature**.

All four are *data*. That means a QApp can be a **signed semantic graph** — a hypermedia document — rather than a JavaScript bundle. This is the natural Webizen artifact and it is what makes the 274 disciplines (and third-party apps) distributable.

> "Hypermedia" here is used in the HATEOAS sense: the container declares **affordances** (actions, links, bindings) as graph data; the Webizen runtime interprets them. The UI is *driven by* controls described in the graph, not by imperative code shipped per app.

---

## 2. The format is q42, not a new thing

A `.q42app` is a **q42 volume** (`q42_volume` / `q42_reader`) — the same 48-byte-NQuin container the engine already reads — carrying a QApp subgraph. Authoring is human-friendly via **`yaml_ld_q42`** (YAML-LD ⇄ q42); wire/storage form is q42 + **`cbor_compiler`** compression. Nothing new is invented; we define a *vocabulary* and a *profile* over existing machinery.

```
.q42app  ==  a signed q42 volume containing one qapp:Application subgraph
             (authored as YAML-LD via yaml_ld_q42, stored as CBOR-LD q42)
```

### 2.1 Two physical forms
- **Single-file** `name.q42app` — a self-contained q42 volume (manifest + UI + bindings + shapes + signature). Good for install/share.
- **Torrent-distributed** — content-addressed by q42 hash, seeded via `webtorrent_seeder` / `webtorrent_routes`; the magnet *is* the canonical id. Good for the marketplace.

---

## 3. The vocabulary (`qapp:` over existing QualiaDB terms)

A `.q42app` graph has one `qapp:Application` root. Authored YAML-LD (compiles to q42):

```yaml
"@context": "https://webizen.org/ns/qapp.q42.jsonld"
"@type": qapp:Application
id: q42:anthropology                      # content-addressed id (q_hash)
name: "Anthropology"
version: "1.0.0"
author: "did:q42:…"                        # webizen_identifiers / identifier
license: "CC-BY-NC-ND-4.0"

# (1) Declarative UI — reuses the studio WebizenWorkspace pane model
ui:
  launchModes: [full-app, panel, embedded-card]
  panes:
    - component: discipline-form
      bind: "#analysis"                     # a hypermedia control id
      layout: { x: 0, y: 0, w: 62, h: 62 }
    - component: engine-panel
      bind: "#commit"

# (2) Capability bindings — what QualiaDB does, declared as data
bindings:
  - id: "#analysis"
    capability: sparql                      # -> sparql_library
    query: |
      SELECT ?field ?value WHERE { … }
  - id: "#commit"
    capability: rule                        # -> modalities (N3) / deontic_logic
    rule: "shapes/anthropology.n3"
    provenance: required                    # -> provenance + fiduciary_crypto
  - id: "#assist"
    capability: llm                         # -> llm_agent (internal wgpu LLM)
    system: "You are an ethnographic analysis aide…"

# (3) Knowledge assets carried in the volume
shapes: ["shapes/anthropology.shacl"]       # -> shacl_compiler
ontologies: ["onto/anthropology.ttl"]       # -> ontology_loader

# (4) Sandboxing — explicit capability grants (no implicit access)
permissions: [graph-read, graph-write, llm, sparql]   # -> extension_manifest / extension_bus

# (5) Optional compiled logic
bytecode: "logic/anthropology.wbc"          # -> webizen_bytecode (Webizen VM)
```

### 3.1 Capability verbs (all map to existing modules)
| `capability:` | Executed by | Notes |
|---|---|---|
| `sparql` | `sparql_library`, `query_engine` | SPARQL 1.1 + SPARQL-star, streaming |
| `rule` | `modalities/` (N3/OWL), `deontic_logic`, `epistemic` | logic over the graph |
| `llm` | `llm_agent` + `gguf_bridge`/`lora` | internal wgpu LLM |
| `solve` | `solvers/`, `ode_solver`, `qubo_compiler`, `quantum_dft` | scientific compute |
| `shape` | `shacl_compiler`, `webizen_validator` | validation |
| `render3d` | `geometric_algebra` + `webizen-runtime` wgpu | 3D scenes (see 3D engine doc) |

The UI side never implements any of these; it issues a typed binding and renders the result. This is exactly the existing `qapp_engine`/`qapp_analyze` contract, generalized from one verb to the table above.

---

## 4. Trust: signed, verifiable, sandboxed

- **Signature:** the volume carries an author-DID-signed Merkle root (`fiduciary_crypto` / `key_vault`). The runtime verifies it **before** install/launch (`webizen_validator`).
- **Provenance:** every write a QApp makes is stamped (`provenance`) and attributable to the QApp id + author DID.
- **Sandbox:** `permissions` are explicit grants brokered by `extension_bus`; a QApp with only `graph-read` cannot write, call the LLM, or reach the network. No ambient authority.
- **Content address:** `id` = `q_hash` of the canonical graph, so a `.q42app` is tamper-evident and dedup-able; the torrent magnet derives from it.

---

## 5. Runtime: how the studio loads one

1. Fetch/open the `.q42app` (file, OPFS cache, or WebTorrent).
2. `webizen_validator` checks signature + SHACL self-consistency.
3. `q42_reader` parses the volume; the `qapp:Application` subgraph yields the manifest.
4. The Dioxus shell builds panes from `ui.panes` (the existing `WebizenWorkspace`/`pane_registry` path — already present).
5. Each binding routes through the generalized `qapp_engine` contract to the capability module; results render in the bound pane.
6. Provenance/permissions enforced per call by `extension_bus`.

The current hardcoded `qapp_dispatcher` becomes the **fallback/registry for built-in components**; user/third-party QApps arrive as `.q42app` data and need no recompile.

---

## 6. Migration path (incremental, low-risk)

- **Phase A:** define the `qapp:` YAML-LD context + a `.q42app` reader that produces the existing in-memory `WebizenWorkspace`. Round-trip one built-in QApp (e.g. Anthropology) through it.
- **Phase B:** generalize `qapp_engine` from the single `analyze` verb to the §3.1 capability table (sparql/rule/llm/solve/shape).
- **Phase C:** export the 274 disciplines as `.q42app` files (scriptable from the existing catalogue) and load them as data; the dispatcher keeps serving platform components.
- **Phase D:** signing + WebTorrent marketplace (`webtorrent_seeder`), permission brokering (`extension_bus`), third-party authoring via `yaml_ld_q42`.

---

## 7. Open questions for ratification

1. **Context URL / vocabulary namespace** — confirm `https://webizen.org/ns/qapp.q42.jsonld` and the `qapp:` term set.
2. **UI vocabulary scope** — how much layout/interaction is declarable before a QApp needs `webizen_bytecode` logic? (Start minimal: panes + bindings; escalate to bytecode only when needed.)
3. **Bytecode boundary** — what logic must be compiled vs. expressible as N3/SPARQL bindings?
4. **Versioning & dependencies** — semantic-version ranges for required engine capabilities / ontologies.
5. **Extension vs. QApp** — relationship between `.q42app` and the existing `extension_manifest` (likely: a QApp *is* an extension profile with a UI section).

---

*Everything above is q42 + existing `qualia-core-db` modules. The format adds a vocabulary and a loader, not a new engine or a JS toolchain. A QApp becomes a signed, content-addressed semantic document — the natural unit of the Human-Centric Internet.*
