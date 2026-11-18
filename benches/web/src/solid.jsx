// The reactive_ui benchmark ui (assets/macro_benchmarks/reactive_ui/closures.lua) in idiomatic Solid:
// a store for the items, `createSelector` for the selection, compiled JSX. Solid updates the DOM synchronously.
import { batch, createMemo, createSelector, createSignal, For, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { render } from "solid-js/web";
import { initialItems, register, SLOTS } from "./harness.js";

const [shown, setShown] = createSignal(false);
const [items, setItems] = createStore(initialItems());
const [hp, setHp] = createSignal({ current: 100, max: 100 });

function Slot(props) {
    return (
        <div
            style={{ background: props.selected ? "#ffcc00" : "#333333" }}
            onClick={() => props.onSelect(props.item.name)}
        >
            <span>x{props.item.count}</span>
        </div>
    );
}

function Inventory() {
    const [selected, setSelected] = createSignal(null);
    const isSelected = createSelector(selected);
    return (
        <div style={{ display: "flex", "flex-direction": "column" }}>
            <span>Inventory ({items.length})</span>
            <div>
                <For each={items}>
                    {(item) => (
                        <Slot item={item} selected={isSelected(item.name)} onSelect={setSelected} />
                    )}
                </For>
            </div>
            <Show when={selected()}>{(name) => <span>details: {name()}</span>}</Show>
        </div>
    );
}

function HealthBar() {
    const low = createMemo(() => hp().current < hp().max * 0.25);
    return (
        <div style={{ display: "flex", "flex-direction": "column" }}>
            <div
                style={{
                    width: `${(hp().current / hp().max) * 100}%`,
                    background: low() ? "#ff0000" : "#00ff00",
                }}
            />
            <span>
                {hp().current} / {hp().max}
            </span>
            <Show when={low()}>
                <span>LOW HP</span>
            </Show>
        </div>
    );
}

register({
    setup(scenario) {
        const app = document.getElementById("app");
        if (scenario === "health") {
            render(() => <HealthBar />, app);
        } else {
            render(() => <Show when={shown()}><Inventory /></Show>, app);
            if (scenario !== "mount") setShown(true);
        }
    },
    op(scenario, n) {
        switch (scenario) {
            case "mount": {
                const show = n % 2 === 0;
                return () => setShown(show);
            }
            case "health": {
                const value = { current: 50 + (n % 2), max: 100 };
                return () => setHp(value);
            }
            case "all":
                return () =>
                    batch(() => {
                        for (let i = 0; i < SLOTS; i++) setItems(i, "count", n);
                    });
        }
    },
});
