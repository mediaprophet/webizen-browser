# Webizen Design Notes & Requirements

*Generated: 2026-06-16*
*Purpose: Consolidated requirements and structured approach from all design documents*

## Document Overview

This document consolidates requirements, insights, and design principles from across the Webizen project design documentation to provide a structured approach to implementation.

---

## Table of Contents
1. [Core Design Philosophy](#core-design-philosophy)
2. [3D Engine Requirements](#3d-engine-requirements)
3. [Native Vision & 3D Integration](#native-vision--3d-integration)
4. [Application Lifecycle](#application-lifecycle)
5. [Consciousness & Interaction Model](#consciousness--interaction-model)
6. [Hypermedia Format](#hypermedia-format)
7. [Network Architecture](#network-architecture)
8. [QualiaDB Integration](#qualiadb-integration)
9. [Realms & Context Management](#realms--context-management)
10. [Technical Organization](#technical-organization)
11. [Implementation Priorities](#implementation-priorities)

---

## Core Design Philosophy

*Source: WEBIZEN_DESIGN_PHILOSOPHY.md, WEBIZEN_CONSCIOUSNESS_INSIGHTS.md*

### Seven Core Principles

#### P1 — Attention is Sacred
- **Forbids**: Engagement-bait, infinite feeds, dark patterns, anxiety-inducing motion
- **Requires**: Motion/notifications serve comprehension, never capture attention
- **Implementation**: Calm-by-default UI, reduced-motion support, no interruption during deep focus

#### P2 — Human is Participant-Author, Not User
- **Forbids**: Read-only apps, locked content, passive consumption
- **Requires**: Every QApp editable/forkable, omnibox as primary interaction, AS-OF queries for temporal context
- **Implementation**: QApp Studio, fork + re-sign workflow, personal vs published distinction

#### P3 — Truth Has Provenance
- **Forbids**: Unsourced assertions, opaque "magic," AI output without citation
- **Requires**: Every claim shows where it came from, NQuin provenance everywhere
- **Implementation**: Citation chips, qapp_engine returns provenance hash, provenance-stamped interactions

#### P4 — Externalise and Spatialise Mind
- **Requires**: Use space as thinking medium (method of loci / memory palace)
- **Implementation**: 3D spatial graph views, Mind realm, chat-graph DAG as navigable tree

#### P5 — Stewardship and Commons
- **Requires**: Net as shared craft we crew together
- **Implementation**: Social realm, cooperative features, shared resources

#### P6 — Ontological Responsibility
- **Requires**: Every default is an ethical choice, design recursively shapes designers
- **Implementation**: Careful consideration of all defaults, ethical design review process

#### P7 — Language/Semantics as Expansion
- **Requires**: Language is generative; better language creates better realities
- **Implementation**: Ontology Workbench as headline feature, author vocabularies, semantic tools

### Philosophical Foundations

#### Participatory Universe (Stapp, Penrose, Hameroff)
- Observer's choice of question is causally efficacious
- Consciousness involves non-computable processes (Orch-OR)
- Attention can hold quantum states (Quantum Zeno Effect)
- **Architecture**: Omnibox as "Process 1" - user's free act of inquiry

#### Ontological Design (Silva, Willis)
- "Everything we design, in turn, designs us back"
- Language is generative; better language creates better realities
- Consciousness is an engineering project

#### Consciousness as Not-Computation (Penrose, Searle, Feynman, Faggin)
- Understanding is non-computable (Goedel incompleteness)
- Syntax is not semantics (Chinese Room)
- Qualia are irreducible first-person experience
- **Architecture**: LLM framed as tool, never as mind (HCAI: "signature is not authorization")

#### Divided Brain (McGilchrist)
- Right hemisphere: broad, relational, contextual attention
- Left hemisphere: narrow, focused, analytical
- Right should be "master" providing context for left's analysis
- **Architecture**: 3D spatial views (Layer C) + QApp panels for analytical work

---

## 3D Engine Requirements

*Source: WEBIZEN_3D_ENGINE.md, Current Codebase Analysis*

### Main Purpose
- Replace Babylon.js/three.js with internal Rust 3D engine
- Provide developer kit for 3D rendering
- Design engine to use QualiaDB (geometric algebra, physics, diffusion, query-driven scenes)

### Technical Requirements
- **Backend-agnostic, immediate-mode engine**: Pure Rust, compiles to wasm
- **Renderer trait**: Swappable backends (Canvas2D reference, WebGPU next)
- **Scene graph**: Nested transforms (no matrix composition needed)
- **Semantic layer**: Binding to QualiaDB query/inference output via `SemanticScene`
- **No external 3D crates**: Pure Rust implementation

### Current Implementation Status ✅ PARTIALLY COMPLETE

**Implemented Components:**
- ✅ `webizen-render/src/scene_contract.rs` - Full 10D tensor support with spectral rendering
- ✅ `webizen-render/src/scene.rs` - Basic scene structures (SceneNode, SceneEdge, SceneCamera)
- ✅ `webizen-render/src/wgpu_renderer.rs` - WGPU-based renderer with camera projection
- ✅ `webizen-render/src/math/` - PGA (Projective Geometric Algebra) math module
  - `motor_encoder.rs` - Motor encoding for transformations
  - `buffer_alignment.rs` - Memory alignment utilities
- ✅ `webizen-render/src/pipeline/` - WebGPU pipeline infrastructure
  - `bind_groups.rs` - Bind group management for GPU resources
- ✅ `webizen-render/src/shaders/` - Shader module infrastructure
- ✅ `webizen-render/src/audio_contract.rs` - Audio data contract for 10D integration

**Pending Components:**
- ❌ glTF/atlas loading as layout provider
- ❌ Orbit/arc-ball camera controls + interaction (pick/hover → SceneItem)
- ❌ Advanced lighting/shading model (flat → Lambert → PBR)
- ❌ `SceneSource` over SPARQL (query → SemanticScene via qapp_engine)
- ❌ Browser-based engine benchmark
- ❌ GPU-resident QualiaDB math operations

**Design Principles - IMPLEMENTED:**
- ✅ Scenes are projections of QualiaDB query/inference output with provenance
- ✅ 10D tensor integration with spectral rendering (CIE XYZ projection)
- ✅ GPU compute and rendering share the wgpu device
- ✅ Zero-heap considerations in tensor operations

### Implementation Recommendations (Updated Priority)
1. **HIGH PRIORITY**: Implement glTF/atlas loading for biomedical assets (anatomy project)
2. **HIGH PRIORITY**: Add orbit/arc-ball camera controls + interaction (pick/hover → SceneItem)
3. **MEDIUM PRIORITY**: Create `SceneSource` over SPARQL (query → SemanticScene via qapp_engine)
4. **MEDIUM PRIORITY**: Add advanced lighting/shading model (flat → Lambert → PBR)
5. **LOW PRIORITY**: Create browser-based engine benchmark
6. **LOW PRIORITY**: GPU-resident QualiaDB math operations

### Technologies
- Rust (pure), wasm, WebGPU 0.19, geometric algebra (SIMD), Projective Geometric Algebra (PGA)

---

## 10D Tensor Backplane Implementation

*Source: 10D_INTEGRATION_SUMMARY.md, 10D_INTEGRATION_PLAN.md, Current Codebase Analysis*

### Main Purpose
- Transform browser from 2D/3D graph viewer to 10D viewport with spectral rendering
- Implement Q42 volumetric tensor specification [q,v,w,x,y,z,t,α,μ,σ]
- Enable quantum context management with wavefunction collapse
- Support zero-heap binary IPC for high-throughput biomedical data

### Implementation Status ✅ HIGH/MEDIUM PRIORITY PHASES COMPLETE

**Completed Phases:**

#### Phase 1: Data Contract Evolution ✅
- ✅ `Tensor10DProjection` struct [q,v,w,x,y,z,t,α,μ,σ] with Default impl
- ✅ `EpistemicState` enum (Collapsed, Pending, Sandbox)
- ✅ `SceneNode` extended with tensor, epistemic_state, version fields
- ✅ `RenderScene` extended with temporal_slice, epistemic_filter fields
- ✅ Spectral mapping functions (σ→color, α→opacity, μ→noise indicator)
- ✅ CIE XYZ projection with stack-allocated matrices
- ✅ Updated scene_to_contract.rs to populate 10D fields with defaults

#### Phase 2: Quantum UI ✅
- ✅ `collapse_wavefunction` Tauri command
- ✅ `set_temporal_slice` Tauri command with zero-heap AtomicU64 bit-casting
- ✅ `TemporalSlice` state using AtomicU64 (bit-cast to f64, avoids Mutex<f64> heap)
- ✅ `EpistemicStatus` UI component (`webizen-studio/src/components/epistemic_status.rsx`)
- ✅ `TemporalScrubber` UI component (`webizen-studio/src/components/temporal_scrubber.rsx`)

#### Phase 3: Spectral Rendering ✅
- ✅ Full CIE XYZ projection implementation with stack-allocated matrices
- ✅ CIE 1931 2-degree color matching functions (simplified approximation)
- ✅ XYZ to sRGB transformation matrix (stack-allocated)
- ✅ WgpuRenderer updated to use spectral colors from tensor.sigma
- ✅ Fallback to node.color when tensor.sigma == 0.0

#### Phase 4: Hardware Capability Detection ✅
- ✅ `HardwareTier` enum (Tier0-3)
- ✅ `BrowserCapabilities` struct
- ✅ `HardwareCapabilities` UI component (`webizen-studio/src/components/hardware_capabilities.rsx`)
- ✅ `register_browser_capabilities` Tauri command
- ✅ Stack-allocated tier determination logic

#### Phase 5: Binary IPC ✅
- ✅ `TensorBufferView` with binary index table support (`webizen-studio/src/render/tensor_buffer.rs`)
- ✅ `BinaryNodeRegistry` for string ID → u64 index mapping (`webizen-desktop/src/commands/binary_registry.rs`)
- ✅ `collapse_wavefunction` updated to accept u64 index (binary IPC)
- ✅ `collapse_wavefunction_legacy` for backward compatibility
- ✅ Binary index table builder (O(1) node lookup)
- ✅ Tauri managed state for BinaryNodeRegistry

**Pending Phases:**

#### Phase 6: Audio Synthesis ❌ (LOW PRIORITY)
- ❌ AudioWorklet-based spectral synthesis pipeline
- ❌ SharedArrayBuffer zero-copy transport
- ❌ Audio-visual synchronization component
- **Status**: Backend scaffold complete, implementation paused (see AUDIO_PROJECT_STATUS.md)

### Zero-Heap Compliance Report

**Fully Zero-Heap Compliant Components:**
- ✅ Tensor10DProjection - Copy type, stack-allocated
- ✅ EpistemicState - Copy enum, stack-allocated
- ✅ TemporalSlice - AtomicU64 with bit-casting, stack-allocated
- ✅ TensorBufferView - Copy type, zero-copy buffer access
- ✅ Tensor10DView - Copy type, stack-allocated
- ✅ CIE XYZ projection - Stack-allocated matrices, stack-only math
- ✅ Hardware tier determination - Stack-allocated comparisons

**Unavoidable Heap Usage (Documented Assumption Violations):**
- ⚠️ Frontend UI components - Dioxus/React state management is inherently heap-based
- ⚠️ String types - Rust String is heap-allocated (unavoidable for user-facing data)
- ⚠️ Tauri IPC - Cross-process serialization requires heap allocation
- ⚠️ Browser APIs - WebGPU, AudioContext return heap-allocated objects (uncontrollable)
- ⚠️ RenderScene Vec<T> - Dynamic sizing requires heap (data contract requirement)
- ⚠️ ActiveAnchor - String heap allocation for node IDs (unavoidable for identification)

**Zero-Heap Mitigation Strategies Applied:**
- ✅ Used AtomicU64 bit-casting instead of Mutex<f64> for TemporalSlice
- ✅ Used stack-allocated arrays for CIE XYZ matrices
- ✅ Used Copy types for all tensor views and projections
- ✅ Implemented zero-copy buffer access via TensorBufferView
- ✅ Used stack-only arithmetic in spectral projection functions
- ✅ Binary IPC uses u64 indices instead of String IDs

### Architecture Alignment

**Stateful Orchestrator / Stateless Viewer Pattern:**
- ✅ Backend (Tauri): Manages state (ActiveAnchor, TemporalSlice)
- ✅ Frontend (Dioxus): Stateless viewport display
- ✅ 10D tensor computation: Backend responsibility
- ✅ Spectral projection: GPU rendering (webizen-render)

### Build Verification
- ✅ Phase 1: webizen-render (2m 55s)
- ✅ Phase 2: webizen-desktop (7.39s)
- ✅ Phase 3: webizen-desktop (1m 27s)
- ✅ Phase 4: webizen-desktop (27.96s)
- ✅ Phase 5: webizen-desktop (27.69s)
- **Warnings**: Only unused imports and dead code (no blocking errors)

### Related Projects

**Anatomy Project Status** (ANATOMY_PROJECT_STATUS.md):
- ✅ GLB Ingestion System for CCF biomedical assets
- ✅ Semantic Extraction (FMA, SNOMED-CT ontology parsing)
- ✅ 10D Tensor Mapping for spatial/topological dimensions
- ✅ Binary Node Registry integration
- ⚠️ Frontend rendering issue (Dioxus blank screen) - blocking stress test execution
- ⚠️ 18MB vasculature stress test ready but awaiting UI fix

**Audio Project Status** (AUDIO_PROJECT_STATUS.md):
- ✅ AudioSpectralSheet data contract designed
- ✅ Dual-mode audio processing architecture (temporal/spectral)
- ❌ Implementation paused (awaiting anatomy project validation)
- ❌ Audio file ingestion, spectral analysis, WebGPU visualization pending

---

## Native Vision & 3D Integration

*Source: WEBIZEN_3D_NATIVE_VISION.md*

### Main Purpose
- Make 3D and motion a broad, native medium across the whole product (not bolt-on)
- Stay human-centric: provenance-first, consent-first, dignified, accessible, performant

### Technical Requirements
- **Native Rust render engine**: No JavaScript 3D
- **Layered motion strategy**: 4 scales (icons → components → spatial views → full scenes)
- **Semantic motion**: Driven by QualiaDB meaning, not arbitrary loops
- **Respect `prefers-reduced-motion`**: At every layer

### Design Principles
1. **Provenance-first**: Motion reveals where knowledge comes from (NQuin sources)
2. **Consent & dignity**: Calm, legible motion; never anxiety-inducing
3. **Accessible by default**: Static equivalents for all animations
4. **Performance-respectful**: Canvas2D everywhere today; WgpuRenderer is the upgrade
5. **Semantic, not random**: Motion driven by QualiaDB meaning

### Layered Strategy
- **Layer A (immediate)**: Animated icons & micro-interactions (breathing/parallax, hover effects, semantic state pulses)
- **Layer B**: Component micro-3D (card depth, tilt-on-hover, parallax, provenance tokens)
- **Layer C (differentiator)**: Spatial views (knowledge graph, chat-graph DAG, provenance chains, QDP discovery)
- **Layer D**: Full native scenes (anatomy, physics/science)

### Implementation Requirements
- Add `render::motion` module with Timeline, Tween/Spring, Anim<T>, IconScene
- One global Timeline per app; components subscribe
- Shared MotionProfile across 274 icons for coherence
- Semantic hooks: qapp_engine events drive animation targets
- Cheap path first (SVG/CSS via timeline) before WgpuRenderer

### Technologies
- Dioxus, web-sys, SVG/CSS transforms, WebGPU (future), semantic graph binding

---

## Application Lifecycle

*Source: WEBIZEN_APP_LIFECYCLE.md*

### Main Purpose
- Enable every QApp to be editable and forkable (personal versions)
- Support export as wasm PWAs (mobile) and local apps (desktop)
- Ground editability in the `.q42app` hypermedia format

### Technical Requirements
- **`.q42app` format**: Declarative UI + capability bindings + shapes + signature (data, not compiled code)
- **QApp Studio canvas**: For editing
- **Three export targets**: Browser (instant), mobile (wasm PWA), desktop (wasm-in-webview or native Rust)
- **Content-addressed**: By q_hash; re-sign with personal DID for forks

### Design Principles
- **Editability rule**: Every QApp editable and forkable without permission/rebuild
- **Fork + re-sign**: Copy the `.q42app`, modify, re-sign with your DID
- **Personal vs published**: Personal forks stay in Personal realm; publishing is separate, consented act
- **Built-in disciplines exportable**: 274 disciplines should be exportable to `.q42app` (scriptable from catalogue)

### Export Targets

#### 1. Mobile (wasm PWA)
- Template: `qualia-mobile-harness`
- Build features: `android_pwa_edge`, `ios_pwa_edge`
- Persistence: OPFS + WAL
- Footprint: Use `profile_minimal_512` on qualia-core-db (≤512 MB RAM)
- Optional engine: qualia-core-db is optional dependency

#### 2. Desktop Local Apps
- **Default**: wasm-in-webview (instant, no compile, fully sandboxed, editable)
- **Advanced**: Native Rust (Dioxus-desktop/Tauri binary) for native APIs/perf

### Implementation Recommendations
1. Build `.q42app` reader/writer (Hypermedia format)
2. Export 274 disciplines to `.q42app` (scriptable)
3. Implement "fork + re-sign" flow in QApp Studio
4. Build "export to mobile/desktop" templating
5. Replace inline `html5-qrcode` JS with `web-sys` BarcodeDetector/getUserMedia

### Technologies
- `.q42app` format, OPFS, WAL, Tauri, Dioxus-desktop, WebTorrent, fiduciary_crypto, key_vault

---

## Consciousness & Interaction Model

*Source: WEBIZEN_CONSCIOUSNESS_INSIGHTS.md*

### Main Purpose
- Extract philosophical, scientific, and design considerations from 96 consciousness transcripts
- Ground Webizen architecture in consciousness research
- Map insights to concrete architectural decisions

### Architecture Mappings

| Insight | Architectural Mapping |
|---|---|
| Observer's choice shapes reality | **Omnibox** is "Process 1": user's free act of inquiry |
| Attention is sacred | Motion/notifications serve comprehension, never capture |
| Consciousness is non-computable | LLM framed as tool, never as mind (HCAI: "signature is not authorization") |
| Sustained attention holds states | UI supports deep focus: calm defaults, reduced-motion, no interruption |
| Observer participates in reality | Every interaction is provenance-stamped (AS OF queries) |
| Design recursively shapes designers | **P6 — Ontological Responsibility**: every default is an ethical choice |
| Spaces shape minds | **Realms as places**: Personal is warm/close, Mind is open/abstract |
| Language creates realities | **P7 — Ontology Workbench** as headline feature; author vocabularies |
| Right-hemisphere attention | **3D spatial views** (Layer C): knowledge graphs, chat DAG, provenance chains |
| Left-hemisphere attention | **QApp panels**, SPARQL playground, form-based analysis |
| Relational is primary | **Realm model** integrates both: spatial overview + drill-down analysis |

### Confirmed Principles
- P1: Attention is sacred
- P2: Human is participant-author, not user
- P3: Truth has provenance
- P4: Externalise and spatialise mind
- P5: Stewardship and commons
- P6: Ontological responsibility
- P7: Language/semantics as expansion

---

## Hypermedia Format

*Source: WEBIZEN_HYPERMEDIA_FORMAT.md*

### Main Purpose & Scope
- Define `.q42app` hypermedia format for declarative, editable applications
- Support semantic structure, 3D embedding, temporal dimensions
- Enable content-addressed, signed applications

### Format Requirements
- **Semantic structure**: Rich semantic markup for content understanding
- **3D embedding**: Native support for 3D content embedding
- **Temporal dimensions**: Support for time-based and temporal content
- **Qualia integration**: Direct integration with QualiaDB semantic graph
- **Declarative UI**: Component-based, not compiled code
- **Capability bindings**: Secure, signed access to system capabilities
- **Content-addressed**: By q_hash for integrity and deduplication

### Content Types
- **Spatial content**: 3D models, environments, spatial data
- **Temporal content**: Time-series, animations, temporal graphs
- **Semantic content**: Knowledge graphs, ontologies, semantic relationships
- **Interactive content**: User-interactive elements and behaviors

### Implementation Requirements
- **Parser/serializer**: Efficient parsing and serialization of hypermedia
- **Validation**: Schema validation for content integrity
- **Extension points**: Plugin architecture for custom content types
- **Versioning**: Support for format evolution and backward compatibility
- **Signature verification**: DID-based signature verification for security
- **Capability system**: Fine-grained capability requests and grants

---

## Network Architecture

*Source: WEBIZEN_NETWORK.md*

### Main Purpose
- Define P2P networking architecture for distributed features
- Enable decentralized identity and secure communication
- Support offline-first operation

### Network Requirements
- **P2P capabilities**: Peer-to-peer communication for distributed features
- **Decentralized identity**: User identity not dependent on central servers
- **Secure communication**: Encrypted and authenticated communication
- **Offline support**: Core functionality available without network

### Protocols & Standards
- **Libp2p integration**: Use libp2p for P2P networking
- **DNS integration**: Integration with QDP DNS for decentralized naming
- **WebRTC**: Real-time communication for collaborative features
- **IPFS**: Distributed file storage for content sharing

### Performance & Reliability
- **Connection management**: Robust connection handling and recovery
- **Bandwidth optimization**: Efficient data transfer and compression
- **Latency management**: Minimize latency for interactive features
- **Fallback mechanisms**: Graceful degradation when network unavailable

---

## QualiaDB Integration

*Source: WEBIZEN_QUALIADB_GAPS.md*

### Main Purpose
- Identify gaps between current QualiaDB capabilities and Webizen requirements
- Define integration points and required features
- Plan implementation of missing capabilities

### Integration Requirements
- **Semantic graph**: Direct integration with QualiaDB knowledge graph
- **Tensor operations**: GPU-accelerated tensor processing for AI features
- **Query interface**: Efficient query interface for semantic searches
- **Event system**: Reactive event system for real-time updates

### Identified Gaps
- **Crypto capabilities**: Review and implement required cryptographic features
- **Storage drivers**: Implement required storage abstraction layers
- **Query optimization**: Optimize semantic queries for 3D context
- **Caching strategy**: Implement caching for frequently accessed semantic data
- **GPU integration**: Deep integration with WGPU for tensor operations

### Data Synchronization
- **Local storage**: Efficient local storage and indexing
- **Sync protocol**: Synchronization protocol for multi-device access
- **Conflict resolution**: Strategy for handling concurrent modifications
- **Privacy preservation**: Privacy-preserving synchronization

---

## Realms & Context Management

*Source: WEBIZEN_REALMS.md*

### Main Purpose
- Define realm concept as navigable 3D contexts
- Enable context switching and composition
- Support spatial, temporal, and semantic organization

### Realm Concept
- **Navigable contexts**: Realms as navigable 3D contexts
- **Context switching**: Smooth transitions between different realms
- **Realm composition**: Ability to compose and nest realms
- **Persistence**: Realm state persistence and restoration

### Context Management
- **Spatial organization**: Information organized spatially within realms
- **Temporal organization**: Time-based organization within realms
- **Semantic organization**: Knowledge-based organization within realms
- **User-defined contexts**: Allow users to create custom realms

### Realm Types
- **Personal realm**: Warm/close, personal data and preferences
- **Mind realm**: Open/abstract, knowledge and thinking tools
- **Social realm**: Collaborative spaces and shared contexts
- **Published realm**: Public content and shared applications

### Implementation Requirements
- **Realm graph**: Data structure for realm relationships
- **Navigation system**: 3D navigation between and within realms
- **Rendering pipeline**: Efficient rendering of complex realm hierarchies
- **Memory management**: Efficient memory usage for large realm graphs

---

## Technical Organization

*Source: WEBIZEN_REORG_NOTES.md, WEBIZEN_FLUTTER_REVIEW.md*

### Crate Structure

#### webizen-desktop (Tauri-based desktop application)
- Native system integration
- WGPU device management
- IPC bridge to web UI
- Desktop-specific features (system tray, file system access)

#### webizen-runtime (Shared WGPU runtime)
- Device abstraction
- Memory management
- Compute pipeline
- Shared between desktop and web

#### webizen-render (3D rendering engine)
- Rendering pipeline
- Shader management
- Frame delivery
- **Status**: Promote from `render/` after WgpuRenderer implementation

#### webizen-studio (Dioxus-based web UI)
- User interface
- 3D canvas integration
- State management
- QApp editing capabilities

### Technology Decisions
- **Rust**: Core implementation language for performance and safety
- **Tauri**: Desktop application framework (chosen over Electron for performance)
- **Dioxus**: Reactive UI framework for web interface
- **WebGPU**: Cross-platform 3D rendering API (WGPU 0.19)
- **QualiaDB**: Semantic graph database for knowledge management

### Technology Rejections
- **Flutter**: Review indicated poor fit for 3D-native requirements
  - Flutter's 3D capabilities are immature
  - Performance concerns for complex 3D scenes
  - Limited WebGPU support
- **Electron**: Too resource-heavy for performance requirements
- **Traditional web frameworks**: Not suitable for 3D-native interface

### Reorganization Notes
- **8 crate boundary**: Organize into 8 distinct crates for clear separation of concerns
- **Chat-graph cleanup**: Separate chat functionality into dedicated module
- **Clear dependencies**: Define clear dependency relationships between crates

---

## Implementation Priorities

*Source: WEBIZEN_MASTER_PLAN.md, Current Implementation Status Analysis*

### Phase 1: Foundation (CRITICAL - Current Blockers)
- [ ] **Fix critical build errors**: Remove duplicate QAppsRoute definition in webizen-studio/src/main.rs:51-52
- [ ] **Establish stable build pipeline**: Fix all compilation warnings and errors
- [ ] **Code formatting**: Run `cargo fmt` to fix formatting issues
- [ ] **Fix Dioxus frontend rendering**: Resolve blank screen issue blocking anatomy stress test
- [ ] **QualiaDB integration setup**: Basic connection and query capabilities

### Phase 2: Complete 3D Engine (PARTIALLY IMPLEMENTED)
- [ ] **Implement glTF/atlas loading**: For biomedical assets and general 3D content
- [ ] **Add orbit/arc-ball camera controls**: Interactive camera manipulation
- [ ] **Implement pick/hover interaction**: SceneItem selection and interaction
- [ ] **Add advanced lighting/shading**: Lambert → PBR lighting model
- [ ] **Create SceneSource over SPARQL**: Query → SemanticScene via qapp_engine

### Phase 3: Core 3D Interface (NEW PRIORITY)
- [ ] **Implement spatial navigation system**: 3D navigation between and within contexts
- [ ] **Build basic realm management**: Realm creation, switching, persistence
- [ ] **Create 3D interaction primitives**: Pick, hover, manipulate 3D objects
- [ ] **Implement consciousness-inspired feedback**: Attention indication, anticipatory behavior
- [ ] **Motion system**: Implement Timeline, Tween/Spring, Anim<T> for semantic motion

### Phase 4: Hypermedia & Content
- [ ] **Implement hypermedia format parser**: `.q42app` reader/writer
- [ ] **Build content ingestion system**: GLB ingestion (partially complete for anatomy), asset loading
- [ ] **Create 3D content embedding**: Embed 3D content in hypermedia
- [ ] **Implement semantic search**: QualiaDB-powered search
- [ ] **Export built-in disciplines**: Scriptable export of 274 disciplines to `.q42app`

### Phase 5: Network & Distribution
- [ ] **Implement P2P networking**: Libp2p integration
- [ ] **Build decentralized identity**: DID-based identity system
- [ ] **Create sync protocol**: Multi-device synchronization
- [ ] **Implement collaborative features**: Real-time collaboration
- [ ] **Mobile PWA export**: `qualia-mobile-harness` template

### Phase 6: Audio System (LOW PRIORITY - PAUSED)
- [ ] **Implement AudioSpectralSheet**: Complete data structure implementation
- [ ] **Add audio file ingestion**: Memory-mapped audio loading
- [ ] **Implement spectral analysis**: FFT-based frequency domain processing
- [ ] **WebGPU audio visualization**: GPU-accelerated audio rendering
- **Status**: Awaiting anatomy project validation before resuming

### Phase 7: Polish & Optimization
- [ ] **Performance optimization**: 60fps rendering target, memory optimization
- [ ] **User experience refinement**: Accessibility, reduced motion, calm defaults
- [ ] **Documentation and testing**: Comprehensive test coverage
- [ ] **Developer experience**: APIs and tools for third-party developers

---

## Open Questions & Research Areas

- [ ] **Gesture recognition**: Best approach for 3D gesture input (WebXR, hand tracking)
- [ ] **Performance targets**: Specific performance metrics for different hardware tiers
- [ ] **Accessibility**: How to make 3D interface accessible to users with disabilities
- [ ] **Migration strategy**: How to migrate users from traditional 2D interfaces
- [ ] **Developer experience**: Tools and APIs for third-party QApp developers
- [ ] **Storage scaling**: How to handle large-scale QualiaDB deployments
- [ ] **Network topology**: Optimal P2P topology for Webizen use cases

---

## Success Criteria

### Current Achievement Status
- [ ] **Build stability**: ❌ CRITICAL ERROR - Duplicate QAppsRoute definition blocking compilation
- [ ] **10D backplane**: ✅ COMPLETE - Phases 1-5 implemented (data contract, quantum UI, spectral rendering, hardware detection, binary IPC)
- [ ] **3D engine foundation**: ✅ PARTIAL - Basic WGPU renderer, scene contracts, math modules implemented
- [ ] **Zero-heap compliance**: ✅ MOSTLY COMPLIANT - Core tensor operations use stack allocation, documented unavoidable heap usage
- [ ] **Binary IPC**: ✅ COMPLETE - TensorBufferView and BinaryNodeRegistry for zero-heap data transfer
- [ ] **Anatomy project**: ⚠️ BLOCKED - Backend infrastructure complete, frontend rendering issue blocking stress test
- [ ] **Audio system**: ⚠️ PAUSED - Data contract complete, implementation awaiting anatomy validation

### Remaining Success Criteria
- [ ] **Performance**: 60fps rendering on target hardware, <100ms interaction latency
- [ ] **User experience**: Intuitive 3D navigation and interaction
- [ ] **Semantic understanding**: Effective integration with QualiaDB
- [ ] **Network reliability**: Robust P2P networking with offline support
- [ ] **Cross-platform**: Consistent experience across desktop platforms
- [ ] **Editability**: Every QApp can be edited and forked without technical barriers
- [ ] **Provenance**: Every interaction and piece of content shows clear provenance
- [ ] **Accessibility**: Full keyboard navigation, screen reader support, reduced motion
- [ ] **Privacy**: User control over data collection, storage, and sharing

---

## Implementation Status Summary

**Overall Progress: ~40% Complete**

### Completed Major Components ✅
1. **10D Tensor Backplane** (Phases 1-5): High and medium priority phases complete
2. **3D Rendering Foundation**: WGPU renderer, scene contracts, math modules
3. **Binary IPC System**: Zero-heap data transfer infrastructure
4. **Quantum UI Components**: Epistemic status, temporal scrubber, hardware detection
5. **Spectral Rendering**: CIE XYZ projection with stack-allocated matrices
6. **Anatomy Backend**: GLB ingestion, semantic extraction, tensor mapping

### Critical Blockers ❌
1. **Build Error**: Duplicate QAppsRoute definition preventing compilation
2. **Frontend Rendering**: Dioxus blank screen issue blocking anatomy stress test
3. **Code Formatting**: Extensive formatting issues across codebase

### Next Critical Steps
1. Fix duplicate route definition (5 minutes)
2. Run cargo fmt (5 minutes)
3. Resolve Dioxus rendering issue (1-2 hours)
4. Execute anatomy stress test (30 minutes)
5. Complete 3D engine interaction features (2-3 days)

---

*Last Updated: 2026-06-16*
*Next Review: After fixing critical build errors and frontend rendering issues*