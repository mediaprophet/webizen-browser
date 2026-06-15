#!/usr/bin/env python3
"""
Catalogue <-> dispatcher consistency check.

Every academic-discipline QApp listed in `qapps.rs` (`cat: Cat::Academic`) must
have a matching arm in `qapp_dispatcher.rs`, otherwise its "Open in Studio" card
dead-ends (this is the class of bug that the physics-sim / physics-simulator id
mismatch was). Run in CI so the invariant is enforced on every push.

Exit code 0 = consistent, 1 = mismatches found.
Run from repo root:  python tools/check_qapp_consistency.py
"""

import os
import re
import sys

BASE = os.path.join(
    os.path.dirname(__file__), "..", "webizen-studio", "src", "components"
)
QAPPS = os.path.join(BASE, "qapps.rs")
DISPATCHER = os.path.join(BASE, "qapp_dispatcher.rs")

ID_RE = re.compile(r'id:\s*"([^"]+)"')
ARM_RE = re.compile(r'^\s*"([^"]+)"\s*=>')
# `QApp { ... }` literal body (no nested braces in the field values).
QAPP_RE = re.compile(r"QApp\s*\{(.*?)\}", re.DOTALL)


def academic_ids(text: str) -> list[str]:
    ids = []
    for body in QAPP_RE.findall(text):
        if "cat: Cat::Academic" in body:
            m = ID_RE.search(body)
            if m:
                ids.append(m.group(1))
    return ids


def dispatcher_tags(text: str) -> set[str]:
    tags = set()
    for line in text.splitlines():
        m = ARM_RE.match(line)
        if m:
            tags.add(m.group(1))
    return tags


def main() -> int:
    with open(QAPPS, encoding="utf-8") as f:
        qapps = f.read()
    with open(DISPATCHER, encoding="utf-8") as f:
        dispatcher = f.read()

    ids = academic_ids(qapps)
    tags = dispatcher_tags(dispatcher)

    missing = [i for i in ids if i not in tags]
    dupes = sorted({i for i in ids if ids.count(i) > 1})

    print(f"Academic catalogue entries: {len(ids)}")
    print(f"Dispatcher arms:            {len(tags)}")

    ok = True
    if missing:
        ok = False
        print(f"\nERROR: {len(missing)} Academic QApp(s) with no dispatcher arm:")
        for m in missing:
            print(f"  - {m}")
    if dupes:
        ok = False
        print(f"\nERROR: duplicate catalogue ids: {dupes}")

    if ok:
        print("\nOK: every Academic QApp has a dispatcher arm.")
        return 0
    return 1


if __name__ == "__main__":
    sys.exit(main())
