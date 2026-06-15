# Webizen — Realms / Spheres (the engagement environment)

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Design proposal. Builds on `WEBIZEN_3D_NATIVE_VISION.md`, `WEBIZEN_NETWORK.md`, and the existing canvas (`webizen-studio/src/studio_canvas.rs` — `WebizenWorkspace` / `Page` / panes).

> The canvas should not be one flat workspace. People live across **realms** — personal, local, social, spatial, mind, real-world — and each has its own feel, tools, and boundaries. This doc proposes realms as the organizing metaphor for the engagement environment, grounded in QualiaDB's native **context vector**.

---

## 1. The core insight: a realm is a *context*, not a folder

A QualiaDB NQuin is a 5-vector — **subject, predicate, object, context, metadata** — and the **context** dimension is pervasive in the engine (present across ~71 `qualia-core-db` files). That context vector *is* the realm mechanism. A realm is therefore a **bounded semantic context over the one graph** — a `q42:context` binding — not a separate store or a tab.

This matters because it means realms are:
- **Native, not bolted on** — the engine already separates and queries by context.
- **Composable** — a "project" realm can span personal + social + spatial contexts.
- **A privacy boundary by construction** — what's in your personal context is not in your social context unless you move it there (with consent).
- **Lensable & time-travelable** — contextual lenses + `AS OF` temporal queries (grounded in `sparql_executor`) apply *within* a realm.

> Honest grounding: the **context vector** (well-grounded), **geospatial domain** (`domains/geospatial`, grounded), **AS-OF temporal** (grounded), and **epistemic modalities** (`epistemic`/`modalities`, present) are real. "Selfhood/Personhood **zones**" and "possible **worlds**" appear in the design/UX language (e.g. the Context Studio description) but are **not yet named engine primitives** — they are the realm layer to formalize *on top of* the context vector.

---

## 2. The realm taxonomy (your list, structured)

Each realm = a default context binding + the QualiaDB modules that serve it + a signature representation.

| Realm | What it holds | Context / engine grounding | Signature space (3D vision) |
|---|---|---|---|
| **Personal** (Selfhood) | Your identity, health vault, private notes, keys | private `q42:context`; `key_vault`, `wellfare-core`, `webizen_identifiers` | intimate near-space; warm; closed |
| **Local** | This device/node: hardware, daemon, offline-first state | local node context; `system_telemetry`, `daemon`, `storage_driver` | a HUD of your immediate surroundings |
| **Social** (Personhood) | Contacts, shared graphs, civic, conversations | shared contexts gated by HCAI; `social_connect`, `web_civics`, `daemon_swarm` (SocialWebNet), Front Door DIDs | a constellation of people (DID node-link) |
| **Spatial** | Real-world places, geography, routes | `domains/geospatial`, spatial sieve, GeoSPARQL | a map / globe |
| **Mind** (Epistemic) | Beliefs, hypotheses, reasoning, "possible worlds" | `epistemic`/`doxastic`, ASP stable models, `modalities` | an abstract branching world-space |
| **Real-world** (Embodied) | Sensors/IoT, anatomy, physical phenomena | `domains/physical`, `anatomy_context`, `dicom`, solvers | embodied scenes (body map, fields) |

Realms **nest and overlap**: a cooperative project is a realm that draws people from Social, places from Spatial, and obligations from Personal. The model is a *graph of contexts*, not a fixed list.

---

## 3. Representation — making realms intuitive (ties to the 3D vision)

Realms are where the native-3D medium earns its keep: instead of tabs, each realm is a **place you move into**, with a distinct, legible character.

