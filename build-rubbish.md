# Build Artifacts Directory List

*Generated: 2026-06-16*
*Purpose: List of build artifact directories for cleanup consideration*

## Build Artifact Directories Found

### Main Project Build Artifacts
- `C:\Projects\webizen-browser\target` - Rust build artifacts (debug, release, wasm-dev)
- `C:\Projects\webizen-browser\target\debug\build` - Debug build outputs
- `C:\Projects\webizen-browser\target\release\build` - Release build outputs  
- `C:\Projects\webizen-browser\target\wasm-dev\build` - WASM development build outputs

### Workspace Member Build Artifacts
- `C:\Projects\webizen-browser\webizen-desktop\target` - Desktop app build artifacts
- `C:\Projects\webizen-browser\webizen-studio\target` - Studio build artifacts
- `C:\Projects\webizen-browser\webizen-studio\dist` - Dioxus frontend distribution files

### Legacy Project Build Artifacts
- `C:\Projects\webizen-browser\legacy\build` - Legacy SvelteKit build outputs
- `C:\Projects\webizen-browser\legacy\node_modules` - Legacy Node.js dependencies
- `C:\Projects\webizen-browser\legacy\src-tauri\target` - Legacy Tauri build artifacts
- `C:\Projects\webizen-browser\legacy\webizen-studio\dist` - Legacy studio distribution
- `C:\Projects\webizen-browser\legacy\webizen-studio\target` - Legacy studio build artifacts

## Cleanup Recommendations

### Safe to Remove (Regenerable)
- All `target/` directories (Rust rebuilds from source)
- All `dist/` directories (Dioxus/SvelteKit rebuilds from source)
- `node_modules/` (reinstallable via package managers)

### Caution Required
- `legacy/` directory - appears to be a separate project, verify before cleanup
- Any custom build configurations or cached dependencies

## Cleanup Commands

### Remove Main Project Build Artifacts
```powershell
# Remove Rust build artifacts
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\target"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\webizen-desktop\target"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\webizen-studio\target"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\webizen-studio\dist"
```

### Remove Legacy Project Build Artifacts (Verify First)
```powershell
# Only remove if legacy directory is no longer needed
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\legacy\build"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\legacy\node_modules"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\legacy\src-tauri\target"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\legacy\webizen-studio\dist"
Remove-Item -Recurse -Force "C:\Projects\webizen-browser\legacy\webizen-studio\target"
```

### Standard Rust Cleanup
```powershell
# Clean all Rust build artifacts
cargo clean
```

## Estimated Space Savings

Based on typical Rust and Node.js build artifacts:
- Rust `target/` directories: 500MB - 2GB per project
- Node.js `node_modules/`: 200MB - 500MB
- Dioxus `dist/`: 10MB - 50MB
- **Total estimated savings**: 1GB - 5GB

## Notes

- Build artifacts are safe to remove as they regenerate on next build
- `cargo clean` is the standard way to clean Rust build artifacts
- Always verify legacy directory purpose before cleanup
- Consider using `.gitignore` to prevent committing build artifacts