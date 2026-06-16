# Webizen Development Pipeline
*Created: 2026-06-16*
*Purpose: Consolidated development approach for Edge-Native 10D Epistemic Manifold Host*

## Core Identity: Geometrically Executable Epistemic Engine

**CRITICAL**: webizen-desktop is NOT a traditional 3D engine. It is a **10D Volumetric Relational Tensor Host** that manages real-time physical simulation of human-centric knowledge, sovereignty, and multi-modal perception.

### Architectural Paradigm
- **High-Dimensional Relational Processor**: Manages dense memory-map of 10D packed vectors, executes geometric projections in spacetime manifold
- **Spectral Synthesis Conductor**: Treats all perception data as continuous physical spectrum with unified invariants [α, μ, σ]
- **Gravito-Thermodynamic Engine**: Semantic weights as mass, activation as thermal energy, logic as physical forces
- **Epistemic Anchor Coordinator**: Manages quantum contexts, wavefunction collapse, and decentralized reality interface

### Zero-Heap Mandate

**CRITICAL**: All development work must respect the zero-heap mandate as a fundamental design principle.

### Zero-Heap Philosophy
- **Primary Goal**: Minimize heap allocation during runtime operations, especially in rendering loops and tensor operations
- **Stack Allocation Priority**: Use stack-allocated types (Copy types, arrays, primitives) wherever possible
- **Zero-Copy Operations**: Use byte references and views instead of copying data
- **Binary IPC**: Use u64 indices instead of String IDs for cross-process communication

### Documented Unavoidable Heap Usage
The following heap usage patterns are documented as unavoidable and acceptable:
- Frontend UI components (Dioxus/React state management is inherently heap-based)
- String types (Rust String is heap-allocated for user-facing data)
- Tauri IPC (cross-process serialization requires heap allocation)
- Browser APIs (WebGPU, AudioContext return heap-allocated objects)
- Dynamic data structures (Vec<T> for dynamic sizing requirements)

### Zero-Heap Compliance Checklist
Before implementing any feature, verify:
- [ ] Core data structures use Copy types where possible
- [ ] Runtime operations use stack allocation
- [ ] Data transfer uses zero-copy views (byte references)
- [ ] IPC uses binary indices instead of strings
- [ ] GPU operations don't depend on Rust heap
- [ ] Document any unavoidable heap usage with justification

---

## Pipeline Overview

This pipeline addresses issues from the [20260616_audit.md](20260616_audit.md) while supporting the implementation priorities in [webizen_design_notes.md]. It's designed to work around current QualiaDB build issues by focusing on independent work first, **while strictly adhering to the zero-heap mandate**.

---

## Phase 0: Quick Wins (No QualiaDB Dependency) - 30 minutes

### 0.1 Fix Critical Build Error - 5 minutes
**Priority**: CRITICAL  
**Blocks**: All compilation and testing  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: None (code organization fix)

**Task**: Remove duplicate QAppsRoute definition
```rust
// File: webizen-studio/src/main.rs:51-52
// REMOVE THESE LINES:
#[route("/qapps")]
QAppsRoute {},  // Duplicate - DELETE THIS
```

**Verification**:
```bash
cargo build
```

**Success Criteria**: Project compiles without the duplicate route error

---

### 0.2 Code Formatting - 5 minutes
**Priority**: HIGH  
**Blocks**: Code quality and consistency  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: None (code formatting only)

**Task**: Run cargo fmt to fix formatting issues
```bash
cargo fmt
```

**Expected Impact**: Fix trailing whitespace, blank line inconsistencies, import ordering in:
- webizen-desktop/src/commands/binary_registry.rs
- webizen-desktop/src/commands/glb_ingest.rs
- Multiple other files across workspace

**Verification**:
```bash
cargo fmt --check
```

**Success Criteria**: No formatting differences reported

---

### 0.3 Fix Clippy Warnings - 20 minutes
**Priority**: MEDIUM  
**Blocks**: Code quality  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Positive (removes unused heap allocations)

**Task**: Address clippy warnings in webizen-studio

