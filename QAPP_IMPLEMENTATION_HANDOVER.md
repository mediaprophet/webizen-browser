# Multi-Agent Handover: Liberal Arts QApp Implementation

## 🎯 The Mission
The objective is to implement the comprehensive taxonomy of Liberal Arts, Humanities, and Social Sciences as native **QApps** for the Webizen Browser. The underlying logic capabilities for these QApps are already 100% implemented in the `qualia-core-db` engine (e.g., Epistemic logic, Allen Interval Algebra, Neuro-Symbolic sieves, Graph Theory). 

Your job as an incoming agent is to build the front-end Dioxus UI components that expose these capabilities.

## 📁 Critical Context Files (Read These First!)
1. `C:\Projects\qualiaDB\AGENTS.md` - **CRITICAL!** The master coordination layer for the engine. Contains immovable rules (Zero-Heap, 42MB Sentinel), the `PermissiveRoutingLane` routing logic, Bilateral Micro-Commons, and exact constraints on how to bind logic.
2. `C:\Projects\qualiaDB\ARCHITECTURE.md` - The master architecture document outlining systems for DNS-over-HTTPS/DNSSEC, the Permissive Commons ecosystem, PDF parsing rules, and the broader engine design.
3. `C:\Projects\webizen-browser\QUALIA_DB_LOGIC_AUDIT.md` - The exhaustive mapping of all logic engines, solvers, and capabilities available in QualiaDB. You MUST read this so you know exactly what logic systems you can wire your UI components to without re-inventing the wheel!
4. `C:\Projects\webizen-browser\liberal_arts.md` - The raw, exhaustive taxonomy of all fields that need QApps.
5. `C:\Projects\webizen-browser\liberal_arts_qapp_strategy.md` - The architecture strategy and tracking checklist.

## 🤝 Coordination & Tracking Protocol
To prevent duplicate work and ensure systematic progress, all agents MUST follow this protocol:

1. **Claiming a Task:** 
   - Open `C:\Projects\webizen-browser\liberal_arts_qapp_strategy.md`.
   - Find an unchecked task `[ ]`.
   - Before writing any code, modify the file to mark it as in-progress `[/]` so parallel subagents do not collide.
2. **Implementation:**
   - Create the corresponding `<qapp_name>.rs` file inside `C:\Projects\webizen-browser\webizen-studio\src\components\`.
   - Build a visually rich and functional Dioxus component using `rsx!`. Mock the backend QualiaDB engine bindings if they are not yet fully exposed, but ensure the UI provides the full parameters for the underlying logic (e.g., dialectical thesis inputs, epistemic certainty thresholds).
   - Add the component to `components/mod.rs`.
   - Register the component in `components/qapp_dispatcher.rs`.
3. **Completion:**
   - Run `cargo check` inside `webizen-studio` to verify your UI code compiles.
   - Update `liberal_arts_qapp_strategy.md` to mark the task as fully complete `[x]`.
   - Proceed to the next available task, or spawn subagents to work on different categories simultaneously.

## 🤖 Subagent Delegation Strategy
If you have available token-funding, the fastest way to progress is to spawn expert subagents for specific domains. 
For example:
- **"The Philosopher"**: Assign to `ethical_simulator.rs` and `debate_studio.rs`.
- **"The Linguist"**: Assign to `syntax_modeler.rs`.
- **"The Historian"**: Assign to `historiography_mapper.rs`.

Always pass this handover document (`QAPP_IMPLEMENTATION_HANDOVER.md`) in the initial prompt to any subagents you spawn so they understand the coordination protocol.

## 🛑 Strict Rules for Incoming Agents
- **Do NOT re-implement backend logic**. The engine (`qualia-core-db`) is already fully capable. You are building the front-end Dioxus interface.
- **Always update the tracking file** (`liberal_arts_qapp_strategy.md`) immediately to establish a single source of truth for task progress.
- **Ensure Visual Excellence**: The Dioxus components should be dynamic, parameter-rich, and visually engaging. Do not build minimal viable stubs; build premium interfaces.

***
*End of Handover Document. Good luck, Agents!*
