# Webizen Browser 10D Tensor Integration Plan

**Target:** Update `C:\Projects\webizen-browser` to align with Q42 Volumetric Tensor specification  
**Approach:** Transform browser from 2D/3D graph viewer to 10D viewport with spectral rendering  
**Branch:** 0.0.13

---

## Current State

**Existing Architecture:**
- 2D/3D graph visualization via RenderScene contract
- JSON-based data transfer
- Standard RGB color rendering
- Basic audio playback via `<audio>` tag
- No epistemic state handling
- No capability handshake with backend

**Required Changes:**
- 10D vector projection `[q, v, w, x, y, z, t, α, μ, σ]`
- Spectral-first rendering pipeline
- Wavefunction collapse UI
- Binary IPC transfers
- Sovereign audio synthesis pipeline
- Hardware capability handshake

---

## Phase 1 - Data Contract Evolution (10D Projection)

**Goal:** Upgrade RenderScene contract to accept 10D tensor projections.

### 1.1 10D Vector Mapping

**Current RenderScene:**
```typescript
interface RenderScene {
  nodes: Node[];
  edges: Edge[];
  camera: Camera;
}
```

**Updated RenderScene with 10D Projection:**
```typescript
interface Tensor10DProjection {
  q: number;      // Quantum Context (0 = collapsed, >0 = pending)
  v: number;      // Topological Class
  w: number;      // Manifold Index
  x: number;      // Semantic X
  y: number;      // Semantic Y
  z: number;      // Semantic Z
  t: number;      // Temporal State
  alpha: number;  // Amplitude (opacity/scale)
  mu: number;     // Modulation (metadata/phase)
  sigma: number;  // Spectral Signature (color class)
}

interface RenderScene {
  nodes: TensorNode[];
  edges: TensorEdge[];
  camera: Camera;
  temporal_slice: number;  // Current t value for time-travel
  epistemic_filter: EpistemicState;  // Filter by q state
}

interface TensorNode {
  id: string;
  position: [number, number, number];  // x, y, z from tensor
  tensor: Tensor10DProjection;
  epistemic_state: EpistemicState;
  version: number;  // t value
}

enum EpistemicState {
  Collapsed,    // q = 0 (ground truth)
  Pending,      // q > 0 (GSR escrow)
  Sandbox       // q > 0 (parallel context)
}
```

### 1.2 scene_to_contract Mapping Layer

**New Module:** `src/scene/tensor_projection.ts`

```typescript
export class TensorProjectionMapper {
  /**
   * Transform 10D Q42 vectors into RenderScene primitives
   */
  static mapTensorToScene(
    quins: NQuin[],
    tensors: Tensor10D[],
    currentT: number
  ): RenderScene {
    const nodes: TensorNode[] = tensors.map((tensor, i) => ({
      id: quins[i].subject.toString(16),
      position: [tensor.x, tensor.y, tensor.z],
      tensor: this.extractProjection(tensor),
      epistemic_state: this.mapEpistemicState(tensor.q),
      version: tensor.t,
    }));

    return {
      nodes,
      edges: this.mapEdges(quins, tensors),
      camera: { /* existing camera logic */ },
      temporal_slice: currentT,
      epistemic_filter: EpistemicState.Collapsed,
    };
  }

  private static extractProjection(tensor: Tensor10D): Tensor10DProjection {
    return {
      q: tensor.q,
      v: tensor.v,
      w: tensor.w,
      x: tensor.x,
      y: tensor.y,
      z: tensor.z,
      t: tensor.t,
      alpha: tensor.alpha,
      mu: tensor.mu,
      sigma: tensor.sigma,
    };
  }

  private static mapEpistemicState(q: number): EpistemicState {
    if (q === 0) return EpistemicState.Collapsed;
    return EpistemicState.Pending;
  }
}
```

### 1.3 Spectral Payload Visual Mapping

**Node Styling Updates:**

| Tensor Dimension | Visual Mapping | Implementation |
|-----------------|----------------|----------------|
| **α (Amplitude)** | Opacity, scale, glow intensity | `node.opacity = tensor.alpha` |
| **σ (Spectral Signature)** | Color mapping | `spectralToRGB(tensor.sigma)` |
| **μ (Modulation)** | Visual noise/steganographic indicator | `node.hasHiddenMetadata = tensor.mu > 0` |