**Files to Fix**:
1. `webizen-studio/src/components/mod.rs` - Duplicate module loading
   ```rust
   // Change from:
   pub mod epistemic_status;
   pub mod temporal_scrubber;
   pub mod hardware_capabilities;
   pub mod anatomy_test;
   
   // To (keep one as mod, others as use):
   pub mod epistemic_status;
   use crate::components::temporal_scrubber::TemporalScrubber;
   use crate::components::hardware_capabilities::HardwareCapabilities;
   use crate::components::anatomy_test::AnatomyTest;
   ```

2. `webizen-studio/src/components/marine_biology_qapp.rs:11` - Unused variable
   ```rust
   // Change from:
   let mut dissolved_o2_mgl = use_signal(|| 7.0f64);
   
   // To:
   let _dissolved_o2_mgl = use_signal(|| 7.0f64);
   ```

3. `webizen-studio/src/components/meteorology_qapp.rs:11` - Unused variable
   ```rust
   // Change from:
   let mut visibility_km = use_signal(|| 10.0f64);
   
   // To:
   let _visibility_km = use_signal(|| 10.0f64);
   ```

**Verification**:
```bash
cargo clippy
```

**Success Criteria**: No clippy warnings in webizen-studio

---

## Phase 1: Code Quality & Foundation (No QualiaDB) - 2 hours

### 1.1 Fix Dead Code in webizen-runtime - 30 minutes
**Priority**: MEDIUM  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Positive (removes unused heap allocations)

**Task**: Remove unused struct fields in WgpuDiffusionBackend
```rust
// File: webizen-runtime/src/wgpu_backend.rs:124-129
// Either use these fields or remove them:
surface: Option<wgpu::Surface<'a>>,
render_pipeline: Option<wgpu::RenderPipeline>,
vertex_buffer: Option<wgpu::Buffer>,
index_buffer: Option<wgpu::Buffer>,
depth_texture: Option<wgpu::Texture>,
depth_texture_view: Option<wgpu::TextureView>,
```

**Approach**: If these are planned for future use, add `#[allow(dead_code)]` attribute. If not, remove them.

---

### 1.2 Standardize Dependency Versions - 30 minutes
**Priority**: MEDIUM  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Neutral (dependency management)

**Task**: Resolve reqwest version mismatch
- Current: webizen-desktop uses 0.12, webizen-studio uses 0.13.4
- Action: Standardize on 0.13.4 across workspace

**Files to Update**:
- `webizen-desktop/Cargo.toml`: Change reqwest from "0.12" to "0.13"

**Verification**:
```bash
cargo build
```

---

### 1.3 Add Basic Tests - 1 hour
**Priority**: MEDIUM  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Positive (validates zero-heap compliance)

**Task**: Add basic unit tests for core modules

**Test Areas**:
1. BinaryNodeRegistry tests (already exist, verify they pass)
2. Tensor10DProjection tests
3. Spectral projection tests
4. Hardware tier determination tests

**Example Test**:
```rust
// File: webizen-render/src/scene_contract.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tensor_default() {
        let tensor = Tensor10DProjection::default();
        assert_eq!(tensor.q, 0.0);
        assert_eq!(tensor.sigma, 0.0);
        // Zero-heap: Tensor10DProjection is Copy type, stack-allocated
    }

    #[test]
    fn test_spectral_to_color() {
        let mut tensor = Tensor10DProjection::default();
        tensor.sigma = 0.5;
        let color = tensor.spectral_to_color();
        assert!(color.starts_with("rgb("));
        // Zero-heap: Stack-allocated CIE XYZ matrices in projection
    }

    #[test]
    fn test_tensor_copy_type() {
        let tensor1 = Tensor10DProjection::default();
        let tensor2 = tensor1; // Copy, no heap allocation
        assert_eq!(tensor1.q, tensor2.q);
        // Zero-heap: Copy type enables stack-only operations
    }
}
```

**Verification**:
```bash
cargo test
```

---

## Phase 2: Frontend & UI (No QualiaDB) - 3 hours

