// The reactive_ui benchmark ui (assets/macro_benchmarks/reactive_ui/closures.lua) in idiomatic React:
// memoised slots, stable callbacks, updates committed synchronously with `flushSync`.
import { memo, useCallback, useState } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";
import { initialItems, register } from "./harness.js";

let setShown, setItems, setHp;
let items = initialItems();

const Slot = memo(function Slot({ name, count, selected, onSelect }) {
    return (
        <div
            style={{ background: selected ? "#ffcc00" : "#333333" }}
            onClick={() => onSelect(name)}
        >
            <span>x{count}</span>
        </div>
    );
});

function Inventory() {
    const [list, set] = useState(items);
    setItems = set;
    const [selected, setSelected] = useState(null);
    const onSelect = useCallback((name) => setSelected(name), []);
    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <span>Inventory ({list.length})</span>
            <div>
                {list.map((item) => (
                    <Slot
                        key={item.name}
                        name={item.name}
                        count={item.count}
                        selected={selected === item.name}
                        onSelect={onSelect}
                    />
                ))}
            </div>
            {selected !== null && <span>details: {selected}</span>}
        </div>
    );
}

function Root() {
    const [shown, set] = useState(false);
    setShown = set;
    return shown ? <Inventory /> : null;
}

function HealthBar() {
    const [hp, set] = useState({ current: 100, max: 100 });
    setHp = set;
    const low = hp.current < hp.max * 0.25;
    return (
        <div style={{ display: "flex", flexDirection: "column" }}>
            <div
                style={{
                    width: `${(hp.current / hp.max) * 100}%`,
                    background: low ? "#ff0000" : "#00ff00",
                }}
            />
            <span>
                {hp.current} / {hp.max}
            </span>
            {low && <span>LOW HP</span>}
        </div>
    );
}

register({
    setup(scenario) {
        const root = createRoot(document.getElementById("app"));
        if (scenario === "health") {
            flushSync(() => root.render(<HealthBar />));
        } else {
            flushSync(() => root.render(<Root />));
            if (scenario !== "mount") flushSync(() => setShown(true));
        }
    },
    op(scenario, n) {
        switch (scenario) {
            case "mount": {
                const show = n % 2 === 0;
                return () => flushSync(() => setShown(show));
            }
            case "health": {
                const hp = { current: 50 + (n % 2), max: 100 };
                return () => flushSync(() => setHp(hp));
            }
            case "all": {
                items = items.map((item) => ({ name: item.name, count: n }));
                const next = items;
                return () => flushSync(() => setItems(next));
            }
        }
    },
});