**Spectral to RGB Conversion:**
```typescript
export function spectralToRGB(sigma: number): [number, number, number] {
  // Project spectral data into CIE XYZ
  const xyz = spectralToCIE_XYZ(sigma);
  // Convert to display gamut (sRGB, P3, etc.)
  const rgb = cieXYZToDisplayGamut(xyz, window.displayGamut);
  return rgb;
}
```

---

## Phase 2 - Quantum Interaction UI

**Goal:** Implement wavefunction collapse UI for epistemic states.

### 2.1 Pending Resolution State

**Component:** `src/components/EpistemicStatus.tsx`

```typescript
interface EpistemicStatusProps {
  state: EpistemicState;
  onCollapse?: () => void;
}

export function EpistemicStatus({ state, onCollapse }: EpistemicStatusProps) {
  if (state === EpistemicState.Collapsed) {
    return null;  // No UI for collapsed states
  }

  return (
    <div className="epistemic-pending">
      <div className="pulse-effect">
        <span className="ghosted-node">In Escrow</span>
      </div>
      {onCollapse && (
        <button onClick={onCollapse}>
          Collapse Wavefunction
        </button>
      )}
    </div>
  );
}
```

**CSS Styling:**
```css
.epistemic-pending {
  opacity: 0.6;
  animation: pulse 2s infinite;
}

.pulse-effect {
  box-shadow: 0 0 20px rgba(255, 255, 255, 0.5);
}

@keyframes pulse {
  0%, 100% { opacity: 0.4; }
  50% { opacity: 0.8; }
}
```

### 2.2 Resolution Triggers

**Tauri Command:** `collapse_wavefunction`

```typescript
// src/api/epistemic.ts
export async function collapseWavefunction(
  nodeId: string
): Promise<void> {
  await invoke('collapse_wavefunction', { nodeId });
}
```

**Backend Handler (Rust):**
```rust
#[tauri::command]
async fn collapse_wavefunction(
    node_id: String,
    tensor_db: State<'TensorDatabase'>
) -> Result<(), String> {
    // Promote q > 0 to q = 0
    tensor_db.collapse_context(node_id).await
}
```

### 2.3 Temporal Scrubbing (Time-Travel)

**Component:** `src/components/TemporalScrubber.tsx`

```typescript
interface TemporalScrubberProps {
  currentT: number;
  minT: number;
  maxT: number;
  onScrub: (t: number) => void;
}

export function TemporalScrubber({
  currentT,
  minT,
  maxT,
  onScrub
}: TemporalScrubberProps) {
  return (
    <div className="temporal-scrubber">
      <input
        type="range"
        min={minT}
        max={maxT}
        value={currentT}
        onChange={(e) => onScrub(Number(e.target.value))}
      />
      <span>Time Slice: {currentT}</span>
    </div>
  );
}
```

---

## Phase 3 - Spectral-First Rendering Pipeline

**Goal:** Replace RGB rendering with spectral projection using WebGL/WebGPU shaders.

### 3.1 Shader Updates

**Fragment Shader (GLSL):**

```glsl
#version 450

// Spectral input instead of RGB
layout(location = 0) in float v_alpha;    // Amplitude
layout(location = 1) in float v_mu;       // Modulation
layout(location = 2) in float v_sigma;    // Spectral Signature

layout(location = 0) out vec4 fragColor;

// Color Matching Functions (CIE 1931 2-degree)
const vec3 CMF_R = vec3(0.4124, 0.3576, 0.1805);
const vec3 CMF_G = vec3(0.2126, 0.7152, 0.0722);
const vec3 CMF_B = vec3(0.0193, 0.1192, 0.9505);

vec3 spectralToCIE_XYZ(float sigma) {
    // Project spectral data using Color Matching Functions
    float x = dot(vec3(sigma, sigma * 0.8, sigma * 0.6), CMF_R);
    float y = dot(vec3(sigma, sigma * 0.9, sigma * 0.4), CMF_G);
    float z = dot(vec3(sigma, sigma * 0.7, sigma * 0.5), CMF_B);
    return vec3(x, y, z);
}

vec3 cieXYZToDisplayGamut(vec3 xyz) {
    // Convert to sRGB (or detected display gamut)
    mat3 xyz_to_srgb = mat3(
        3.2406, -1.5372, -0.4986,
        -0.9689, 1.8758, 0.0415,
        0.0557, -0.2040, 1.0570
    );
    return xyz_to_srgb * xyz;
}

void main() {
    // 1. Project spectral to CIE XYZ
    vec3 xyz = spectralToCIE_XYZ(v_sigma);
    
    // 2. Convert to display gamut
    vec3 rgb = cieXYZToDisplayGamut(xyz);
    
    // 3. Apply amplitude (opacity/glow)
    float alpha = clamp(v_alpha, 0.0, 1.0);
    
    // 4. Apply modulation (noise/steganographic indicator)
    if (v_mu > 0.5) {
        rgb += vec3(0.1) * sin(v_mu * 10.0);  // Subtle noise
    }
    
    fragColor = vec4(rgb, alpha);
}
```