### 2.1 Debug Dioxus Frontend Rendering - 2 hours
**Priority**: HIGH  
**Blocks**: Anatomy stress test, general UI functionality  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Neutral (frontend UI inherently uses heap, but backend should remain zero-heap)

**Current Issue**: Tauri application shows blank screen

**Debugging Steps**:
1. **Check Dioxus version compatibility**
   ```bash
   # Check current Dioxus version in webizen-studio/Cargo.toml
   # Verify it's compatible with current Rust version
   ```

2. **Verify dist assets**
   ```bash
   # Check if webizen-studio/dist exists and contains:
   # - index.html
   # - JS bundle
   # - WASM file
   ls webizen-studio/dist/
   ```

3. **Test simple component**
   ```rust
   // Temporarily simplify main.rs to test basic rendering
   fn main() {
       dioxus::launch(|| {
           rsx! {
               div { "Hello World" }
           }
       });
   }
   ```

4. **Check browser console for errors**
   - Open DevTools in Tauri window
   - Look for JavaScript errors, WASM loading failures

5. **Verify Tauri configuration**
   ```toml
   # Check webizen-desktop/src-tauri/tauri.conf.json
   # Verify frontendDist path is correct
   ```

**Potential Solutions**:
- Rebuild frontend with trunk (if installed)
- Update Dioxus version
- Fix asset path configuration
- Check for missing WASM dependencies

---

### 2.2 Integrate Anatomy Test Component - 1 hour
**Priority**: MEDIUM  
**Blocks**: Anatomy stress test execution  
**QualiaDB Dependency**: None (backend ready, just UI integration)  
**Zero-Heap Impact**: Positive (validates zero-heap binary IPC)

**Task**: Complete anatomy_test.rsx integration

**Current Status**: Component exists but not fully integrated

**Steps**:
1. **Add Tauri command invocations**
   ```rust
   // File: webizen-studio/src/components/anatomy_test.rsx
   use dioxus::prelude::*;
   
   #[component]
   pub fn AnatomyTest() -> Element {
       let test_result = use_signal(|| String::new());
       
       rsx! {
           div {
               h1 { "Anatomy Stress Test" }
               button {
                   onclick: move |_| {
                       // Invoke Tauri command
                       // test_result.set("Executing...");
                   },
                   "Run IPC Handshake Test"
               }
               p { "{test_result}" }
           }
       }
   }
   ```

2. **Add to main navigation**
   ```rust
   // File: webizen-studio/src/main.rs
   // Ensure route is properly configured
   ```

3. **Test component rendering**
   - Navigate to /anatomy-test
   - Verify buttons render
   - Test Tauri command invocation

---

## Phase 3: 3D Engine Features (No QualiaDB) - 4 hours

### 3.1 Implement Manifold Projection Controls - 2 hours
**Priority**: HIGH  
**Supports**: Geometric examination of epistemic relationships  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Positive (stack-allocated geometric operations)

**Task**: Add viewpoint adjustment controls for manifold projection examination

**Architectural Context**: Camera controls are not for visual navigation but for adjusting the projection viewpoint to examine geometric relationships in the 10D epistemic manifold.

**Implementation**:
```rust
// File: webizen-render/src/wgpu_renderer.rs
impl Camera {
    /// Rotate camera around target (orbit control)
    /// Zero-heap: Stack-allocated math operations only
    pub fn orbit(&mut self, yaw: f64, pitch: f64) {
        // Implement orbit rotation math using stack-allocated f64 values
        // No heap allocation in camera transformation
    }
    
    /// Zoom camera in/out
    /// Zero-heap: Stack-allocated scalar operations
    pub fn zoom(&mut self, delta: f64) {
        // Implement zoom math using stack-allocated f64
        // No heap allocation in zoom calculation
    }
    
    /// Pan camera
    /// Zero-heap: Stack-allocated vector operations
    pub fn pan(&mut self, dx: f64, dy: f64) {
        // Implement pan math using stack-allocated f64
        // No heap allocation in pan calculation
    }
}
```

