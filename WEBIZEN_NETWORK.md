# Webizen — Socially-Defined Network Architecture

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Architecture incorporated from `legacy/devnotes/` (QDP / Front Door / HCAI / Nym / WebRTC). Maps the SvelteKit-era design onto the Rust/Dioxus stack.
**Source notes:** `legacy/devnotes/notes.md`, `orchastration-webai.md` §3.12–§3.13, `orchastration-webai-implementation.md`, `port_requirements.md`.

> ⚠️ **WireGuard note up front:** WireGuard does **not** appear anywhere in the legacy notes. The "socially-defined network" the notes describe is **QDP DNS + Front Door DIDs + HCAI Agreement negotiation + Nym Mixnet + WebRTC (DTLS-SRTP) + WebTorrent**. WireGuard would be a *new* transport choice (a trusted-peer "social VPN" overlay). It is addressed as an explicit decision in §6 — incorporated as an option, not asserted as existing design.

---

## 1. The thesis: the local install **is** a new browser

The intended product is not "a wasm app with a daemon." It is a **Human-Centric browser** in which the QualiaDB Rust daemon is the local agent, baked in (legacy `notes.md`: *"the QualiaDB Rust Daemon is the local agent, seamlessly baked into the browser… your objective for 'IdP via Domain Name' is precisely the QDP we just built"*).

The inversion at the heart of it: **the user's browser becomes the globally discoverable hub; remote LLMs/platforms become inbound supplicants** that must negotiate access on the user's terms. The local graph is never exposed — only a minimal discovery record routing all inbound contact through a single chokepoint.

This is what Master-Plan profile **(B) the local install** delivers. Profiles (A) public wasm and (C) browser-projection are *views*; (B) is the browser.

---

## 2. The stack (five layers, all QualiaDB-native)

```
┌──────────────────────────────────────────────────────────────────────┐
│ 5. TRANSPORT   Nym Mixnet (anon) · WebRTC/DTLS-SRTP (sessions) ·       │
│                WebTorrent+DHT (content) · [WireGuard? §6]              │
├──────────────────────────────────────────────────────────────────────┤
│ 4. GATEKEEPER  HCAI Agreement Negotiation — the ONLY inbound door.     │
│                "Signature is not authorization": structural            │
│                minimisation gates data; the signature only binds a     │
│                liable Operator. Refusal is costless. Consent revocable.│
├──────────────────────────────────────────────────────────────────────┤
│ 3. FRONTDOOR   did.json @ /.well-known/did.json  +  _did TXT record.   │
│                One service endpoint: HCAIAgreementNegotiation.          │
│                Zero-telemetry. DNS-AID compatible.                     │
├──────────────────────────────────────────────────────────────────────┤
│ 2. IDENTITY    Front Door DIDs — a per-domain isolated DID so cross-   │
│                domain activity cannot be correlated.                   │
├──────────────────────────────────────────────────────────────────────┤
│ 1. DISCOVERY   QDP (Qualia Discovery Protocol) — 4-tier Q42 DNS        │
│                cascade: did:q42: → NS-encoding → HTTP QDP/.well-known  │
│                → DNS TXT. The DNS root is a free, DNSSEC-anchored       │
│                key-value directory.                                    │
└──────────────────────────────────────────────────────────────────────┘
```

### Layer details
- **Discovery (QDP/Q42 DNS):** four-tier resolution; NS-record encoding lets any compliant resolver resolve a `did:q42:` with no web server. A group/org DID resolves back to its organisational structure.
- **Identity (Front Door DIDs):** each domain gets a *purpose-built* DID "only for that domain… to ensure it doesn't lead to unintended consequences" (notes.md) — i.e. unlinkable per-context identity.
- **Frontdoor:** `did.json` exposes exactly one service (`HCAIAgreementNegotiation`); no API, no graph endpoint. For DNS-only registrars, the DID is encoded into `_did` TXT/NS records.
- **Gatekeeper (HCAI):** inbound agents receive a machine-enforceable signed policy (NQuin triples in `urn:webai:policy-graph`: `noDataRetention`, `noContextPersistence`, `sessionScopedOnly`, `dutyOfCareVersion`, `penaltyOnViolation`). The **Operator** (the liable human/company behind the agent), not the agent, is bound. Three invariants: structural minimisation is the only authorization; refusal is costless (browser fully functional with zero bound agents); consent is instantly revocable.
- **Transport:** Nym for anonymous/contextual comms (VerifiedComms chat bound to `q_hash(active_url)`); WebRTC DTLS-SRTP for live sessions; WebTorrent/DHT for ontology & content distribution.

---

## 3. QualiaDB / Rust module map (what backs each layer)

Everything is QualiaDB-native (`qualia-core-db`) or `qualia-client-core` — no external networking framework beyond the W3C/transport primitives.

| Layer | `qualia-core-db` / client modules | Already-live Tauri commands (in `webizen-desktop`) |
|---|---|---|
| Discovery | `resolver`, client `dns_resolver` (4-tier QDP), `webizen_identifiers` | `resolve_qdp_did`, `get_ns_records_for_did` ✅ |
| Identity | `identifier`, `webizen_identifiers`, `key_vault`, `fiduciary_crypto` | `generate_front_door`, `get_front_doors`, `generate_front_door_invite` ✅ |
| Frontdoor | `webizen_server`, `web_civics`, `provenance` | (did.json serving — to add) |
| Gatekeeper (HCAI) | `deontic_logic`, `modalities`, `agency`, `provenance` | `apply_semantic_handshake`, `accept_vault_handshake`, `evaluate_data_request` ✅ |
| Transport | `nym_adapter`, `p2p`, `webtorrent_seeder`/`webtorrent_routes`, `daemon_swarm`, `acoustic_ble_mesh` | `toggle_nym_relay`, `fetch_torrent_telemetry` ✅ |