### 3.2 WebGPU Implementation

**Pipeline Setup:**

```typescript
export class SpectralRenderPipeline {
  private device: GPUDevice;
  private pipeline: GPURenderPipeline;

  async initialize(device: GPUDevice) {
    this.device = device;
    
    const shaderModule = device.createShaderModule({
      code: SPECTRAL_FRAGMENT_SHADER,
    });

    this.pipeline = device.createRenderPipeline({
      vertex: {
        module: shaderModule,
        entryPoint: 'vertex_main',
      },
      fragment: {
        module: shaderModule,
        entryPoint: 'fragment_main',
        targets: [{
          format: 'bgra8unorm-srgb',  // HDR-capable format
        }],
      },
      primitive: {
        topology: 'triangle-list',
      },
    });
  }

  render(spectralData: Float32Array) {
    // Pass spectral data [α, μ, σ] instead of RGB
    // Zero-copy from ArrayBuffer to GPU buffer
  }
}
```

---

## Phase 4 - Epistemic Capability Handshake

**Goal:** Browser informs backend about hardware capabilities.

### 4.1 Capability Detection

**Module:** `src/capabilities/detector.ts`

```typescript
export interface HardwareCapabilities {
  webgpu_supported: boolean;
  webgpu_adapter: string;
  vram_mb: number;
  power_state: 'battery' | 'plugged';
  thermal_state: 'nominal' | 'throttling';
  tier: HardwareTier;
}

export enum HardwareTier {
  Tier0Edge = 0,      // Mobile, battery constrained
  Tier1Mainstream = 1, // Standard laptop
  Tier2HighPerformance = 2, // Desktop with GPU
  Tier3QPU = 3,        // QPU available
}

export class CapabilityDetector {
  static async detect(): Promise<HardwareCapabilities> {
    const webgpu = await this.detectWebGPU();
    const power = await this.detectPowerState();
    
    return {
      webgpu_supported: webgpu.supported,
      webgpu_adapter: webgpu.adapter,
      vram_mb: webgpu.vram,
      power_state: power.state,
      thermal_state: power.thermal,
      tier: this.determineTier(webgpu, power),
    };
  }

  private static async detectWebGPU() {
    if (!navigator.gpu) {
      return { supported: false, adapter: 'none', vram: 0 };
    }
    
    const adapter = await navigator.gpu.requestAdapter();
    const info = await adapter.requestAdapterInfo();
    
    return {
      supported: true,
      adapter: info.description,
      vram: info.memory || 0,
    };
  }

  private static async detectPowerState() {
    const battery = await navigator.getBattery();
    return {
      state: battery.charging ? 'plugged' : 'battery',
      thermal: 'nominal',  // Navigator API pending
    };
  }

  private static determineTier(
    webgpu: any,
    power: any
  ): HardwareTier {
    if (power.state === 'battery' && webgpu.vram < 4000) {
      return HardwareTier.Tier0Edge;
    }
    if (webgpu.vram >= 8000) {
      return HardwareTier.Tier2HighPerformance;
    }
    return HardwareTier.Tier1Mainstream;
  }
}
```

### 4.2 Handshake with Backend

**Tauri Command:** `register_browser_capabilities`