**Integration with Dioxus**:
```rust
// File: webizen-studio/src/components/camera_controls.rsx
#[component]
pub fn CameraControls() -> Element {
    rsx! {
        div {
            button { "Rotate Left" }
            button { "Rotate Right" }
            button { "Zoom In" }
            button { "Zoom Out" }
        }
    }
}
```

---

### 3.2 Implement Epistemic Anchor Coordination - 2 hours
**Priority**: HIGH  
**Supports**: Quantum context selection and wavefunction collapse  
**QualiaDB Dependency**: None  
**Zero-Heap Impact**: Positive (stack-allocated ray casting, binary indices for anchors)

**Task**: Add ray casting for epistemic anchor coordination

**Architectural Context**: Object picking is not for selection but for coordinating epistemic anchors and quantum context selection via ray casting through the manifold. Returns binary indices for zero-heap IPC.

**Implementation**:
```rust
// File: webizen-render/src/wgpu_renderer.rs
impl WgpuRenderer {
    /// Pick object at screen coordinates
    /// Zero-heap: Stack-allocated ray casting, returns binary index
    pub fn pick(&self, x: f64, y: f64) -> Option<usize> {
        // Implement ray casting using stack-allocated math
        // Return binary node index (not String ID) for zero-heap IPC
    }
    
    /// Check if mouse is hovering over object
    /// Zero-heap: Stack-allocated ray casting, returns binary index
    pub fn hover(&self, x: f64, y: f64) -> Option<usize> {
        // Similar to pick but for hover state
        // Return binary node index for zero-heap operations
    }
}
```

**Integration with Dioxus**:
```rust
// File: webizen-studio/src/components/scene_viewer.rsx
#[component]
pub fn SceneViewer() -> Element {
    let hovered_node = use_signal(|| None::<usize>);
    
    rsx! {
        div {
            onmousemove: move |evt| {
                // Convert mouse coordinates to scene coordinates
                // Call renderer.hover()
                // Update hovered_node signal
            },
            // Render scene with hover highlighting
        }
    }
}
```

---

## Phase 4: Documentation & Preparation (No QualiaDB) - 2 hours

### 4.1 Update AGENTS.md - 1 hour
**Priority**: MEDIUM  
**Supports**: Future development efficiency  
**QualiaDB Dependency**: None

**Task**: Document learned project information

**Content to Add**:
```markdown
## Build Commands
- Standard build: `cargo build`
- Release build: `cargo build --release`
- Check formatting: `cargo fmt --check`
- Run clippy: `cargo clippy`
- Run tests: `cargo test`

## Verification Steps
- After any changes: run `cargo build`
- Before commits: run `cargo fmt` and `cargo clippy`
- For UI changes: test in Tauri dev mode

## Common Issues
- Duplicate route definition: Check webizen-studio/src/main.rs
- Frontend blank screen: Check Dioxus version and dist assets
- QualiaDB build issues: See QUALIADB_BUILD_FIXES.md

## Project Status
- 10D backplane: 80% complete (Phases 1-5 done)
- 3D engine: 40% complete (foundation done, interaction missing)
- Anatomy project: Backend ready, frontend blocked
- Audio system: Designed, implementation paused
```

---

### 4.2 Prepare QualiaDB Integration Plan - 1 hour
**Priority**: LOW  
**Supports**: Design notes Phase 1 (Foundation)  
**QualiaDB Dependency**: None (planning only)

**Task**: Create integration checklist for when QualiaDB is ready

**Checklist**:
```markdown
## QualiaDB Integration Preparation

### Pre-Integration Checks
- [ ] QualiaDB builds successfully
- [ ] QualiaDB version compatible with workspace
- [ ] All QualiaDB dependencies resolved

### Integration Steps
1. Update webizen-desktop/Cargo.toml with correct QualiaDB path
2. Test basic QualiaDB connection
3. Implement SPARQL query interface
4. Create SceneSource over SPARQL
5. Test semantic scene generation
6. Integrate with 10D tensor backplane

### Testing
- [ ] Test QualiaDB query execution
- [ ] Test tensor extraction from QualiaDB
- [ ] Test scene generation from semantic data
- [ ] Test end-to-end pipeline
```

---

## Phase 5: QualiaDB Integration (When Available) - 4 hours

