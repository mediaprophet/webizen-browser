# Webizen — Personal Apps: Authoring, Editability & Export

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Design + findings. Builds on `WEBIZEN_HYPERMEDIA_FORMAT.md`, `WEBIZEN_REORG_NOTES.md`.

> Two requirements raised: (1) **every QApp should be editable**, so people can make *personal versions*; (2) personal apps should **export** — as wasm PWAs for **mobile** install, and as **local apps on desktop** (wasm or native Rust). This doc grounds both in what exists.

---

## 1. Correction first: what `webizen-component-harvester` actually does

It was suggested the harvester handles the editability/personal-versions factors. **It doesn't.** On a full read, `webizen-component-harvester/src/main.rs` fetches Shoelace's `custom-elements.json` (CEM) from unpkg and **generates `generated_dioxus_components.rs`** — Rust/Dioxus wrappers around Shoelace custom elements. It is purely the **Shoelace → Dioxus binding generator** (a build-time tool feeding `webizen-studio/src/components/shoelace.rs`).

So **QApp editability needs its own home** — it is *not* the harvester. That home is the **QApp Studio + the `.q42app` hypermedia format** (below).

---

## 2. The editability rule (make it first-class)

**Rule: every QApp is editable and forkable.** A person can open any QApp, change it, and keep a *personal version* — without our permission and without a rebuild.

This is only possible because a QApp's essence is **data, not compiled code** (`WEBIZEN_HYPERMEDIA_FORMAT.md`): a `.q42app` is a declarative UI (panes + bindings) + capability bindings + shapes + signature. Therefore:

- **Edit** = open the `.q42app` in **QApp Studio** (the canvas already exists: `studio_canvas.rs` / `WebizenWorkspace`), rearrange panes, change bindings/queries, save.
- **Fork** = copy the `.q42app`, modify, **re-sign with your own DID** (`fiduciary_crypto`/`key_vault`). The fork is content-addressed by its new `q_hash`; provenance links it to the original (attribution preserved).
- **The 274 built-in disciplines** should be *exportable to `.q42app`* (scriptable from the catalogue) so they too become editable templates, not frozen binaries.
- **Personal vs published:** a personal fork stays in your Personal realm; publishing it (WebTorrent, `webtorrent_seeder`) is a separate, consented act.

> Implication: the built-in `qapp_dispatcher` (hardcoded components) is the *seed library*; the durable model is **editable `.q42app` data** loaded at runtime. Migrating the disciplines to `.q42app` (Hypermedia §6 Phase C) is what unlocks editability at scale.

---

## 3. Export target A — Mobile (wasm PWA): `qualia-mobile-harness`

The mobile story already has a template crate. `qualia-mobile-harness` is a **Dioxus (web/wasm) PWA** with real mobile-edge support:

- **Build features per platform:** `android_pwa_edge` and `ios_pwa_edge` — each pulls in `qualia-core-db` (optional dep), `opfs_storage`, `wal_persistence`, `crdt_dvv_eae` (+ `wasm_simd` on Android).
- **Persistence:** **OPFS** (`FileSystemSyncAccessHandle` — must run in a Web Worker; `assets/sync_io_worker.js` is that worker) + WAL. So a personal app stores its graph locally on the phone.
- **Footprint:** the notes say use **`profile_minimal_512`** on `qualia-core-db` to stay under a 512 MB RAM budget — i.e. the *engine itself* can ship inside the mobile wasm bundle, minimised.
- **Optional engine:** `qualia-core-db` is an *optional* dependency — a light personal app can ship without the full engine and talk to a daemon, or embed the minimal engine for offline.

**So the export path is:** a personal app (`.q42app` + a Dioxus shell) → built with `android_pwa_edge`/`ios_pwa_edge` → a **wasm PWA installable on the phone**, offline-capable via OPFS. `qualia-mobile-harness` is the reference shell for this; productising "export my app to mobile" means templating a build from it.

> ⚠️ Carry-over from the reorg notes: the harness currently uses inline `html5-qrcode` **JS** for the camera — replace with `web-sys` `BarcodeDetector`/`getUserMedia` to honour the no-JS rule.

---

## 4. Export target B — Desktop local apps (wasm **or** native Rust)

Desktop personal apps have two viable shapes; support both:

| Shape | How | When |
|---|---|---|
| **wasm-in-webview** | The `.q42app` runs in the desktop Tauri webview (the same path the studio uses), served by `qapps_protocol` (loopback asset server from `{storage}/Qapps/`). | Default — instant, no compile, fully sandboxed, editable. |
| **Native Rust** | The app compiles as a Dioxus-desktop / Tauri binary (native window), linking `qualia-client-core`. | When the app needs native APIs/perf beyond the webview, or a standalone installer. |

The **wasm-in-webview** path is the default for personal apps (zero-build, editable, sandboxed by `extension_bus` permissions). The **native** path is an *export/package* step (like the mobile build) for apps that graduate to standalone distribution.

---

## 5. The unifying model: one app, three runtimes

A personal app is **one `.q42app`** (declarative UI + capability bindings + signature) that can be:

```
                       .q42app  (editable, signed, content-addressed)
                          │
        ┌─────────────────┼─────────────────────────────┐
        ▼                 ▼                             ▼
   RUN in the         EXPORT to mobile             PACKAGE for desktop
   Webizen canvas     (wasm PWA: android_/         (wasm-in-webview default,
   (QApp Studio,      ios_pwa_edge, OPFS,           or native Rust binary)
   instantly          minimal-512 engine)
   editable)
```

- **Author/edit:** QApp Studio (canvas) → save `.q42app`.
- **Run:** in the browser immediately.
- **Fork:** copy + re-sign with your DID → personal version.
- **Export:** mobile wasm PWA (`qualia-mobile-harness` template) or desktop (webview/native).
- **Publish (optional, consented):** seed the `.q42app` to WebTorrent.

This makes "make your own app, run it, keep a personal version, put it on your phone" a coherent, native, no-Node pipeline.

---

## 6. What exists vs. what to build

**Exists:** QApp Studio canvas, `qapp_manifest`/`qapp_registry`/`qapps_protocol`, the mobile PWA harness (with platform features + OPFS), `key_vault`/`fiduciary_crypto` for signing, `webtorrent_seeder` for publishing, `extension_bus` for sandboxing.

**To build:** the `.q42app` reader/writer (Hypermedia format), exporting the 274 disciplines to `.q42app`, the "fork + re-sign" flow in QApp Studio, the "export to mobile/desktop" build templating, and the no-JS camera fix in the harness.

---

## 7. Open questions

1. **Editing model:** purely declarative (panes + bindings) for everyone, with `webizen_bytecode` only for advanced logic? (Recommend yes — keeps most apps safely editable by non-programmers.)
2. **Native desktop export:** Tauri-per-app (heavier) vs. a shared host that loads native plugins? (Recommend shared host first.)
3. **Mobile productisation:** is "export to mobile" a one-click build in the browser, or a CLI step initially? (Recommend CLI/template first.)
4. **Discipline migration:** export all 274 to `.q42app` now (unlocks editability) or after the format is ratified? (Recommend after ratification, then scripted.)