```typescript
// src/api/capabilities.ts
export async function registerCapabilities(): Promise<void> {
  const caps = await CapabilityDetector.detect();
  await invoke('register_browser_capabilities', { caps });
}
```

**Backend Handler (Rust):**
```rust
#[tauri::command]
async fn register_browser_capabilities(
    caps: BrowserCapabilities,
    dispatcher: State<'HardwareTierDispatcher>
) -> Result<(), String> {
    dispatcher.register_client(caps).await
}
```

### 4.3 Tier-Aware UI

**Component:** `src/components/TierIndicator.tsx`

```typescript
export function TierIndicator({ tier }: { tier: HardwareTier }) {
  const config = {
    [HardwareTier.Tier0Edge]: {
      label: 'Battery Saver',
      icon: '🔋',
      color: 'yellow',
    },
    [HardwareTier.Tier1Mainstream]: {
      label: 'Standard',
      icon: '💻',
      color: 'green',
    },
    [HardwareTier.Tier2HighPerformance]: {
      label: 'High Performance',
      icon: '⚡',
      color: 'blue',
    },
    [HardwareTier.Tier3QPU]: {
      label: 'QPU Accelerated',
      icon: '🔮',
      color: 'purple',
    },
  };

  const cfg = config[tier];

  return (
    <div className={`tier-indicator tier-${cfg.color}`}>
      <span className="icon">{cfg.icon}</span>
      <span className="label">{cfg.label}</span>
    </div>
  );
}
```

---

## Phase 5 - Efficient IPC (Binary Transfers)

**Goal:** Replace JSON serialization with binary ArrayBuffer transfers.

### 5.1 Remove JSON Serialization

**Before:**
```typescript
const scene = await invoke('get_scene', { nodeId });
// Returns: { nodes: [...], edges: [...] } as JSON
```

**After:**
```typescript
const buffer = await invoke('get_scene_binary', { nodeId });
// Returns: ArrayBuffer with packed NQuin structs
```

### 5.2 Binary Transfer Implementation

**Tauri Command:** `get_scene_binary`

```typescript
// src/api/scene.ts
export async function getSceneBinary(nodeId: string): Promise<ArrayBuffer> {
  return await invoke('get_scene_binary', { nodeId });
}
```

**Backend Handler (Rust):**
```rust
#[tauri::command]
fn get_scene_binary(
    node_id: String,
    tensor_db: State<'TensorDatabase>
) -> Result<Vec<u8>, String> {
    // Zero-copy: return memory-mapped tensor data
    tensor_db.get_tensor_binary(node_id)
}
```

### 5.3 Memory-Map Emulation

**Module:** `src/memory/buffer_view.ts`

```typescript
export class TensorBufferView {
  private buffer: ArrayBuffer;
  private dataView: DataView;

  constructor(buffer: ArrayBuffer) {
    this.buffer = buffer;
    this.dataView = new DataView(buffer);
  }

  // Zero-deserialization access
  getTensor(index: number): Tensor10D {
    const offset = index * 40;  // 10 * 4 bytes (f32)
    return {
      q: this.dataView.getFloat32(offset + 0, true),
      v: this.dataView.getFloat32(offset + 4, true),
      w: this.dataView.getFloat32(offset + 8, true),
      x: this.dataView.getFloat32(offset + 12, true),
      y: this.dataView.getFloat32(offset + 16, true),
      z: this.dataView.getFloat32(offset + 20, true),
      t: this.dataView.getFloat32(offset + 24, true),
      alpha: this.dataView.getFloat32(offset + 28, true),
      mu: this.dataView.getFloat32(offset + 32, true),
      sigma: this.dataView.getFloat32(offset + 36, true),
    };
  }

  // Direct GPU buffer upload (zero-copy)
  uploadToGPU(device: GPUDevice): GPUBuffer {
    const gpuBuffer = device.createBuffer({
      size: this.buffer.byteLength,
      usage: GPUBufferUsage.VERTEX | GPUBufferUsage.COPY_DST,
      mappedAtCreation: true,
    });
    
    new Uint8Array(gpuBuffer.getMappedRange()).set(
      new Uint8Array(this.buffer)
    );
    gpuBuffer.unmap();
    
    return gpuBuffer;
  }
}
```

---

## Phase 6 - Sovereign Audio Synthesis Pipeline

