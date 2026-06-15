# Webizen Browser — Project Review

**Reviewer:** Claude (Opus 4.8)
**Date:** 2026-06-15
**Branch reviewed:** `clean-main` (→ `main`)
**Scope:** Full workspace — `webizen-studio` (wasm/web UI), `webizen-desktop` (Tauri shell), `webizen-runtime` (wgpu compute), CI, repo hygiene.

---

## 1. Executive Summary

Webizen Browser is an ambitious three-crate Rust workspace:

| Crate | Role | Target | State |
|-------|------|--------|-------|
| `webizen-studio` | Dioxus UI: dashboard, 274 academic QApps, QApp Studio canvas | `wasm32` (web) + native | **Live demo deployed** |
| `webizen-desktop` | Tauri v1 shell: 78 commands, system tray, daemon, wallet, updater | native | Builds locally only |
| `webizen-runtime` | wgpu compute diffusion kernel + fixed-step clock + ledger | native | **Genuinely solid** |

**The good:** The `webizen-runtime` wgpu backend is real, well-structured GPU compute code (ping-pong storage buffers, staging readback, state hashing). The QApp catalogue is now comprehensive (274 disciplines wired end-to-end: catalogue → dispatcher → component). The theme engine in the shell is elegant. CI deploys to GitHub Pages.

**The gaps that matter most:**
1. **No real browser-side 3D/GPU.** The "GPU benchmark" only works inside the Tauri desktop app; the web demo has no WebGL/WebGPU path. The physics simulator is a CPU software rasterizer drawing to Canvas 2D.
2. **All 274 QApps ignore the theme engine** — every one hardcodes a Catppuccin dark palette, clashing with the shell's "Human Warmth" light theme.
3. **The 274 QApps are presentational shells** — they collect form input but do not actually call QualiaDB logic (the stated mission in `QAPP_IMPLEMENTATION_HANDOVER.md`).
4. **Boot-path panic risks** in `telemetry.rs` (`.unwrap()` on `local_storage()`).
5. **Repo hygiene:** ~26 GB `legacy/` folder still on disk, stray `fix*.py` scripts, no tests in the studio crate.

This document inventories everything found, grouped by severity, and closes with an enhancement roadmap.

---

## 2. Architecture Notes

- **Workspace split is correct** but `webizen-studio/Cargo.toml` declares its own empty `[workspace]`, detaching it from the root workspace. This was a deliberate CI fix (isolating it from the path-dep) but means `cargo build` at the root does **not** build the studio. Document this explicitly or unify.
- **`qualia-core-db` is an out-of-tree path dependency** (`../../qualiaDB/...`). CI works around it by sparse-cloning the sibling repo at build time. This is fragile: the demo build depends on a *second* GitHub repo staying public and structurally stable. The wasm build correctly stubs it out via `#[cfg(not(target_arch = "wasm32"))]`, but the native desktop build cannot be built by anyone who doesn't have `qualiaDB` checked out as a sibling.
- **Two runtime worlds:** `webizen-runtime` (wgpu, native) is invoked only through Tauri commands (`get_latest_diffusion_snapshot`, `reconfigure_diffusion`, …). The web demo cannot reach any of it — hence the benchmark harness was crashing in-browser (now guarded).

---

## 3. Critical Issues (break the product / block users)

### C1 — Web demo has no GPU/3D path *(architectural)*
The user's "GPU work" lives in `webizen-runtime` (wgpu compute) and is reachable only via Tauri `invoke()`. In a plain browser `window.__TAURI__` is undefined.
- **Fixed (this session):** `benchmark_harness.rs` now guards with `is_tauri()` and fails gracefully instead of crashing the wasm runtime.
- **Still open:** there is no browser-native benchmark. To benchmark GPU *in the demo*, a `wgpu`-on-WebGPU or WebGL path is required (see §8 and Roadmap R1).

### C2 — Boot-path panics in `telemetry.rs`
```rust
let window = window().expect("should have a window in this context");
let storage = window.local_storage().expect("should have local storage").unwrap();
…storage.set_item(...).unwrap();
```
`local_storage()` returns `Err`/`None` in private-browsing, sandboxed iframes, or when cookies are blocked. Any of these **panics the entire app at boot**, leaving the `index.html` spinner ("Human-Centric Internet Initiating…") stuck forever — the same symptom the user reported. Make the whole function best-effort with `let Ok(Some(storage)) = … else { return };`.

