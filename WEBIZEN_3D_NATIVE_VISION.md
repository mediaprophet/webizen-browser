# Webizen — A 3D-Native, Human-Centric Web (Vision)

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Vision + grounded technical path. Builds on `WEBIZEN_3D_ENGINE.md`, `WEBIZEN_FLUTTER_REVIEW.md`, `WEBIZEN_NETWORK.md`.

> The prior implementations (SvelteKit, Flutter) treated 3D as a single bolt-on screen — and the Flutter one did it via a **WebView running JavaScript**. We now have a **native Rust render engine** (`webizen-studio/src/render`). The opportunity: make 3D and motion a *broad, native medium* across the whole product — not a gimmick screen — while staying human-centric (provenance-first, consent-first, dignified, accessible, performant).

---

## 1. Principles (these constrain every flourish)

1. **Provenance-first, always.** Motion and depth should *reveal* where knowledge comes from (NQuin sources), never decorate over it. The Flutter `super_quin_provenance_chip` / `citation_chips` become living, inspectable spatial objects.
2. **Consent & dignity over spectacle.** The Shield/HCAI surface (`shield_alert`, `vault_hud_bar`, `guardian_affirmation_chip`) uses calm, legible motion — never anxiety-inducing flashing. For survivor/high-risk users (WellFare threat model), motion must never draw attention in a shoulder-surf context.
3. **Accessible by default.** Respect `prefers-reduced-motion`: every animation has a static equivalent; nothing essential is conveyed by motion alone. Keyboard-navigable; no motion-locked affordances.
4. **Performance-respectful.** Canvas2D backend runs everywhere today; `WgpuRenderer` is the upgrade. Animations idle when off-screen, throttle on battery, and never block the main thread (the engine already submits world-space geometry and projects per-frame).
5. **Semantic, not random.** Motion is driven by *meaning from QualiaDB* (a node pulses because its data is live / an inference fired), not arbitrary loops. This is the `render::qualia::SemanticScene` binding's whole point.

---

## 2. The layered strategy (fine motion → full spatial scenes)

Native 3D/motion is woven at four scales, each shipping independently:

