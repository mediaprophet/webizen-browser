# Webizen — Swarm / Collaborative Processing

**Author:** Claude (Opus 4.8) · **Date:** 2026-06-15 · **Status:** Findings + surfacing plan. Builds on `WEBIZEN_NETWORK.md` (SocialWebNet), `WEBIZEN_REALMS.md` (Social realm).

> Goal raised: **difficult jobs done in collaboration with others who agree, via the network** — swarm processing. On audit, the engine machinery for this already exists in `qualia-core-db`; the gaps are the *consent/agreement wiring* and the *browser surface*.

---

## 1. What exists (the swarm machinery)

Two complementary swarm dimensions are implemented:

### 1a. Remote **peer** swarm — `daemon_swarm.rs`
`DaemonOrchestrator` coordinates **`WorkerCell`s** — 512 MB-bounded "Fractal Sharding" isolates — across a **WireGuard SocialWebNet** peer mesh (peers discovered via DNSSEC semantic payloads; `WEBIZEN_NETWORK.md` §2.5):
- `init_worker_cells_infrastructure()`, `spawn_fractal_shard(cell_id)` — provision compute cells.
- `delegate_dense_algebra(cell_id)`, `spawn_neuro_symbolic_isolates()` — assign heavy work.
- `WorkerCell::execute_tensor_contraction(...)`, `execute_quantum_chemistry(smiles)` — cells run real compute.
- `bootstrap_peer_connection()` / `bootstrap_social_wireguard()` / `get_active_peers()` — connect to other people's nodes.

### 1b. **Ambient device** swarm — `ambient_orchestration.rs`
`submit_task(Task)`, `discover_devices()`, `register_device(AmbientDevice)`, `execute_neural_inference(device, model, input)`, `execute_sub_threshold_computation(...)`, `get_performance_stats()` — distributes work across **nearby/edge devices** (your phone, a second machine), with a device registry and global metrics.

### 1c. Scheduling, distribution & consensus
- `local_scheduler.rs` — `Job` + `ComputeTarget`, progress/velocity tracking, `run()`.
- `specialized_libs/physics_simulation.rs` — `NodeDistribution`, `distribute_simulation()`, `run_distributed_simulation()` — split a simulation across nodes.
- `crdt.rs` — `apply_consensus_token(...)` + LWW CRDT + "suspended multi-party transactional consensus" (the audit's words) — the **"others who agree"** layer.
- `qpu_bridge.rs` — `submit_job(QPUJobSubmissionParams)` for quantum jobs; `orchestrator.rs` — thermal-aware local inference; `platform_scheduler.rs` — QoS thread binding.

**Net:** a difficult job (dense algebra, quantum chemistry, a physics simulation, neural inference) can be **sharded into WorkerCells and distributed across consenting peers' nodes over WireGuard**, or across **ambient local devices**, with CRDT consensus and provenance.

---

## 2. The thesis: consent-gated collaborative compute

This is the compute analogue of the network's social layer. The same primitives that gate *data* sharing gate *compute* sharing:

- **Agreement, not conscription.** A peer only runs your shard if they've **agreed** — an HCAI/cooperative agreement (`deontic_logic`) authorises the collaboration; `apply_consensus_token` records the multi-party consent. No node is silently used.
- **It runs over the trusted mesh.** Swarm peers are SocialWebNet/WireGuard peers — people in your Social realm with Front Door DID links, not anonymous strangers. (For anonymous compute you'd need a different trust model — out of scope.)
- **Provenance + attribution.** Each contributing cell/peer stamps its output (`provenance`); contributors are attributable — which ties directly to the **Cooperative obligation/attribution framework** (`port_requirements.md` §4.1/§4.8): swarm contribution *is* a non-monetary contribution that can be credited.
- **Bounded & safe.** Each `WorkerCell` is a 512 MB isolate (Fractal Sharding) — a peer bounds exactly what they lend; nothing escapes the cell.

---

## 3. Where it fits the product

- **A Social-realm capability** (`WEBIZEN_REALMS.md`): "do this hard job with my trusted peers." The swarm is the Social realm's compute fabric, just as VerifiedComms is its messaging.
- **Cooperative projects** (`port_requirements.md` §4): a co-op can pool compute; contributions feed obligation/attribution accounting.
- **Heavy QApps**: a discipline QApp facing a job too big for one device (large simulation, big inference) can offer "run across my swarm" — same `qapp_engine` contract, but the capability routes to the swarm.
- **The 3D/physics vision**: `distribute_simulation` lets a large `SemanticScene`/physics field be computed across nodes, then rendered locally.

---

## 4. Gaps to close (surface + wire)

The engine has the mechanism; missing are:

1. **Tauri command surface** — none exists yet for swarm (unlike Nym's `toggle_nym_relay`). Add: `swarm_status`, `swarm_submit_job(job, peers)`, `swarm_peers`, `swarm_contributions`. Mirror them through `qualia-client-core`.
2. **The agreement gate** — wire `apply_consensus_token` + an HCAI/cooperative agreement so a job is only distributed to peers who have signed up; surface the agreement in the UI before dispatch.
3. **Job model in the UI** — submit a `Job`/`Task`, watch `NodeDistribution` progress per peer, see provenance of each contribution, handle partial failure/peer drop.
4. **Attribution loop** — feed contributions into the cooperative obligation/attribution ledger so collaborators are credited.
5. **Consent symmetry** — a "lend my compute" toggle + budget (how many cells, when — e.g. only on mains power), so *offering* compute is as explicit and revocable as *requesting* it.

---

## 5. Browser surface (Dioxus)

- A **Swarm panel** (Social realm): your active peers, who's lending compute, jobs in flight with per-peer progress, contribution provenance.
- A **"Run on swarm"** affordance on heavy jobs (QApps, simulations) — shows which consenting peers will participate and the agreement before dispatch.
- A **"Lend compute"** control in Settings — cell count, power/network conditions, revoke-anytime (mirrors the network doc's "refusal is costless / consent revocable" invariants).

---

## 6. Open questions

1. **Trust model for swarm peers:** restricted to SocialWebNet/WireGuard (trusted) peers only, or also opt-in pools of semi-trusted nodes? (Recommend trusted-only first.)
2. **Failure/verification:** how is a peer's result verified (re-compute quorum, ZK proof via `zk_proofs`, or trust+provenance)? Matters for correctness of distributed science.
3. **Scheduling policy:** who decides shard placement — `DaemonOrchestrator` heuristics, thermal/QoS (`orchestrator`/`platform_scheduler`), or user hints?
4. **Attribution currency:** swarm contributions credited as cooperative obligation µ-units, care-credits, or just provenance? (Ties to `port_requirements.md` §4.)
5. **Ambient vs remote priority:** prefer local ambient devices before reaching to remote peers? (Recommend yes — cheaper, more private.)

*The swarm is consent-gated collaborative compute: bounded WorkerCells, distributed across trusted peers (WireGuard) or ambient devices, agreed via CRDT consensus + HCAI, with provenance and contributor attribution. The engine machinery exists; the work is the command surface, the agreement gate, and a dignified Dioxus UI for requesting and lending compute.*