- **Each realm has a signature scene** (`render::qualia::SemanticScene` built from that realm's context): Social = people constellation, Spatial = map, Mind = branching worlds, Real-world = body/field, Personal = intimate room, Local = device HUD.
- **A camera + theme + motion profile per realm** — entering Personal feels close and warm; Mind feels open and abstract. The canvas already has `PresentationMode` (Grid/Spatial), `CoordinateSpace`, and per-page `ThemeBinding` — extend `Page` with a **`realm` (context binding)** that selects all of these.
- **Realm-switching is navigation, not a tab swap** — a brief spatial transition (respecting reduced-motion → instant). The omnibox can switch realms ("go to my social realm"); a persistent "realm compass" shows where you are.
- **Boundaries are visible.** You can *see* you're in your private space vs a shared one — privacy becomes spatial and legible, which is the human-centric point.

---

## 4. Tools & doing work in a realm

A realm is not just a view — it scopes *the work*:

1. **Realm-scoped context.** Every pane/QApp in a realm issues queries, writes, and provenance **within that realm's context** by default (the `qapp_engine` call carries the realm context). Same QApp, different realm → different data, different scope.
2. **Realm-relevant tools.** QApps are **realm-tagged**: wallet/health → Personal; address book/chat → Social; the 274 disciplines → Mind/Knowledge; anatomy/sensors → Real-world; maps → Spatial; telemetry/hardware → Local. The realm surfaces its tools; the catalogue filters by realm.
3. **Lens + time within the realm.** A contextual lens narrows what you see; the `AS OF` scrubber moves the realm through time.
4. **Cross-realm work is explicit & consented.** Moving a fact from Personal → Social, or letting a Social peer see a Spatial location, **crosses a context boundary → triggers the Shield/HCAI consent flow** (`WEBIZEN_NETWORK.md` §4). Front Door DIDs give you a **different face per realm** (anti-correlation by design).

This unifies the whole product: realms are *where* tools live, *what* context they act in, and *which* consent boundary applies.

---

## 5. Canvas implementation (grounded, incremental)

The existing canvas already has most of the scaffolding:

- `WebizenWorkspace` → `Page { panes, layout_strategy, presentation_mode, coordinate_space, theme, … }`.
- **Add `Page.realm: RealmBinding`** where `RealmBinding` = `{ context: q42_context, kind: RealmKind, camera, theme, motion }`.
- `RealmKind` enum: `Personal | Local | Social | Spatial | Mind | RealWorld | Custom(context)`.
- The `qapp_engine` contract gains an optional **realm context** so engine calls scope to the active realm (a small extension of the existing `AnalysisRequest`).
- The 3D engine renders the realm's signature `SemanticScene`; realm switch = re-`build_scene` + camera transition.
- Realm tagging of QApps = one field in the catalogue (`cat` already exists; add `realm`), so the QApps view can group by realm.
- Crossing a realm boundary routes through the consent/Shield surface.

No engine changes are required to *start* — realms ride on the existing context vector and the canvas's existing page/presentation/theme machinery. The new surface is: the `realm` binding on a page, realm tagging on QApps, the realm compass/switcher UI, and per-realm scenes.

---

## 6. Why this is the right organizing idea

- **It matches how people actually think** — by life-area (me / here / us / place / mind / world), not by app.
- **It makes privacy spatial and intuitive** — you see and feel the boundary between private and shared; consent is a *place you cross*, not fine print. This is dignity-centred design.
- **It unifies everything we've planned** — QApps become realm tools, the network/Front-Door is the Social realm's fabric, the 3D engine renders each realm, the context vector is the substrate, HCAI/Shield is the boundary.
- **It's natively grounded** — realms are the context vector wearing a human face.

---

## 7. Roadmap (folds into the Master Plan)

- **Phase 1:** define `RealmBinding`/`RealmKind`; add `realm` to `Page` and a `realm` tag to the QApp catalogue; a simple **realm switcher** (start as themed views, not yet full 3D). Personal / Social / Mind first.
- **Phase 2:** per-realm `SemanticScene` + camera/theme/motion (Layer C of the 3D vision) — Social constellation and the Mind branching-world space.
- **Phase 3:** Spatial (map/geo) and Real-world (anatomy/sensors) realms; the realm compass; spatial realm-switch transitions.
- **Phase 4:** cross-realm consent flows wired to the Shield/HCAI; Front-Door-per-realm faces; nested/project realms.

---

## 8. Open questions

1. **Realm vs context cardinality:** is a realm exactly one context, or a *set/expression* of contexts (recommend: a realm is a context **query**, so it can union/scope)?
2. **Default realm + first-run:** which realm does the user land in? (Recommend Personal, with a visible compass.)
3. **Formalising zones:** should "Selfhood/Personhood zones" become real engine primitives over the context vector, or stay a UI grouping? (Affects whether `qualia-core-db` gains a `realm`/`zone` concept.)
4. **Mind realm semantics:** map "possible worlds" onto the existing `epistemic`/ASP machinery, or add an explicit worlds primitive?
5. **Motion budget per realm** (ties to the 3D vision §6.1): each realm likely wants its own calm-by-default profile.

*Realms turn the flat canvas into a set of natural places to think and work — personal, local, social, spatial, mind, real-world — each a context wearing a human, navigable, consent-aware face. The substrate already exists (the NQuin context vector); the work is the realm binding, per-realm scenes/tools, and consent-aware boundaries.*
