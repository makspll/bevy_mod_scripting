"""Merges every `<scenario>_<path>.json` result in a directory into `results.js`.

python3 benches/engines/collect.py <results dir> results.js

The book page loads it from the `bench-media` release with a `<script>` tag: release assets send no CORS headers,
so `fetch` can't read them, but scripts don't need CORS.
"""

import json
import sys
from datetime import datetime, timezone
from pathlib import Path


def main() -> None:
    src, dst = Path(sys.argv[1]), Path(sys.argv[2])
    runs = [json.loads(p.read_text()) for p in sorted(src.rglob("s*_*.json"))]
    if not runs:
        sys.exit(f"no results in {src}")

    # every path must have simulated the same thing as the Rust baseline
    for scenario in ("s1", "s2"):
        base = next((r for r in runs if r["scenario"] == scenario and r["path"] == "rust"), None)
        for run in runs:
            if base is None or run["scenario"] != scenario:
                continue
            run["verified"] = verify(base, run)
            if not run["verified"]:
                print(f"warning: {run['engine']}/{run['path']} {scenario} diverged from the baseline", file=sys.stderr)

    dst.parent.mkdir(parents=True, exist_ok=True)
    dst.write_text("window.ENGINE_RESULTS = " + json.dumps({
        "generated": datetime.now(timezone.utc).isoformat(timespec="seconds"),
        "runs": runs,
    }) + ";\n")


def verify(base: dict, run: dict) -> bool:
    if run["scenario"] == "s1":
        a, b = base["checksum"], run["checksum"]
        return a["alive"] == b["alive"] and abs(a["score"] - b["score"]) <= 0.01 * a["score"] \
            and abs(a["sum"] - b["sum"]) <= 0.01 * abs(a["sum"])
    return all(abs(x - y) <= max(5, 0.05 * x) for x, y in zip(base["infected"], run["infected"]))


if __name__ == "__main__":
    main()