**Goal:** Replace `<audio>` tag with AudioWorklet-based spectral synthesis.

### 6.1 Audio Spectral Sheet Struct

**Shared Type Definition:**
```typescript
// src/audio/types.ts
export interface AudioSpectralSheet {
  frequency_bins: Float32Array;  // σ (spectral signature)
  amplitudes: Float32Array;      // α (amplitude)
  phase_data: Float32Array;      // μ (modulation/phase)
  sample_rate: number;
  duration: number;
}
```

### 6.2 AudioWorklet Processor

**File:** `src/audio/worklet/processor.ts`

```typescript
class SpectralSynthesisProcessor extends AudioWorkletProcessor {
  private spectralSheet: AudioSpectralSheet | null = null;
  private phaseAccumulator: Float32Array;
  private outputBuffer: Float32Array;

  constructor() {
    super();
    this.phaseAccumulator = new Float32Array(2048);
    this.outputBuffer = new Float32Array(2048);
  }

  static get parameterDescriptors() {
    return [];
  }

  process(
    inputs: Float32Array[][],
    outputs: Float32Array[][],
    parameters: Record<string, Float32Array>
  ): boolean {
    if (!this.spectralSheet) {
      return true;  // Silent if no data
    }

    const output = outputs[0][0];
    const sheet = this.spectralSheet;

    // Inverse CQT / Inverse STFT synthesis
    for (let i = 0; i < output.length; i++) {
      let sample = 0.0;

      // Sum across frequency bins
      for (let bin = 0; bin < sheet.frequency_bins.length; bin++) {
        const freq = sheet.frequency_bins[bin];
        const amp = sheet.amplitudes[bin];
        const phase = sheet.phase_data[bin];

        // Oscillator synthesis
        this.phaseAccumulator[bin] += 2.0 * Math.PI * freq / sheet.sample_rate;
        sample += amp * Math.sin(this.phaseAccumulator[bin] + phase);
      }

      // Apply amplitude (α)
      output[i] = sample;  // Gain staging applied at output
    }

    return true;
  }

  // Set spectral sheet from main thread
  setSpectralSheet(sheet: AudioSpectralSheet) {
    this.spectralSheet = sheet;
  }
}

registerProcessor('spectral-synthesis', SpectralSynthesisProcessor);
```

### 6.3 Zero-Copy Transport

**Main Thread Setup:**
```typescript
// src/audio/context.ts
export class SovereignAudioContext {
  private audioContext: AudioContext;
  private workletNode: AudioWorkletNode;
  private sharedBuffer: SharedArrayBuffer;

  async initialize() {
    this.audioContext = new AudioContext();
    
    // Load worklet
    await this.audioContext.audioWorklet.addModule(
      '/audio/worklet/processor.ts'
    );

    // Create shared buffer for zero-copy
    this.sharedBuffer = new SharedArrayBuffer(
      4096 * 3 * 4  // 4096 bins * 3 channels * 4 bytes
    );

    this.workletNode = new AudioWorkletNode(
      this.audioContext,
      'spectral-synthesis'
    );

    this.workletNode.connect(this.audioContext.destination);
  }

  // Receive spectral sheet from backend
  async loadSpectralSheet(nodeId: string) {
    const sheet = await invoke<AudioSpectralSheet>(
      'get_audio_spectral_sheet',
      { nodeId }
    );

    // Zero-copy transfer to worklet
    const view = new Float32Array(this.sharedBuffer);
    view.set(sheet.frequency_bins, 0);
    view.set(sheet.amplitudes, 4096);
    view.set(sheet.phase_data, 8192);

    this.workletNode.port.postMessage({
      type: 'set-sheet',
      offset: 0,
    });
  }
}
```

### 6.4 Backend Integration

**Tauri Command:** `get_audio_spectral_sheet`

```typescript
// src/api/audio.ts
export async function getAudioSpectralSheet(
  nodeId: string
): Promise<AudioSpectralSheet> {
  return await invoke('get_audio_spectral_sheet', { nodeId });
}
```

