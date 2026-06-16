# Webizen Architecture Overview
*Edge-Native 10D Epistemic Manifold Host*

## Core Identity

**webizen-desktop** is not a traditional 3D engine. It is a **Geometrically Executable Epistemic Engine** and **10D Volumetric Relational Tensor Host** that manages real-time physical simulation of human-centric knowledge, sovereignty, and multi-modal perception.

### Architectural Paradigm

The system treats the entire Human-Centric knowledge base as a pre-compiled, quantized physics engine for logic, provenance, and multi-modal perception. Rather than treating 3D space as a visual destination, it uses lower-dimensional projections as a zero-heap, stateless diagnostic interface for navigating high-dimensional graphs.

---

## Operational Pillars

### 1. High-Dimensional Relational Processor

**Traditional Architecture (Outdated):**
- Backend queries database → serializes tables to JSON → 3D engine uses scene graph → coordinates to meshes

**Actual Reality:**
- **Dense Memory Management**: Host manages un-de-serialized memory-map (`mmap`) of 10D packed vectors
- **Geometric Execution**: Queries are geometric projections and distance calculations executed directly within spacetime manifold `[q, v, w, x, y, z, t]`
- **Algebraic Traversal**: Bypasses traditional traversal by evaluating algebraic varieties (`v`) and cross-domain bifurcations (`w`) via raw matrix math

### 2. Spectral Synthesis Conductor

**Traditional Approach (Outdated):**
- Manage arbitrary asset files (PNGs, MP3s) → throw at standard browser decoders

**Actual Reality:**
- **Unified Physical Spectrum**: All perception data treated as continuous physical spectrum
- **Multi-Modal Invariants**: Attribute channels `[α, μ, σ]` represent absolute physical invariants:
  - `α` (Amplitude): Linear intensity
  - `μ` (Modulation): Carrier information
  - `σ` (Spectral): Frequency signatures
- **Pipeline Orchestration**: Desktop host feeds raw visual Spectral Power Distributions (SPD) to GPU shaders and time-frequency sheets (STFT/CQT) to audio worklets
- **Browser as Client**: Browser reduced to hardware-aware client executing localized projection of unified physical truth

### 3. Gravito-Thermodynamic Knowledge Engine

**Mechanical Physics Applied to Knowledge:**
- **Semantic Mass**: Semantic weights act as mass exerting gravitational attraction (`α`)
- **Thermal Dynamics**: Activation states and cluster loads behave as thermal energy and local pressure (`T` and `P`)
- **Geometric Equilibrium**: Engine bakes dynamics into geometric equilibrium before data touches screen
- **Mechanical Sympathy**: Logic can "pull" and "react" with physical force simulation

### 4. Asynchronous Epistemic Anchor Coordinator

**Gateway to Decentralized Reality:**
- **Tier 3 Ground-State Resolvers**: Interface with GSR/Quantum Annealers without blocking runtime loop
- **Quantum Sandbox**: Isolates probabilistic or sandboxed inferences in non-zero quantum contexts (`q > 0`)
- **Wavefunction Collapse**: Upon cryptographic verification or user selection:
  - Locks context to `q=0` (Ground Truth)
  - Generates permanent historical logging slice (`t`)
  - Triggers cascade re-crystallization of local manifold

---

## Technical Architecture

### 10D Tensor Structure
```rust
// [q, v, w, x, y, z, t, α, μ, σ]
pub struct Tensor10DProjection {
    q: f64,  // Quantum context (0 = ground truth, >0 = probabilistic)
    v: f64,  // Algebraic variety (semantic dimension)
    w: f64,  // Cross-domain bifurcation (context switching)
    x: f64,  // Spatial coordinate
    y: f64,  // Spatial coordinate  
    z: f64,  // Spatial coordinate
    t: f64,  // Temporal coordinate
    α: f64,  // Amplitude (semantic mass/gravity)
    μ: f64,  // Modulation (carrier information)
    σ: f64,  // Spectral signature (frequency)
}
```

### Zero-Heap Binary IPC
- **String → u64 Index Mapping**: BinaryNodeRegistry converts semantic IDs to stack-allocated indices
- **Zero-Copy Transport**: TensorBufferView provides memory-mapped access without heap allocation
- **Stack-Only Operations**: All geometric calculations use stack-allocated f64 values

### Distributed System Components

**webizen-desktop (Tauri)**:
- Epistemic manifold host
- 10D tensor computation backend
- Binary IPC coordinator
- Quantum context management
- Wavefunction collapse execution

**webizen-studio (Dioxus)**:
- Diagnostic viewport for manifold projection
- UI controls for epistemic interaction
- Stateless display layer
- Hardware capability detection

**webizen-runtime (WGPU)**:
- GPU shader execution for spectral synthesis
- Native graphics operations
- Hardware abstraction layer

**webizen-render**:
- Geometric projection mathematics
- Camera as manifold viewpoint adjustment
- Ray casting as epistemic anchor coordination
- Stack-allocated geometric operations

---

## Functional Redefinition

### Camera Controls
**Old**: Navigate 3D scene for visual exploration
**New**: Adjust manifold projection viewpoint for geometric examination of epistemic relationships

### Object Picking/Hovering
**Old**: Select 3D objects for manipulation
**New**: Coordinate epistemic anchors and quantum context selection via ray casting through manifold

### Rendering
**Old**: Generate visual appeal from scene graphs
**New**: Project lower-dimensional diagnostic interface from high-dimensional geometric truth

### Asset Loading
**Old**: Load files for display
**New**: Ingest spectral data for unified physical synthesis pipeline

---

## Zero-Heap Mandate

**Core Principle**: Minimize heap allocation during geometric execution to maintain real-time physical simulation performance.

**Compliance Areas**:
- Tensor operations: Stack-allocated f64 calculations
- Binary IPC: u64 indices instead of String IDs
- Geometric math: Stack-only matrix operations
- Memory mapping: Zero-copy buffer access
- GPU operations: Separate from Rust heap

**Documented Unavoidable Heap Usage**:
- Frontend UI state (Dioxus/React inherent)
- String types (user-facing data)
- Tauri IPC (cross-process serialization)
- Browser APIs (uncontrollable)
- Dynamic data structures (sizing requirements)

---

## Development Philosophy

The system is designed as an **edge-native physics simulation** where:
- Knowledge is treated as physical matter with mass, energy, and forces
- Queries are geometric operations, not database lookups
- Perception is unified spectral synthesis, not asset management
- User interaction is quantum context manipulation, not UI state changes
- The browser is a diagnostic client, not the primary execution environment

This architectural understanding should guide all development decisions, code organization, and feature implementation.

---

*Last Updated: 2026-06-16*
*Architectural Paradigm: Geometrically Executable Epistemic Engine*