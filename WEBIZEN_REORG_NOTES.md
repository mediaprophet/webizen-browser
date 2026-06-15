# Webizen / QualiaDB — Reorganisation Notes

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Notes for ratification (not yet executed — these are *recommendations*, no code moved).
**Scope:** crate/module boundaries across the two repos after auditing all 8 QualiaDB crates + the webizen-browser crates.

---

## 0. The two-repo boundary (intended dependency direction)

```
   webizen-browser/                         qualiaDB/crates/
   ├─ webizen-studio   (Dioxus wasm UI) ───▶ qualia-client-core ───▶ qualia-core-db
   ├─ webizen-desktop  (Tauri shell)    ───▶ qualia-client-core, qualia-core-db
   └─ webizen-runtime  (wgpu compute)         (+ qualia-solid-bridge, wellfare-core,
                                               qualia-extensions, qualia-cli, …)
```

**Rule of placement:** *engine / logic / data* → QualiaDB crates; *presentation / rendering / shell* → webizen-browser. Dependencies point **browser → QualiaDB**, never the reverse. Any browser module that re-implements engine logic is a smell (should be a thin client over a QualiaDB call).

---

## 1. 3D rendering — confirmed in webizen-browser (correct), promote to a crate

Per the gap audit, rendering exists nowhere in QualiaDB (only compute). So the 3D engine correctly lives in **webizen-browser**:

| Piece | Today | Recommendation |
|---|---|---|
| Scene graph, mesh, `Renderer` trait, Canvas2D backend, QualiaDB scene binding | `webizen-studio/src/render/` | Promote to a workspace crate **`webizen-render`** once `WgpuRenderer` lands, so both `webizen-studio` (wasm) and `webizen-desktop` (native) consume it. |
| wgpu **compute** (diffusion kernel) | `webizen-runtime/` | Keep as the compute crate; the future `WgpuRenderer` **shares its `wgpu::Device`** with `qualia-extensions/webgpu_extension` compute. Consider folding `webizen-runtime` into `webizen-render` as a `compute` module, or keep separate and have `webizen-render` depend on it. |
| Math / physics | — | Delegate to QualiaDB `geometric_algebra` (native) + solvers; do **not** add math to `webizen-render`. |

**Do this *after* `WgpuRenderer` exists** to avoid churn. Until then `render/` stays a module in `webizen-studio` (compiles to wasm, tested).

---

## 2. Chat graph — three copies, one misnamed (needs cleanup)

| Location | What it actually is | Action |
|---|---|---|
| `qualia-client-core/src/chat_graph.rs` | **The real engine:** branching-conversation **DAG** — `ChatFragment`, `ChatGraphEdge`, `ChatGraphSnapshot`, `create_fragment_from_selection`, `link_reply_to_fragment`, `build_thread_context_block`; WAL + NQuin backed. | **Keep — this is canonical.** Surface it via Tauri commands. |
| `webizen-studio/src/components/chat_graph.rs` | **Misnamed:** defines `NeuroSymbolicChat`, a Dioxus *UI pane* (Shoelace). It is **not** the chat graph. | **Rename → `neuro_symbolic_chat.rs`** (update `mod.rs` + dispatcher tag). Then a *new, correctly-named* chat-graph UI surfaces the client-core DAG above. |
| `legacy/webizen-studio/src/components/chat_graph.rs` | Retired SvelteKit-era duplicate. | Leave in `legacy/` (reference only). |

The naming collision is actively confusing ("the chat graph code" could mean the DAG engine *or* the misnamed UI pane). Renaming the browser file removes the ambiguity and is a safe, mechanical change.

> Related: the full chat system is `qualia-client-core/chat_{session,inference,retrieval,relay,agents,files,ontology,graph}` — eight modules. The browser should have **one** chat feature area (Dioxus) that surfaces all of them, not scattered, ad-hoc panes.
>
> **Naming:** call that consolidated browser chat feature **`qualia-chat`** (module `webizen-studio/src/qualia_chat/`, or a `webizen-qualia-chat` crate if it grows). It surfaces the client-core `chat_*` engine modules; the misnamed `chat_graph.rs` pane folds into it. The Flutter app (§3a) shows the full target shape — chat with a DAG graph panel, citation/provenance chips, files, images, reactions, sub-agents, environment bar.

---

## 3. The other crates — what they are + reorg implications

All 8 QualiaDB crates are now accounted for:

