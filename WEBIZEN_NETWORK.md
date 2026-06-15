# Webizen — Socially-Defined Network Architecture

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Architecture incorporated from `legacy/devnotes/` (QDP / Front Door / HCAI / Nym / WebRTC). Maps the SvelteKit-era design onto the Rust/Dioxus stack.
**Source notes:** `legacy/devnotes/notes.md`, `orchastration-webai.md` §3.12–§3.13, `orchastration-webai-implementation.md`, `port_requirements.md`.

> ✅ **WireGuard correction (2026-06-15):** An earlier draft of this doc said WireGuard wasn't in the design — that was wrong. It is absent from the legacy *SvelteKit frontend* notes, but it is **already implemented in the engine**: `qualia-core-db/src/daemon_swarm.rs` defines a **`SocialWebNet`** — a WireGuard overlay where peer public keys are distributed via **DNSSEC CBOR-LD semantic payloads** tied to the Q42 DNS overlay and Front Door DIDs. WireGuard is therefore a *first-class existing transport*, not a new option. See §2.5 and §6.

---

## 1. The thesis: the local install **is** a new browser

The intended product is not "a wasm app with a daemon." It is a **Human-Centric browser** in which the QualiaDB Rust daemon is the local agent, baked in (legacy `notes.md`: *"the QualiaDB Rust Daemon is the local agent, seamlessly baked into the browser… your objective for 'IdP via Domain Name' is precisely the QDP we just built"*).

The inversion at the heart of it: **the user's browser becomes the globally discoverable hub; remote LLMs/platforms become inbound supplicants** that must negotiate access on the user's terms. The local graph is never exposed — only a minimal discovery record routing all inbound contact through a single chokepoint.

This is what Master-Plan profile **(B) the local install** delivers. Profiles (A) public wasm and (C) browser-projection are *views*; (B) is the browser.

---

## 2. The stack (five layers, all QualiaDB-native)

```
┌──────────────────────────────────────────────────────────────────────┐
│ 5. TRANSPORT   SocialWebNet/WireGuard (DID-keyed peer mesh, keys via   │
│                DNSSEC) · Nym Mixnet (anon) · WebRTC (P2P SuperBlock     │
│                streaming + sessions) · WebTorrent+DHT (content)        │
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
- **Transport:** **SocialWebNet/WireGuard** for the trusted peer mesh (§2.5); Nym for anonymous/contextual comms (VerifiedComms chat bound to `q_hash(active_url)`); WebRTC for P2P SuperBlock streaming and live sessions; WebTorrent/DHT for ontology & content distribution.

---

## 2.5 The DNS solution in detail (QDP + DNSSEC semantic payloads)

This is the "new form of DNS solution" — two complementary pieces, both grounded in q42.

**(a) QDP — the IETF Internet-Draft.** `draft-webcivics-qdp-protocol` (Timothy Holborn / WebCivics, Standards-Track) formally specifies:
- Discovery endpoint **`https://<domain>/.well-known/QDP`** returning RDF (Turtle + JSON-LD).
- An **agent-type ontology** (`QDP:` namespace): `PersonAgent`, `OrganizationAgent`, `AutomatedAgent` (AI), `EssentialService` (humanitarian), `ContentProvider` (with `schema:isAdultOriented` / `QDP:contentRating`), plus `QDP:hasService`, `sparqlEndpoint`, `hasSolidPod`, `hasEcashAccount`, etc.
- A **query API**: `GET /?domain=<d>&field=<prefix:prop>` and `GET /?domain=<d>&ecash`.
- **DNS verification via Front Door DIDs**: a `_qdp.<domain>` TXT record carries `qdp:signer <did:…>`, where the DID **MUST** be a per-domain Front Door DID (contextually isolated, anti-correlation).
- SHACL constraints, Security/Privacy considerations, IANA well-known + media-type registrations.

The client side is implemented in `qualia-client-core/dns_resolver.rs` as a **4-tier cascade** (header-documented): `did:q42:` → NS-record encoding → HTTP `/.well-known/QDP` → DNS TXT, with `did:web:`/`did:key:` passthrough.

**(b) DNSSEC CBOR-LD semantic payloads — the WireGuard key layer.** `qualia-core-db/daemon_swarm.rs` carries a **`DnssecSemanticPayload`** (CBOR-LD, ≤512 B for DNSSEC limits) in DNSSEC TXT (16) / CERT (37) records:
```
DnssecSemanticPayload { wireguard_pubkey:[u8;32], did_q42:u64,
                        routing_mask:u64 /*5th-vector HW mask*/,
                        semantic_handshake:String, peer_capabilities:u16,
                        semantic_context:u64 }
```
A `DnssecResolver` (trusted anchors + validation) fetches these; `SocialWebNetInterface::establish_wireguard_tunnel(peer_payload, endpoint, port)` then brings up a **WireGuard tunnel to a peer discovered purely via DNS**, keyed by the peer's Q42 DID. WorkerCells (512 MB "Fractal Sharding" isolates) each hold a DNSSEC resolver + WireGuard interface + Q42 lexicon + CBOR-LD parser.