### C3 — `index.html` loading placeholder never clears on failure
The spinner + "Human-Centric Internet Initiating…" in `index.html` (lines 59–61) is *only* removed when WASM mounts into `#main`. Any panic before mount (C1, C2) leaves it visible permanently with no error shown. Add a JS timeout that surfaces a "failed to start" message + console hint after ~10 s so failures are diagnosable instead of an infinite spinner.

---

## 4. High-Priority Issues

### H1 — All 274 QApps hardcode a Catppuccin theme, ignoring the theme engine
Every `*_qapp.rs` hardcodes `background: #1e1e2e; color: #cdd6f4; …` (verified: 274/274 files contain `1e1e2e`). The shell ships a real theme system (`theme_engine.rs`, "Human Warmth" default, CSS variables `--qualia-*`). Result: open any discipline and the warm light shell suddenly frames a dark Catppuccin panel. Migrate QApps to the `--qualia-*` CSS variables (a scripted find/replace plus a shared style helper would do it across all 274).

### H2 — QApps are presentational, not wired to QualiaDB logic
`QAPP_IMPLEMENTATION_HANDOVER.md` states the mission: wire each discipline to QualiaDB capabilities (epistemic logic, Allen interval algebra, neuro-symbolic sieves). In reality, e.g. `anthropology_qapp.rs` collects dropdowns + notes and renders a static footer string ("QualiaDB → graph theory | Allen Interval | …"). No `invoke`, no NQuin emission, no graph write. This is the single biggest gap between the vision and the build. Define a real QApp↔engine contract (one Tauri command + one wasm stub) and pilot it on 2–3 disciplines before scaling.

### H3 — `legacy/` (~26 GB) still on disk
It is now correctly git-ignored (`git check-ignore legacy/` ✓) so it won't be committed, but it bloats the working tree, slows tooling, and was the source of an earlier near-disastrous 32 GB stage. Delete it (or archive outside the repo). Confirm nothing references it first.

### H4 — Stray `fix*.py` scripts in `webizen-studio/`
`fix.py`, `fix2.py`, `fix3.py`, `fix4.py` are untracked one-off RSX patch scripts. Either move them into a `tools/` dir with a note on what they fix, or delete them. Right now they're noise in `git status`.

### H5 — No tests in `webizen-studio`
Tests exist only in `webizen-runtime` (`clock.rs`, `kernel.rs`) and one in `webizen-desktop/main.rs`. The studio crate — 280+ source files — has zero. At minimum add: (a) a catalogue↔dispatcher↔mod consistency test (every `qapp_catalog()` id has a dispatcher arm and a registered module), which would have caught the `physics-sim`/`physics-simulator` mismatch automatically.

---

## 5. Medium-Priority Issues / Tech Debt

### M1 — Hardcoded localhost endpoints throughout `studio_canvas.rs`
`http://127.0.0.1:8080/manifest`, `ws://127.0.0.1:4242`, `http://127.0.0.1:8080/telemetry`, `ws://127.0.0.1:9001`. These silently fail in the web demo (no daemon) and are unconfigurable. Centralize into a config module with build-time/runtime overrides; gate the fetches behind an "is native / daemon present" check to avoid console error spam.

### M2 — `physics-sim` presentation-mode key was stale
`default_presentation_mode()` matched `"physics-sim"` while the catalogue/dispatcher use `"physics-simulator"`. **Fixed this session.** This is exactly the class of bug a consistency test (H5) prevents.

### M3 — `dateCreated` hardcoded in `save_qlink`
`commands/mod.rs` writes `"dateCreated": "2026-06-13T00:00:00Z"` literally instead of `now()`. Every saved QLink gets the same fake timestamp.

### M4 — Deprecated `type:` attribute in ~112 QApp files
Many components use `type: "text"` rather than the Dioxus-idiomatic `r#type: "text"`. It compiles today but is inconsistent with the platform components (which use `r#type`) and risks breakage on Dioxus upgrades.

### M5 — CI lacks a compile/lint gate
`pages.yml` only does a release `dx build`. A broken `clippy`/`cargo check` can still deploy if `dx build` tolerates warnings. Add `cargo clippy --target wasm32-unknown-unknown -- -D warnings` and a `cargo fmt --check` step. Also pin the `dtolnay/rust-toolchain@stable` to a dated toolchain for reproducibility.

### M6 — Catalogue/README count drift
README says "244+" QApps; the codebase now has **274** academic components + ~40 platform/scientific. Update copy to the real number and make it a single source of truth (e.g. derive from `qapp_catalog().len()`).