**Key finding:** a large slice of the social-network layer is *already ported* to the Rust/Tauri stack — the omnibox already routes `qdp://`/`did:q42:` through `submit_omnibox_query` → `resolve_qdp_did`, and Front Door + Nym + semantic-handshake commands exist. The gaps are the **outbound `did.json` frontdoor server** and the **HCAI negotiation endpoint** as first-class surfaces, plus the Dioxus UI for all of it.

---

## 4. What "becomes a browser" requires (gaps to close)

On top of the Master Plan's local-install work (settings/about/tray/projection), delivering the *browser* means:

1. **Omnibox as protocol router** (partly done): `qdp://`, `did:q42:`, `webizen://`, `qualia://`, plus normal `https://`. The Dioxus omnibox calls `submit_omnibox_query` and renders the Front Door resolution result (it already does in the legacy notes).
2. **`webizen://` / `qualia://` scheme handlers** — the native Tauri URI-scheme protocols (the current `webizen-desktop/main.rs` already registers `qualia://` and `webizen://` schemes; extend with `submit_text`/`submit_claims` ingress + frontdoor routing).
3. **DNS Frontdoor server** — serve `did.json` at `/.well-known/did.json`; provision `_did` TXT/NS payloads via existing `ns_records_for_did`.
4. **HCAI negotiation endpoint** — the single inbound door; verify Operator-signed agreement, admit to a structurally-minimised, session-scoped channel; log to `urn:webai:agreement-log`; enforce via `deontic_logic`.
5. **VerifiedComms** — Nym-routed, per-URL contextual chat bound to `q_hash(active_url)` (Dioxus component over `nym_adapter`).
6. **Consent/Shield overlay** — always-one-action-away kill switch for any active session (the "consent is revocable" invariant).
7. **The browser pane** — for normal web browsing (a Tauri child webview), present in profile (B), absent from the public wasm (Master Plan §3).

---

## 5. Principles (non-negotiable, from the notes)

- **Zero-telemetry frontdoor:** `did.json` carries no personal data; resolving it reveals only that *someone* resolved a DID.
- **Signature ≠ authorization:** signing binds accountability, never expands access. Structure protects, not promises.
- **Refusal is costless:** zero bound agents = fully functional browser. Stricter Duty of Care never degrades local capability.
- **Consent is revocable:** instant session kill overrides TTL/task state.
- **Operator liability:** machines have no legal personhood; the human/corporate Operator behind a key is the bound, named, accountable party.

---

## 6. The WireGuard decision (new — needs the user)

WireGuard is **not** in the legacy design; the notes use Nym (anonymity) + WebRTC (sessions) + WebTorrent (content). Where it *could* fit and the trade-offs:

| Option | Role | Assessment |
|---|---|---|
| **A. No WireGuard (as-designed)** | Nym + WebRTC + WebTorrent cover anon, sessions, content | Matches the notes; nothing missing for the described threat model. Recommended unless a concrete need below applies. |
| **B. WireGuard as a "social VPN" overlay** | A persistent encrypted mesh between *already-trusted* peers (your own devices; contacts you've completed an HCAI agreement with) | Complements rather than replaces: Nym stays for anonymous/unknown inbound, WebRTC for calls, WireGuard for durable trusted links (e.g. multi-device personal-graph sync, a cooperative's member mesh). Peer config keyed by Front Door DIDs. |
| **C. WireGuard as primary transport** | Replace WebRTC/Nym | Not recommended — loses Nym's anonymity guarantees and WebRTC's browser-native call path; contradicts the threat model (trafficking/DV survivors need unlinkability, which a static-key VPN mesh weakens). |

**Architectural fit if B is chosen:** WireGuard is a *transport*, like WebRTC (which the design already uses), so it does not violate the "no external compute engine" rule. A Rust userspace implementation (`boringtun`) keeps it dependency-light and cross-platform. It would slot under Transport (layer 5), with tunnel peers provisioned from the Front Door DID directory and gated by the same HCAI agreement that authorises the relationship. **Open question for ratification:** does QualiaDB intend to own the overlay transport (so WireGuard becomes a `qualia-core-db` module alongside `nym_adapter`/`p2p`), or is it a `webizen-desktop` transport plugin?

---

## 7. Roadmap (folds into the Master Plan)

- **Phase 1 (with local-install work):** omnibox protocol router UI; surface the already-live QDP/Front Door/Nym commands in Dioxus; Shield/consent overlay.
- **Phase 2:** DNS Frontdoor server (`did.json` + `_did`) and the HCAI negotiation endpoint; VerifiedComms over Nym.
- **Phase 3:** the browser pane (Tauri child webview) + per-URL contextual binding; Front Door key-rotation policy UI.
- **Decision gate:** ratify §6 (WireGuard) before any tunnel work.

*All layers map to existing `qualia-core-db` / `qualia-client-core` modules; much of the discovery/identity/handshake surface is already ported. The remaining work is the frontdoor/HCAI server surfaces and the Dioxus UI that turns the local install into the browser the notes describe.*
