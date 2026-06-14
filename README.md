# webizen-browser

Implementation using QualiaDB.

**Author:** Timothy Charles Holborn  
**Contact:** timothy.holborn@gmail.com  
**LinkedIn:** https://www.linkedin.com/in/ubiquitous/

## License

Copyright (c) 2026 Timothy Charles Holborn  
Attribution-NonCommercial-NoDerivatives 4.0 International  
See [LICENSE](./LICENSE) for full terms.

## Webizen Studio

A Dioxus/wasm32 application providing 244+ interactive QApp components across all academic disciplines, backed by QualiaDB's epistemic graph engine.

**Demo:** https://mediaprophet.github.io/webizen-browser/

### Build

```bash
cd webizen-studio && dx serve          # dev server
cd webizen-studio && dx build --release  # wasm32 release
```

Requires [Dioxus CLI](https://dioxuslabs.com/) and the `wasm32-unknown-unknown` Rust target.