### M7 — `panic = "abort"` + no panic hook in wasm
With `panic = "abort"` and no `console_error_panic_hook`, any wasm panic gives an opaque `unreachable` with no stack. Add `console_error_panic_hook::set_once()` in `main()` for the web build — this alone would have made C2 instantly diagnosable.

---

## 6. Code Quality & Consistency (positives + polish)

**Genuinely good:**
- `webizen-runtime/src/wgpu_backend.rs` — clean ping-pong buffers, proper staging readback, `div_ceil` workgroup dispatch, state hashing. This is production-grade.
- `webizen-desktop/src/main.rs` — solid Tauri setup: port-conflict fallback loop, system tray, sysinfo telemetry loop, graceful daemon lifecycle.
- The dispatcher/catalogue/registry three-layer indirection is a sound pattern.

**Polish:**
- 274 QApps are near-identical boilerplate. Extract a `DisciplineWorkspace` higher-order component (props: title, field specs, accent) to replace ~250 lines each with ~30. Cuts the crate by an order of magnitude and makes theming (H1) a one-line change.
- Duplicated comment line in `main.rs` (`// ── Start daemon ──` twice).
- `submit_omnibox_query` uses string `.contains()` heuristics — fine for a prototype, but document it as such.

---

## 7. Build / CI / Repo Hygiene Checklist

- [ ] Delete or relocate `legacy/` (~26 GB) — **H3**
- [ ] Remove/relocate `webizen-studio/fix*.py` — **H4**
- [x] `.gitignore` is clean ASCII/CRLF and covers `target/`, `legacy/`, `dist/`, `**/.dx/` — verified
- [ ] Add `clippy` + `fmt --check` gate to `pages.yml` — **M5**
- [ ] Add `console_error_panic_hook` to wasm boot — **M7**
- [ ] Document the `qualiaDB` sibling-repo requirement in README for native builds — **§2**
- [ ] Pin toolchain + dioxus-cli versions for reproducible CI — **M5**
- [ ] Make QApp count derive from code, fix README "244+" → real number — **M6**

---

## 8. The "3D Engine" Question (answered)

> *"usually with 3d engines, like three.js etc, there's some sort of API or outline? what's needed? do we have it? if not, can we make it?"*

**What exists today:**
- `physics_simulator.rs` is a **CPU software 3D renderer** — hand-rolled `Vec3`, manual projection, painter's-algorithm depth sort — drawing onto a **Canvas 2D** context. No GPU, no shaders.
- `webizen-runtime` has **real GPU compute** (wgpu/WGSL diffusion) but it is **native-only** and headless (compute, not render), reachable only via Tauri.

**So: there is no Three.js-style 3D API in the browser path.** The two halves never meet — GPU is native+compute, browser is CPU+2D.

**What "having an API" would mean here — three options:**

| Option | Effort | Payoff |
|--------|--------|--------|
| **A. Three.js via JS interop** | Low–Med | Fastest path to real WebGL 3D in the demo; wrap a `<canvas>` and drive it with `wasm-bindgen`. Pragmatic. |
| **B. `wgpu` targeting WebGPU/WebGL** | Med–High | One renderer for *both* native and web — unifies the two worlds. The `webizen-runtime` already speaks wgpu, so a render pass could be added and compiled to `wasm32` with the WebGPU backend. Highest architectural payoff. |
| **C. Keep CPU rasterizer, formalize it** | Low | Define a small `Renderer` trait (`submit_mesh`, `set_camera`, `frame`) so the Canvas2D version and a future GPU version are swappable. Good intermediate step. |

**Recommendation:** Do **C now** (define the `Renderer` trait + scene/mesh/camera types so call sites are backend-agnostic), then **B** (wgpu→WebGPU) as the real engine — it reuses existing wgpu expertise and finally lets the browser demo benchmark actual GPU work. See Roadmap R1.

---

## 9. Enhancement Roadmap — "Make It Awesome"

### Phase 1 — Stabilize (1–2 days)
- **R0** Fix boot panics (C2) + add panic hook (M7) + spinner timeout (C3). *Demo never hangs silently again.*
- **R0.1** Delete `legacy/` + `fix*.py`; add clippy/fmt CI gate.
- **R0.2** Add the catalogue↔dispatcher↔module consistency test (H5/M2).

### Phase 2 — Make the demo honest & beautiful (3–5 days)
- **R1** Browser GPU/3D: define `Renderer` trait (Option C), then stand up a `wgpu`-WebGPU render path (Option B). Add a **browser-native** GPU benchmark so the demo's benchmark button actually does something without Tauri.
- **R2** Theme unification (H1): migrate all 274 QApps to `--qualia-*` variables via a shared `discipline_shell()` helper. Instantly coherent look.
- **R3** Extract `DisciplineWorkspace` HOC (§6) — shrinks the crate massively and makes R2 trivial.

