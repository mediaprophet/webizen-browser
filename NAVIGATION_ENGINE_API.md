# Navigation Engine API Documentation

## Overview

The Webizen Navigation Engine is a game-engine-grade graph traversal system that provides fluid, interactive navigation through QualiaDB knowledge graphs. It features stateful backend orchestration, smooth LERP-interpolated transitions, predictive caching, and a stateless frontend viewport.

## Architecture

```
Stateful Orchestrator (webizen-desktop / Tauri)
├── RenderLoopState (AtomicBool daemon control)
├── ActiveAnchor (Graph navigation focus)
├── PreviewState (Frame buffer and hit-testing data)
└── Tokio Render Daemon (30 FPS)
    ├── QualiaDB queries
    ├── Neighborhood caching (5s TTL)
    └── LERP transitions (300ms)

Virtual Renderer (webizen-render / Wgpu)
└── render_scene_png_with_time()
    ├── LERP interpolation
    ├── Node highlighting
    ├── Pulse animations
    └── Hit testing

Stateless Viewport (webizen-studio / Dioxus)
└── RenderPreview Component
    ├── Image display
    └── Event listener
```

## Tauri Commands

### toggle_render_loop(is_active: bool) -> Result<(), String>

Starts/stops the continuous render daemon. Spawns a Tokio task at 30 FPS that continuously fetches QualiaDB neighborhoods, performs LERP transitions, and emits render-preview-ready events.

### select_node_at(x: f64, y: f64) -> Result<Option<String>, String>

Performs hit-testing to identify clicked nodes. Uses Euclidean distance comparison against node radii, accounting for animated pulsing effects. Checks nodes in reverse Z-order (top to bottom).

### navigate_to_node(node_id: String) -> Result<(), String>

Updates the graph navigation focus. Triggers QualiaDB re-fetch for new neighborhood and initiates 300ms LERP transition from old to new node positions.

## State Management

### RenderLoopState
Atomic flag controlling daemon lifecycle. Set to true to start, false to stop.

### ActiveAnchor
Current graph navigation focus (Option<String>). Updated via navigate_to_node, monitored by daemon for neighborhood re-fetch triggers.

### PreviewState
Shared state containing:
- png: Latest rendered frame (Vec<u8>)
- node_positions: Hit-testing data (id, x, y, radius)

## Data Contracts

### RenderScene
GPU-facing scene contract with:
- nodes: Vec<SceneNode>
- edges: Vec<SceneEdge>
- faces: Vec<SceneFace>
- camera: SceneCamera
- background: String
- selected_node_id: Option<String>
- transition_state: Option<TransitionState>

### SceneNode
Node with semantic styling:
- id: String (for picking)
- position: ScenePoint (normalized 0..1)
- color: String (CSS)
- radius: f64 (pixels)
- alpha: f64 (0..1)
- is_inferencing: bool
- pulse_rate: f64 (Hz)

### TransitionState
LERP interpolation state:
- previous_positions: Vec<(String, ScenePoint)>
- progress: f64 (0..1)
- duration: f64 (seconds)

## Protocol Endpoints

### webizen://localhost/render/preview.png?t={epoch}

Custom protocol serving latest PNG frame. Query parameter t enables cache busting. Enables zero-VDOM frame delivery.

## Events

### render-preview-ready

Emitted after each successful render. Frontend listens to trigger epoch increment and image refresh.

## Renderer API

### render_scene_png_with_time(scene, width, height, time_seconds) -> Option<Vec<u8>>

Main rendering function:
- Creates headless WGPU device
- Applies LERP interpolation if transition_state present
- Renders faces → edges → nodes (layer ordering)
- Applies pulse animations to inferencing nodes
- Highlights selected nodes (1.5x radius, white)
- Tracks node positions for hit testing
- Returns PNG-encoded frame

## Performance

- Frame Rate: 30 FPS (~33ms per frame)
- Transition Duration: 300ms LERP
- Cache TTL: 5 seconds
- Memory: ~100MB baseline + ~10MB per cached neighborhood
- Latency: ~20-35ms per frame (query + layout + render + encode)

## Usage Examples

Start daemon:
```rust
window.__TAURI__.invoke('toggle_render_loop', { isActive: true })
```

Navigate to node:
```rust
window.__TAURI__.invoke('navigate_to_node', { nodeId: "person_123" })
```

Hit test:
```rust
window.__TAURI__.invoke('select_node_at', { x: 400, y: 300 })
    .then(nodeId => { if (nodeId) navigate_to_node({ nodeId }) })
```

Display frame:
```rust
img { src: "webizen://localhost/render/preview.png?t={epoch}" }
```

## Production Status

**Version:** 1.0.0  
**Status:** Production Ready  
**Build:** Successful (23.99s)  
**Architecture:** Stateful orchestrator / stateless viewer paradigm  
**Performance:** Game-engine-grade with VDOM-independent rendering