**Backend Handler (Rust):**
```rust
#[tauri::command]
fn get_audio_spectral_sheet(
    node_id: String,
    tensor_db: State<'TensorDatabase>
) -> Result<AudioSpectralSheet, String> {
    // Extract [α, μ, σ] from 10D tensor
    // σ → frequency_bins (CQT/STFT data)
    // α → amplitudes
    // μ → phase_data
    tensor_db.get_audio_spectral_sheet(node_id)
}
```

### 6.5 Audio-Visual Synchronization

**Component:** `src/components/AudioVisualizer.tsx`

```typescript
export function AudioVisualizer({ tensor }: { tensor: Tensor10DProjection }) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Visualize frequency bins (σ) in real-time
    const draw = () => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      
      // Draw spectral bars synced with audio
      const bins = tensor.sigma;  // Frequency data from tensor
      for (let i = 0; i < bins; i++) {
        const height = tensor.alpha * Math.sin(i * 0.1) * 100;
        ctx.fillStyle = `hsl(${i * 10}, 70%, 50%)`;
        ctx.fillRect(i * 10, canvas.height - height, 8, height);
      }

      requestAnimationFrame(draw);
    };

    draw();
  }, [tensor]);

  return <canvas ref={canvasRef} width={400} height={200} />;
}
```

---

## Implementation Order

**Week 1: Phase 1 - Data Contract**
- Day 1-2: Update RenderScene interface
- Day 3-4: Implement TensorProjectionMapper
- Day 5: Test 10D vector mapping

**Week 2: Phase 2 - Quantum UI**
- Day 1-2: Implement EpistemicStatus component
- Day 3: Add collapse wavefunction triggers
- Day 4: Implement temporal scrubber
- Day 5: Test quantum interactions

**Week 3: Phase 3 - Spectral Rendering**
- Day 1-2: Update fragment shaders
- Day 3-4: Implement WebGPU pipeline
- Day 5: Test spectral projection

**Week 4: Phase 4 - Capability Handshake**
- Day 1-2: Implement CapabilityDetector
- Day 3: Add backend handshake
- Day 4: Implement tier-aware UI
- Day 5: Test capability detection

**Week 5: Phase 5 - Binary IPC**
- Day 1-2: Implement binary transfer commands
- Day 3-4: Implement TensorBufferView
- Day 5: Test zero-copy transfers

**Week 6-7: Phase 6 - Audio Pipeline**
- Week 6: AudioWorklet implementation
- Week 7: Integration and testing

---

## Success Criteria

**Phase 1:**
- RenderScene accepts 10D tensor projections
- Spectral payload maps to visual properties
- All tensor dimensions accessible in UI

**Phase 2:**
- Pending states show ghosted/pulse effects
- Wavefunction collapse triggers work
- Temporal scrubbing navigates t-slices

**Phase 3:**
- Shaders use spectral input instead of RGB
- CIE XYZ projection implemented
- Display gamut detection working

**Phase 4:**
- Capability handshake completes on load
- Tier indicator shows correct state
- Backend receives capability profile

**Phase 5:**
- Binary transfers replace JSON
- Zero-copy GPU buffer uploads
- Performance improvement measurable

**Phase 6:**
- AudioWorklet synthesizes from spectral data
- Zero-copy SharedArrayBuffer transfer
- Audio-visual synchronization working

---

## Rollback Plan

Each phase can be rolled back independently:
- Phase 1: Keep old RenderScene interface
- Phase 2: Remove epistemic UI components
- Phase 3: Revert to RGB shaders
- Phase 4: Disable capability handshake
- Phase 5: Revert to JSON transfers
- Phase 6: Keep `<audio>` tag as fallback

Git branches recommended:
- `phase1-10d-contract`
- `phase2-quantum-ui`
- `phase3-spectral-rendering`
- `phase4-capability-handshake`
- `phase5-binary-ipc`
- `phase6-audio-synthesis`

---

## Dependencies

**Required:**
- QualiaDB 0.0.13 backend with 10D tensor support
- Tauri 2.x for IPC
- WebGPU support in target browsers
- AudioWorklet support in target browsers

**External:**
- None - uses only standard Web APIs

---

## Notes

- Browser remains stateless viewport - no 10D math in frontend
- All tensor computation happens in backend
- Browser only projects spectral/temporal truth
- Binary transfers essential for performance with 10D data
- Audio synthesis aligns with visual spectral rendering
- Sovereign audio bypasses lossy codec limitations