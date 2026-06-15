#!/usr/bin/env python3
"""
Migrate the academic-discipline QApp components to the shell theme.

Two transforms, both idempotent, applied to every
`webizen-studio/src/components/*_qapp.rs`:

1. THEME (H1): replace the hardcoded Catppuccin Mocha palette with the shell's
   `--qualia-*` CSS variables so each discipline inherits the active theme
   (Human Warmth, etc.) instead of forcing a dark panel.

2. ATTRS (M4): replace the bare `type: "..."` RSX attribute with the idiomatic
   `r#type: "..."` form used by the platform components.

Run from the repo root:  python tools/migrate_qapps.py
"""

import os
import re
import glob

COMPONENTS_DIR = os.path.join(
    os.path.dirname(__file__), "..", "webizen-studio", "src", "components"
)

# Catppuccin Mocha -> qualia theme tokens.
# Structural colors map to surface/bg/border/text; every decorative accent
# collapses to --qualia-accent so contrast tracks the active theme.
COLOR_MAP = {
    # backgrounds (base / mantle / crust)
    "#1e1e2e": "var(--qualia-surface)",
    "#181825": "var(--qualia-bg)",
    "#11111b": "var(--qualia-bg)",
    # borders (surface0 / surface1)
    "#313244": "var(--qualia-border)",
    "#45475a": "var(--qualia-border)",
    # muted text (overlay0 / overlay1 / subtext0 / subtext1)
    "#585b70": "var(--qualia-text-muted)",
    "#6c7086": "var(--qualia-text-muted)",
    "#a6adc8": "var(--qualia-text-muted)",
    "#bac2de": "var(--qualia-text-muted)",
    # primary text
    "#cdd6f4": "var(--qualia-text)",
    # accents (green / blue / yellow / red / mauve / peach / teal / sapphire /
    # lavender / maroon / pink) -> single themed accent
    "#a6e3a1": "var(--qualia-accent)",
    "#89b4fa": "var(--qualia-accent)",
    "#f9e2af": "var(--qualia-accent)",
    "#f38ba8": "var(--qualia-accent)",
    "#cba6f7": "var(--qualia-accent)",
    "#fab387": "var(--qualia-accent)",
    "#94e2d5": "var(--qualia-accent)",
    "#89dceb": "var(--qualia-accent)",
    "#b4befe": "var(--qualia-accent)",
    "#eba0ac": "var(--qualia-accent)",
    "#f5c2e7": "var(--qualia-accent)",
}

# Case-insensitive alternation over the known hex codes only (won't touch other
# hex values that may carry real meaning).
_color_re = re.compile("|".join(re.escape(k) for k in COLOR_MAP), re.IGNORECASE)
# `type: "` not preceded by `#` (r#type) or a word char (prototype).
_type_re = re.compile(r'(?<![#\w])type(\s*:\s*")')


def migrate(text: str) -> str:
    text = _color_re.sub(lambda m: COLOR_MAP[m.group(0).lower()], text)
    text = _type_re.sub(r"r#type\1", text)
    return text


def main() -> None:
    files = sorted(glob.glob(os.path.join(COMPONENTS_DIR, "*_qapp.rs")))
    changed = 0
    for path in files:
        with open(path, "r", encoding="utf-8") as f:
            original = f.read()
        updated = migrate(original)
        if updated != original:
            with open(path, "w", encoding="utf-8") as f:
                f.write(updated)
            changed += 1
    print(f"Processed {len(files)} QApp files; {changed} modified.")


if __name__ == "__main__":
    main()