**Net:** the DNS root becomes a DNSSEC-anchored, q42-encoded directory that distributes both *discovery* (QDP/Front Door DID) and *transport keys* (WireGuard pubkeys) — a DID-keyed social VPN mesh provisioned from DNS. This is the heart of the "socially-defined network."

---

## 3. QualiaDB / Rust module map (what backs each layer)

Everything is QualiaDB-native (`qualia-core-db`) or `qualia-client-core` — no external networking framework beyond the W3C/transport primitives.

| Layer | `qualia-core-db` / client modules | Already-live Tauri commands (in `webizen-desktop`) |
|---|---|---|
| Discovery | `resolver`, client `dns_resolver` (4-tier QDP), `webizen_identifiers` | `resolve_qdp_did`, `get_ns_records_for_did` ✅ |
| Identity | `identifier`, `webizen_identifiers`, `key_vault`, `fiduciary_crypto` | `generate_front_door`, `get_front_doors`, `generate_front_door_invite` ✅ |
| Frontdoor | `webizen_server`, `web_civics`, `provenance` | (did.json serving — to add) |
| Gatekeeper (HCAI) | `deontic_logic`, `modalities`, `agency`, `provenance` | `apply_semantic_handshake`, `accept_vault_handshake`, `evaluate_data_request` ✅ |
| Transport | **`daemon_swarm` (SocialWebNet/WireGuard + DnssecResolver)**, `nym_adapter`, `p2p`, `webtorrent_seeder`/`webtorrent_routes`, `acoustic_ble_mesh` | `toggle_nym_relay`, `fetch_torrent_telemetry` ✅ (WireGuard tunnel mgmt — not yet surfaced as a command) |

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

## 6. WireGuard / SocialWebNet — existing capability to surface

**Status: implemented in the engine, not yet exposed to the browser.** `qualia-core-db/daemon_swarm.rs` already provides `SocialWebNetInterface`, `SocialWebNetPeer`, `DnssecResolver`, `DnssecSemanticPayload`, `init_wireguard_interface()` and `establish_wireguard_tunnel()`. So this is not a "should we add WireGuard" question — it exists and is integrated with the DNS/DID layer (§2.5). The work is to **surface and operationalise** it:

1. **Tauri commands** to drive it from the Dioxus UI: `social_webnet_init`, `social_webnet_add_peer(did_q42)`, `social_webnet_status`, `social_webnet_tunnel_down`. (None exist yet — the WireGuard surface has no command, unlike Nym which has `toggle_nym_relay`.)
2. **Peer provisioning from the Front Door DID directory** — resolve a contact's `DnssecSemanticPayload` (their WireGuard pubkey + Q42 DID) via the `DnssecResolver` and call `establish_wireguard_tunnel`.
3. **HCAI gating** — a tunnel to a peer is authorised by the same HCAI agreement that defines the relationship; `routing_mask` enforces what the link may carry.
4. **Role clarity** (the layers are complementary, not competing): SocialWebNet/WireGuard = durable links between *trusted/known* peers (multi-device personal-graph sync, a cooperative's member mesh); Nym = anonymity for *unknown/inbound*; WebRTC = P2P SuperBlock streaming + calls; WebTorrent = content.
5. **Datapath verification** — confirm whether `establish_wireguard_tunnel` shells out to the OS WireGuard (`wg`/`wg-quick`, kernel/wintun) or expects a userspace impl; the desktop bundle must ship/locate the datapath on Windows/macOS/Linux. **This is the one genuine open implementation question** (not a design decision).

> Earlier this doc framed WireGuard as a new choice. Correction: it is an existing `qualia-core-db` transport (`daemon_swarm`); the only open item is the OS datapath integration in §6.5.

---

## 7. Roadmap (folds into the Master Plan)

- **Phase 1 (with local-install work):** omnibox protocol router UI; surface the already-live QDP/Front Door/Nym commands in Dioxus; Shield/consent overlay.
- **Phase 2:** DNS Frontdoor server (`did.json` + `_did`) and the HCAI negotiation endpoint; VerifiedComms over Nym.
- **Phase 3:** the browser pane (Tauri child webview) + per-URL contextual binding; Front Door key-rotation policy UI.
- **Decision gate:** ratify §6 (WireGuard) before any tunnel work.

*All layers map to existing `qualia-core-db` / `qualia-client-core` modules; much of the discovery/identity/handshake surface is already ported. The remaining work is the frontdoor/HCAI server surfaces and the Dioxus UI that turns the local install into the browser the notes describe.*