### 5.1 Resolve QualiaDB Build Issues - 1 hour
**Priority**: CRITICAL  
**Blocks**: All QualiaDB-dependent features  
**QualiaDB Dependency**: Required

**Task**: Fix QualiaDB build issues

**Steps**:
1. Review QUALIADB_BUILD_FIXES.md
2. Apply documented fixes
3. Test QualiaDB build in isolation
4. Resolve dependency conflicts
5. Verify QualiaDB version compatibility

---

### 5.2 Integrate QualiaDB - 3 hours
**Priority**: HIGH  
**Supports**: Design notes Phase 1 (Foundation)  
**QualiaDB Dependency**: Required

**Task**: Complete QualiaDB integration

**Steps**:
1. Update workspace Cargo.toml with correct QualiaDB path
2. Test basic connection and queries
3. Implement SPARQL → SemanticScene pipeline
4. Integrate with 10D tensor backplane
5. Test end-to-end semantic scene generation

---

## Execution Timeline

### Immediate (Today - 2 hours)
- Phase 0: Quick wins (30 min)
- Phase 1.1: Dead code fix (30 min)
- Phase 1.2: Dependency standardization (30 min)
- Phase 2.1: Start Dioxus debugging (30 min)

### Short-term (This Week - 8 hours)
- Complete Phase 2: Frontend & UI (3 hours)
- Phase 3: 3D Engine Features (4 hours)
- Phase 4: Documentation (1 hour)

### Medium-term (When QualiaDB Ready - 4 hours)
- Phase 5: QualiaDB Integration (4 hours)

---

## Success Metrics

### Phase 0 Success
- [ ] Project compiles without errors
- [ ] No formatting issues
- [ ] No clippy warnings in webizen-studio
- [ ] Zero-heap compliance maintained

### Phase 1 Success
- [ ] No dead code warnings
- [ ] Consistent dependency versions
- [ ] Basic test coverage established
- [ ] All new code follows zero-heap principles

### Phase 2 Success
- [ ] Dioxus frontend renders correctly
- [ ] Anatomy test component functional
- [ ] Tauri commands invokable from UI
- [ ] Backend operations maintain zero-heap compliance

### Phase 3 Success
- [ ] Manifold projection controls implemented
- [ ] Epistemic anchor coordination works
- [ ] Geometric examination interface functional
- [ ] All geometric operations use stack allocation
- [ ] Binary IPC used for epistemic anchor coordination

### Phase 4 Success
- [ ] AGENTS.md updated with project knowledge
- [ ] QualiaDB integration plan documented
- [ ] Team can proceed efficiently when QualiaDB ready
- [ ] Zero-heap best practices documented

### Phase 5 Success
- [ ] QualiaDB builds successfully
- [ ] QualiaDB integration complete
- [ ] Semantic scene generation working
- [ ] End-to-end pipeline functional
- [ ] Zero-heap compliance validated across integration

---

## Risk Mitigation

### QualiaDB Delays
**Risk**: QualiaDB issues take longer than expected  
**Mitigation**: Pipeline designed with extensive QualiaDB-independent work

### Frontend Complexity
**Risk**: Dioxus rendering issue more complex than expected  
**Mitigation**: Can proceed with other phases while investigating

### 3D Engine Scope
**Risk**: Camera controls and interaction more complex than estimated  
**Mitigation**: Start with basic implementations, iterate

### Zero-Heap Compliance
**Risk**: Complex features may require unavoidable heap usage  
**Mitigation**: 
- Document all heap usage with justification
- Prioritize stack allocation in all new code
- Use binary IPC to minimize heap-based data transfer
- Leverage GPU memory (separate from Rust heap) for heavy operations

---

## Next Steps

1. **Execute Phase 0 immediately** (30 min quick wins)
2. **Begin Phase 2.1** (Dioxus debugging) in parallel with other Phase 1 tasks
3. **Assess progress after Phase 0-1 completion**
4. **Adjust timeline based on Dioxus debugging results**

---

*Last Updated: 2026-06-16*
*Pipeline Status: Ready to execute Phase 0*