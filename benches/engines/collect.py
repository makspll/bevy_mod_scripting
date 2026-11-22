"""Verifies every `<scenario>_<path>.json` result in a directory and renders one chart per scenario.

python3 benches/engines/collect.py <results dir> <out dir>

Writes `<out dir>/s1.png` and `<out dir>/s2.png`. They are uploaded to the `bench-media` release, where the book's
Engine Comparison page embeds them.
"""

import json
import sys
from datetime import datetime, timezone
from pathlib import Path

import matplotlib

matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

TITLES = {"s1": "S1 Fountain", "s2": "S2 Infection"}


def main() -> None:
    src, out = Path(sys.argv[1]), Path(sys.argv[2])
    runs = [json.loads(p.read_text()) for p in sorted(src.rglob("s*_*.json"))]
    if not runs:
        sys.exit(f"no results in {src}")
    out.mkdir(parents=True, exist_ok=True)
    generated = datetime.now(timezone.utc).strftime("%Y-%m-%d %H:%M UTC")

    for scenario, title in TITLES.items():
        scenario_runs = [r for r in runs if r["scenario"] == scenario]
        if not scenario_runs:
            continue
        # every path must have simulated the same thing as the Rust baseline
        base = next((r for r in scenario_runs if r["path"] == "rust"), None)
        phases = ["frame"] + [k for k in scenario_runs[0]["stages"][0] if k not in ("target", "alive", "frame")]

        fig, axes = plt.subplots(2, 2, figsize=(12, 9), constrained_layout=True)
        for ax, phase in zip(axes.flat, phases):
            for run in scenario_runs:
                label = f"{run['engine']}/{run['path']}"
                if base is not None and not verify(base, run):
                    label += " (diverged)"
                    print(f"warning: {label} {scenario} diverged from the baseline", file=sys.stderr)
                stages = run["stages"]
                ax.plot([s["target"] for s in stages], [s[phase]["median"] for s in stages], marker="o", label=label)
            ax.set(title=f"{phase} (median ms)", xscale="log", yscale="log", xlabel="entities", ylabel="ms")
            ax.grid(True, which="both", alpha=0.3)
        axes.flat[0].legend(fontsize="small")
        fig.suptitle(f"{title}, {generated}")
        fig.savefig(out / f"{scenario}.png", dpi=110)
        plt.close(fig)


def verify(base: dict, run: dict) -> bool:
    if run["scenario"] == "s1":
        a, b = base["checksum"], run["checksum"]
        return a["alive"] == b["alive"] and abs(a["score"] - b["score"]) <= 0.01 * a["score"] \
            and abs(a["sum"] - b["sum"]) <= 0.01 * abs(a["sum"])
    return all(abs(x - y) <= max(5, 0.05 * x) for x, y in zip(base["infected"], run["infected"]))


if __name__ == "__main__":
    main()
