# RSX One-Off Repair Scripts (archived)

These Python scripts were used during early QApp authoring to bulk-fix a class of
Dioxus RSX errors — inline `{if cond { "#a" } else { "#b" }}` expressions embedded
inside `style:` format strings, which the RSX macro cannot parse.

They are **archived for reference only** and are not part of the build. They hardcode
absolute Windows paths and operate destructively on `webizen-studio/src/components`.
Do **not** re-run them blindly; they reflect a point-in-time state of the codebase.

The proper, durable fix for this class of issue is to avoid inline conditionals inside
format strings entirely (compute the value into a `let` binding first, then interpolate).

| Script | Purpose |
|--------|---------|
| `fix.py`  | Replaced stray `div { value: ... }` and inline-if backgrounds in a few panes |
| `fix2.py` | Flattened conditional `background`/`color` styles to static values |
| `fix3.py` | Removed inline-if button styles in benchmark/extension panes |
| `fix4.py` | (Exploratory) regex sweep for `{if ...}` inside style attributes |
