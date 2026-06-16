# Webizen Browser - Agent Knowledge Base

This document contains essential project knowledge to help future development sessions proceed efficiently.

## Core Architecture

**CRITICAL**: Webizen is NOT a traditional 3D engine. It is an **Edge-Native 10D Epistemic Manifold Host** - a geometrically executable epistemic engine that manages real-time physical simulation of human-centric knowledge, sovereignty, and multi-modal perception.

### Architectural Paradigm
- **High-Dimensional Relational Processor**: Manages dense memory-map of 10D packed vectors, executes geometric projections in spacetime manifold
- **Spectral Synthesis Conductor**: Treats all perception data as continuous physical spectrum with unified invariants [α, μ, σ]
- **Gravito-Thermodynamic Engine**: Semantic weights as mass, activation as thermal energy, logic as physical forces
- **Epistemic Anchor Coordinator**: Manages quantum contexts, wavefunction collapse, and decentralized reality interface

See [ARCHITECTURE_OVERVIEW.md](ARCHITECTURE_OVERVIEW.md) for detailed architectural documentation.

## Build Commands

### Standard Build
```bash
cargo build
```

### Release Build
```bash
cargo build --release
```

### Code Quality Checks
```bash
# Check formatting
cargo fmt --check

# Run linter
cargo clippy

# Run tests
cargo test
```

## Verification Steps

### After Any Changes
- Run `cargo build` to ensure compilation succeeds

### Before Commits
- Run `cargo fmt` to format code
- Run `cargo clippy` to catch potential issues

### For UI Changes
- Test in Tauri dev mode to verify frontend rendering

## Common Issues

### Duplicate Route Definition
- **Location**: Check `webizen-studio/src/main.rs`
- **Symptom**: Build errors about duplicate routes
- **Fix**: Review route definitions and remove duplicates

### Frontend Blank Screen
- **Location**: Dioxus frontend in webizen-studio
- **Symptom**: UI renders blank/white screen
- **Check**: Dioxus version compatibility and dist assets
- **Status**: Currently being investigated

### QualiaDB Build Issues
- **Location**: External dependency
- **Reference**: See `QUALIADB_BUILD_FIXES.md` for detailed fixes
- **Status**: Known blocker, documented fixes available

### .rsx Module Loading
- **Symptom**: Issues loading .rsx modules
- **Status**: Currently being investigated
- **Note**: Active investigation in progress

## Project Status

### 10D Backplane
- **Progress**: 80% complete
- **Completed**: Phases 1-5
- **Remaining**: Phases 6-10

### 3D Engine
- **Progress**: 40% complete
- **Completed**: Foundation layer
- **Missing**: Interaction layer

### Anatomy Project
- **Backend**: Ready
- **Frontend**: Blocked (awaiting resolution of frontend issues)

### Audio System
- **Status**: Designed
- **Implementation**: Paused

## Zero-Heap Mandate

### Core Design Principle
Minimize heap allocation throughout the codebase for performance and predictability.

### Guidelines
- Use stack-allocated types (Copy types, arrays, primitives)
- Binary IPC uses u64 indices instead of String IDs
- Document any unavoidable heap usage with clear justification

### Implementation Notes
- This is a core architectural decision
- Affects data structures, IPC, and serialization choices
- Review all new code for heap allocation patterns

## Workspace Structure

```
webizen-browser/
├── webizen-desktop/    # Tauri desktop application
├── webizen-runtime/    # Shared WGPU runtime
├── webizen-render/     # 3D rendering engine
└── webizen-studio/     # Dioxus web UI
```

### Component Responsibilities
- **webizen-desktop**: Desktop app shell, native integration
- **webizen-runtime**: WGPU context management, shared GPU resources
- **webizen-render**: 3D rendering pipeline, scene graph
- **webizen-studio**: Web-based UI, editor interface

## Current Blockers

### 1. QualiaDB Build Issues
- **Type**: External dependency
- **Impact**: Blocks builds that depend on QualiaDB
- **Reference**: `QUALIADB_BUILD_FIXES.md`
- **Status**: Documented workarounds available

### 2. .rsx Module Loading
- **Type**: Frontend infrastructure
- **Impact**: Affects Dioxus component loading
- **Status**: Active investigation

### 3. Dioxus Frontend Rendering
- **Type**: Frontend rendering
- **Symptom**: Blank screen issue
- **Impact**: Blocks UI development and testing
- **Status**: Investigation in progress

## Development Workflow

1. **Start**: Review this document for context
2. **Build**: Run `cargo build` to establish baseline
3. **Develop**: Make changes following Zero-Heap mandate
4. **Verify**: Run appropriate verification steps
5. **Test**: Use Tauri dev mode for UI changes
6. **Commit**: Ensure `cargo fmt` and `cargo clippy` pass

## Additional Resources

- `QUALIADB_BUILD_FIXES.md` - Specific fixes for QualiaDB issues
- Project README.md - General project information
- Cargo.toml files - Dependency and workspace configuration
