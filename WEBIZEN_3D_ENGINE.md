# Webizen 3D Engine — Design & Dev Kit

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Foundation implemented (compiles, tested), GPU backend + QualiaDB-native math pending.
**Mandate:** replace Babylon.js / three.js with an internal Rust engine, give it a developer kit, and **design it to use QualiaDB** (geometric algebra, physics, diffusion, query-driven scenes). No JS 3D libraries. No external 3D crate.

---

## 1. What exists now (`webizen-studio/src/render/`)

A backend-agnostic, immediate-mode engine + dev kit, all pure Rust, compiling to wasm and passing unit tests:

| Module | Role |
|---|---|
| `scene.rs` | `Vec3`, `Camera` (look-at + projection), `ScreenPoint`. |
| `mesh.rs` | `Transform` (pos/euler/scale) + `Mesh` and primitive builders: `cube`, `quad`, `grid`, `uv_sphere`, `line`. **Dev-kit geometry** — replaces three.js/Babylon primitives. |
| `graph.rs` | `Scene` / `Node` / `Style` scene graph + `Scene::render()` walker that drives any backend; nested transforms compose without matrices. |
| `qualia.rs` | **The QualiaDB binding:** `SemanticScene` / `SceneItem` (graph-query results with state + intensity + **provenance**), `item_color` heatmap, `build_scene(sem, camera, layout)`. |
| `mod.rs` | `Renderer` trait + `prelude`. |
| `canvas2d.rs` | CPU reference backend (Canvas 2D). The physics simulator already renders through it. |

```rust
use crate::render::prelude::*;

let scene = Scene::new(Camera::default())
    .add(Node::new("core").with_mesh(Mesh::uv_sphere(1.0, 12, 16)).with_style(Style::solid("var(--qualia-accent)")))
    .add(Node::new("floor").with_mesh(Mesh::grid(8.0, 16)).with_style(Style::wire("var(--qualia-border)")));
scene.render(&mut renderer);   // renderer: any Renderer (Canvas2D today, WebGPU next)
```

---

## 2. Architecture: one trait, swappable backends

Geometry is submitted in **world space**; each backend projects. The `Renderer` trait (`viewport / set_camera / clear / project / line / point / fill_polygon`) is the seam:

```
Scene graph ──▶ Renderer trait ──▶ ┌ Canvas2dRenderer  (CPU, wasm)        ← exists
                                    ├ WgpuRenderer      (WebGPU + native)  ← next
                                    └ (test/headless)
```

The CPU backend keeps the public demo working with zero GPU requirement; the GPU backend unlocks real performance and is where QualiaDB's GPU math plugs in (§4).

---

## 3. The semantic layer — why this isn't "just a renderer"

Babylon/three render *geometry you hand them*. The Webizen engine renders **meaning from the graph**. `qualia.rs` makes scenes a *projection of QualiaDB query/inference output*:

```
QualiaDB  ── SPARQL / RDF-star / N3 rules ──▶  SemanticScene { items[: id,state,intensity,provenance,reasons], explanations }
                                                        │  build_scene(sem, camera, layout)
   layout (atlas / graph embedding / field) ───────────┤
                                                        ▼
                                                  Scene ──▶ Renderer
```

- **Semantics** (what is shown, how hot, *why*) come from QualiaDB and carry `provenance` back to the justifying NQuin.
- **Geometry** (where, what mesh) comes from a `layout` the consumer supplies, so one semantic scene drives a body map, a knowledge-graph embedding, or a physics field.
- The anatomy use-case from `port_requirements.md` is exactly this: health observations → `modalities` N3 rules → organ items with intensity → `build_scene` with an anatomical-atlas layout → highlighted organs whose color encodes inference confidence and whose label links to evidence.

This is the concrete meaning of "designed to make use of the QualiaDB backend."

---

## 4. Designed to use QualiaDB (the work ahead)

The engine is structured so the heavy lifting is **delegated to QualiaDB**, never reimplemented:

| Engine need | QualiaDB module | Plan |
|---|---|---|
| Vector / matrix / non-euclidean math | `geometric_algebra/` (SIMD kernel) | Native `WgpuRenderer` and transforms call into `geometric_algebra` instead of the f64 fallbacks in `scene.rs`/`mesh.rs`. Keep the pure-Rust path for wasm where the kernel isn't available. |
| Physics / fields | `domains::physical`, `quantum_dft`, `ode_solver`, diffusion `shaders/` | Physics surfaces become `SemanticScene`s fed by these solvers (the existing `webizen-runtime` diffusion is the first instance). |
| Scene from a query | `sparql_library`, `rdf_star`, `modalities` | A `SceneSource` impl that runs a SPARQL/SPARQL-star query (or N3 rule) and returns a `SemanticScene`. |
| Provenance / signing of rendered facts | `provenance`, `fiduciary_crypto` | `SceneItem.provenance` already carries the `q42:` hash; the inspector surfaces it. |
| GPU compute interop | `webizen-runtime` wgpu + QualiaDB `shaders/` | Share the wgpu device between QualiaDB compute and the `WgpuRenderer` so compute output can be rendered without a CPU round-trip. |

**Rule:** the engine owns *presentation* (scene graph, projection, draw calls). All *math/physics/inference* that has a QualiaDB implementation is called, not duplicated.

---

## 5. Dev Kit — current surface and roadmap

**Have:** `prelude`, primitive builders, scene-graph builders (`Node::new().with_mesh().at().child()`), `Style::wire/solid`, `Camera`, `build_scene` for semantic scenes, `item_color` heatmap.

**Next (incremental, each its own task):**
1. **`WgpuRenderer`** behind the existing trait (WebGPU in browser, native on desktop) — the headline unlock.
2. **Orbit/arc-ball camera controls** + interaction (pick/hover → `SceneItem` → evidence callback).
3. **glTF/atlas loading** as a `layout` provider (Rust `gltf` parsing is a *codec*, not an engine — acceptable; geometry then flows through QualiaDB math).
4. **`SceneSource` over SPARQL** — a helper that turns a query string into a live `SemanticScene` via the `qapp_engine` transport (a new `render3d` capability verb in the hypermedia format).
5. **Lighting/shading model** in the GPU backend (flat → Lambert → PBR as needed).
6. **Engine benchmark** that runs in the *browser* (WebGPU) so the demo measures real GPU work — closing the gap the benchmark harness exposed.

---

## 6. Where the engine should live (crate boundary)

Today it's a module in `webizen-studio` (so it compiles to wasm and is testable now). As the GPU backend and QualiaDB math binding land, promote `render/` to a workspace crate **`webizen-render`** depending on `wgpu` and (native) `qualia-core-db::geometric_algebra`, consumed by both `webizen-studio` (wasm) and `webizen-desktop` (native). This mirrors how `webizen-runtime` already isolates wgpu compute. Do this *after* the `WgpuRenderer` exists, to avoid churn.

---

## 7. Summary

- The internal engine + dev kit is **real and tested** (CPU backend, scene graph, primitives, semantic binding).
- It is **designed around QualiaDB**: scenes are graph-query/inference output with provenance; math/physics delegate to `geometric_algebra` / solvers / diffusion shaders; GPU compute and rendering share the wgpu device.
- No Babylon, no three.js, no external 3D crate. The next concrete step is the `WgpuRenderer`, which simultaneously unlocks performance, real in-browser GPU benchmarking, and the path to GPU-resident QualiaDB math.