### Phase 3 — Deliver the actual mission (1–3 weeks)
- **R4** Wire QApps to QualiaDB (H2): define the QApp↔engine contract (Tauri command + wasm stub returning mock data so the web demo still works). Pilot on Anthropology, History, Economics; then template the rest.
- **R5** Replace hardcoded localhost (M1) with a config layer + "native vs web" capability detection so the same UI degrades gracefully in-browser.

### Phase 4 — Distribution polish
- **R6** Native build reproducibility: vendor or submodule `qualiaDB`, or publish `qualia-core-db` to a registry, so the desktop app builds without manual sibling checkout.
- **R7** Real telemetry/observability path; signed-NQuin notifications surfaced in the UI (Notification Center is currently `Soon`).
- **R8** Per-discipline deep links (`/qapps/anthropology`) and search across the 274 so the catalogue is navigable beyond category filters.

---

## 10. Prioritized Action Table

| ID | Issue | Severity | Effort | Status |
|----|-------|----------|--------|--------|
| C1 | No browser GPU/3D path | Critical | High | Foundation laid (R1 `Renderer` trait) |
| C2 | `telemetry.rs` boot panics | Critical | Low | **Fixed (0.0.2)** |
| C3 | Spinner never clears on failure | Critical | Low | **Fixed (0.0.2)** |
| H1 | 274 QApps ignore theme engine | High | Med | **Fixed (0.0.2)** |
| H2 | QApps not wired to QualiaDB | High | High | **Contract + 3 pilots (0.0.2)** |
| H3 | 26 GB `legacy/` on disk | High | Low | Intentionally retained |
| H4 | Stray `fix*.py` | High | Trivial | **Fixed (0.0.2, archived to tools/)** |
| H5 | No studio tests | High | Med | **Fixed (0.0.2)** |
| M1 | Hardcoded localhost endpoints | Med | Med | **Fixed (0.0.2)** |
| M2 | `physics-sim` mode key stale | Med | Trivial | **Fixed** |
| M3 | Hardcoded `dateCreated` | Med | Trivial | **Fixed (0.0.2)** |
| M4 | Deprecated `type:` attr | Med | Low | **Fixed (0.0.2)** |
| M5 | CI lacks clippy/fmt gate | Med | Low | **Fixed (0.0.2)** |
| M6 | README QApp count drift | Med | Trivial | **Fixed (0.0.2)** |
| M7 | No wasm panic hook | Med | Trivial | **Fixed (0.0.2)** |

---

## 11. 0.0.2 Resolution Notes

The `0.0.2` branch addresses every item above except H3 (the `legacy/` folder is
intentionally retained as a record of prior work to be re-implemented properly).

- **C2** `telemetry.rs` rewritten as fully best-effort (no `unwrap`/`expect` on
  `window`/`local_storage`).
- **C3** `index.html` now shows a "failed to start" panel + console hint after a
  12 s boot timeout instead of an infinite spinner.
- **M7** `console_error_panic_hook` added to the wasm boot path.
- **H1** All 274 discipline QApps migrated off hardcoded Catppuccin hex to the
  shell's `--qualia-*` theme variables (`tools/migrate_qapps.py`).
- **M4** `type:` → `r#type:` across the same files (same script).
- **M1** Endpoints centralized in `src/endpoints.rs` with an `is_native_host()`
  capability gate; daemon traffic is skipped in the plain-browser demo.
- **R1 / C1** New backend-agnostic `src/render` module (`Renderer` trait, `Camera`,
  `Vec3`, `Canvas2dRenderer`); the physics simulator is refactored to be its first
  consumer, so a future WebGPU backend is a drop-in.
- **H2** New `qapp_engine` contract: one `analyze()` entry point resolving to the
  real `qapp_analyze` Tauri command (QualiaDB `q_hash` provenance) on desktop and a
  deterministic stub in the browser, with a reusable `EnginePanel`. Piloted on
  Anthropology, History, and Economics; scaling to the rest is mechanical.
- **H5** Added `cargo test` unit tests (catalogue↔dispatcher consistency, stub
  determinism) plus a `tools/check_qapp_consistency.py` CI gate.
- **M5/M6/H4** CI `check` job (consistency + `fmt --check` gate, informational
  `clippy`); README count corrected to 274; stray scripts archived under `tools/`.

*Original review reflects `clean-main`; the status column and this section reflect
the `0.0.2` branch.*