### Layer A — **Animated icons & micro-interactions** (broad, immediate, what you asked for)
Every QApp icon, nav glyph, and status chip becomes *alive but quiet*:
- **Idle:** a subtle breathing/parallax (≤2–3% scale, slow) so the surface feels responsive, not static.
- **Hover/focus:** spring-eased lift + depth shadow (the engine's `Camera`/`Transform` make a tiny per-icon scene trivial; or SVG transform for the cheapest path).
- **Semantic state:** an icon reflects its QApp's live state — a gentle pulse when there's new data, a provenance "spark" when a claim is committed (`qapp_engine` already returns a provenance hash → fire a one-shot motion), a desaturate when offline.
- **How (cheap → rich):** start with CSS/SVG transforms driven by a shared Rust **animation timeline** (an `requestAnimationFrame` loop in `web-sys`, reduced-motion-aware), then promote hero icons to real `Mesh` scenes rendered by `webizen-render` when `WgpuRenderer` lands. The 274 discipline icons get a single shared motion profile, themed by `--qualia-accent`.

### Layer B — **Component micro-3D**
Cards, the dashboard sparklines, the QApp grid get depth: tilt-on-hover, layered parallax, depth-sorted shadows. The existing dashboard SVGs gain a z-axis. Provenance chips become small 3D tokens you can flip to read the NQuin.

### Layer C — **Spatial views of the graph** (the differentiator)
The things that are *natively graph-shaped* render as navigable 3D structures via `webizen-render` + `SemanticScene`:
- **The knowledge graph** — NQuin neighbourhoods as an orbitable node-link field (SPARQL/RDF-star → `SemanticScene` → `build_scene`).
- **The chat-graph DAG** (`qualia-client-core/chat_graph`) — the branching conversation as a literal 3D tree you fly through; fragments and reply edges are nodes/edges.
- **The provenance/temporal chain** — derivation history (`provenance`, `temporal_graph`) as a depth-layered timeline; "drill into evidence" = move through z.
- **QDP discovery** — resolving a Front Door DID animates a route through the QDP cascade (a small spatial map of who-vouches-for-whom).

### Layer D — **Full native scenes**
- **Anatomy** — `anatomy_context` → organs colored by inferred intensity (already wired in `render::qualia`), the flagship replacing the Flutter WebView/JS.
- **Physics/science** — the diffusion field and solver outputs (`webizen-runtime`, `domains::physical`, `quantum_dft`) rendered live, sharing the wgpu device.

---

## 3. Animated icons — the concrete dev-kit need

This is broad surface (every icon), so it must be *systematic*, not per-icon hand-coding. Proposed addition to `webizen-render`:

```
render::motion (new)
  ├─ Timeline        // global rAF clock, dt, prefers-reduced-motion gate
  ├─ Tween/Spring    // ease fns + critically-damped spring (pos/scale/rgba)
  ├─ Anim<T>         // a value animating over time (driven by Timeline)
  └─ IconScene       // a tiny reusable Scene: 1 mesh + semantic state -> motion
```
- One `Timeline` per app; components subscribe. Reduced-motion → tweens snap to final value.
- A `MotionProfile` (idle/hover/active/semantic) is themed and shared across all 274 icons, so the look is coherent and one change re-themes everything.
- Semantic hooks: `qapp_engine` events (provenance committed, data live, offline) drive `Anim` targets — motion *means* something.
- Cheap path first (SVG/CSS transform via the timeline) so it ships before `WgpuRenderer`; same `MotionProfile` later drives real meshes.

---

## 4. Why this is *remarkable* (and honest about scope)

- **No one ships a browser whose UI is driven by a semantic graph engine.** Motion that encodes provenance and consent — not engagement-bait — is a genuinely new, dignity-centred aesthetic.
- It is **native and JS-free**: pure Rust/Dioxus + `web-sys`, replacing the Flutter WebView/JS 3D and any three.js/Babylon path.
- It **reuses what exists**: `webizen-render` (built), `SemanticScene` (built), QualiaDB graph/provenance/geometry (present). The new surface is the `motion` module + per-feature scenes.
- **Restraint is the point.** For the WellFare threat model, the most human-centric choice is often *less* motion. The system must make calm, accessible, reduced-motion the easy default.

---

## 5. Roadmap (folds into the Master Plan)

- **Phase 1 (with Phase-0/1 work):** `render::motion` (Timeline + Spring + reduced-motion gate); apply a shared `MotionProfile` to nav + the 274 QApp icons (Layer A). Cheap SVG/CSS path. *Immediate, broad, visible.*
- **Phase 2:** Layer B (card depth, provenance tokens); `WgpuRenderer` lands → hero icons become real meshes.
- **Phase 3:** Layer C spatial views — start with the **chat-graph DAG** (data exists) and the **knowledge-graph** browser; then provenance/temporal and QDP discovery.
- **Phase 4:** Layer D full scenes — anatomy (flagship), physics/science live.

**Accessibility/perf gates apply at every phase:** `prefers-reduced-motion` honoured, off-screen idle, battery throttle, static equivalents — verified before each ships.

---

## 6. Open questions

1. **Motion budget / brand:** how lively by default? Recommend *quiet* (small, slow, semantic) with an intensity setting in Settings (Calm / Standard / Off).
2. **Icon pipeline:** keep Shoelace/Bootstrap-icon glyphs (animate via transform) or author native mesh icons for hero items? (Ties to the Shoelace decision + `webizen-component-harvester`.)
3. **Where `render::motion` lives:** in `webizen-render` (recommended — motion is presentation) so both wasm and native share it.
4. **Reduced-motion as default for high-risk profiles:** should Sanctuary/WellFare mode force Calm? Recommend yes.

*The engine is built; the medium is native; the graph is semantic. The remaining work is a small motion system + per-feature scenes, applied with human-centric restraint — a web that moves to show you the truth of where things come from, not to capture your attention.*