| Crate | Role | Reorg note |
|---|---|---|
| `qualia-core-db` | The engine (logic, SPARQL, LLM, q42, geometric algebra, daemon/SocialWebNet). | Canonical engine. Missing: HCAI endpoint, render (by design). |
| `qualia-client-core` | Client API the browser calls (`api::*`): chat, anatomy, QApp manifest/registry, social-connect, qpu, ingestion, dns_resolver. | **Primary integration surface.** Most "build" items are really "surface this". |
| `qualia-cli` | CLI over the engine (solve/science/qpu/ingest/query/bench). | Reference for capability + a non-UI driver; not consumed by the browser. |
| `qualia-extensions` | Compute extensions: `pinn`, `snn`, `qpu`, **`webgpu`** (all *compute*, e.g. fluid/EM). | `webgpu_extension` is compute, **not** render — confirms the render gap. `webizen-render` shares its device. |
| `qualia-solid-bridge` | Solid Pod + **OIDC micro-IdP** (`oidc_routes`), LDP translator, proxy. | Identity/auth surface for the browser's frontdoor/login. |
| `wellfare-core` | WellFare/health crate: models, n3_rules, shapes, store, wasm bindings. | F6/F7 (Health Vault / Sanctuary) foundation. Browser surfaces it. |
| `webizen-component-harvester` | Tool: fetches Shoelace `custom-elements.json` (CEM) from unpkg, generates Rust bindings. | **Ties to the Shoelace decision (Master Plan §11.1).** If keeping Shoelace, this is the maintained generator for `webizen-studio/src/components/shoelace.rs`. If going native-Dioxus, deprecate it. Either way it's a build-time *tool*, not a runtime dep. |
| `qualia-mobile-harness` | Dioxus **mobile** app; camera/QR via inline `html5-qrcode` **JS**. | ⚠️ **No-JS-rule violation:** the inline `html5-qrcode` should be replaced by `web-sys` `BarcodeDetector`/`getUserMedia` (W3C API). Flag when the barcode/diet feature is built. Also: is mobile a *webizen-browser* target eventually? (decision §5). |

---

## 4. Recommended target structure (concern → home)

| Concern | Home |
|---|---|
| Engine logic, SPARQL/RDF-star, LLM, q42, geometric algebra, DNS/SocialWebNet | `qualia-core-db` |
| Client API the UI calls (chat, anatomy, qapp manifest, contacts, ingestion) | `qualia-client-core` |
| Identity / Solid / OIDC | `qualia-solid-bridge` |
| Health / sanctuary | `wellfare-core` |
| GPU **compute** | `webizen-runtime` (+ `qualia-extensions/webgpu`) |
| 3D **rendering** + scene graph + dev-kit | **`webizen-render`** (new; promoted from `webizen-studio/src/render`) |
| Dioxus UI, QApps, panes, omnibox, shield | `webizen-studio` |
| Tauri shell, tray, daemon relay, frontdoor server, HCAI endpoint, commands | `webizen-desktop` |
| Shoelace binding generation (if kept) | `webizen-component-harvester` (build-time tool) |
| Mobile | `qualia-mobile-harness` (de-JS first) |

---

## 5. Migration checklist (incremental, non-breaking)

- [ ] **Rename** `webizen-studio/src/components/chat_graph.rs` → `neuro_symbolic_chat.rs` (file + `mod.rs` + dispatcher tag + any pane registry id). Mechanical; covered by the catalogue↔dispatcher test.
- [ ] Add Tauri commands surfacing `qualia-client-core/chat_graph` (the DAG) + the other `chat_*` modules; build one Dioxus chat feature area.
- [ ] When `WgpuRenderer` lands: promote `render/` → `webizen-render` crate; decide `webizen-runtime` fold-in vs. depend-on.
- [ ] Decide Shoelace (Master Plan §11.1); keep/deprecate `webizen-component-harvester` accordingly.
- [ ] Replace `qualia-mobile-harness` inline `html5-qrcode` JS with `web-sys` `BarcodeDetector` before relying on it.
- [ ] Add an HCAI negotiation endpoint home — `webizen-desktop/src/webai/` (mirroring legacy) or a new `qualia-core-db` surface (decision in the gap audit).

---

## 6. Open questions for ratification

1. **`webizen-render` vs `webizen-runtime`:** one crate (render + compute) or two (render depends on runtime)? Recommend **two**, since compute is reusable headless (it already powers the diffusion benchmark).
2. **HCAI endpoint home:** engine (`qualia-core-db`) or shell (`webizen-desktop/webai`)? Legacy put it in the shell; the gap audit leans engine for reuse. Pick one.
3. **Shoelace:** keep (maintained by the harvester) or go native Dioxus? Affects whether the harvester crate stays.
4. **Mobile scope:** is `qualia-mobile-harness` in scope for this browser effort, or a separate track? If in scope, it needs the same no-JS treatment as the desktop UI.

*No code has been moved. This file records the recommended reorganisation so it isn't lost; execute items in §5 individually, each behind the existing test + `cargo check` gates.*
