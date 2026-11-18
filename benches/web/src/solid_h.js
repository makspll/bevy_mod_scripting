// src/solid.jsx without Solid's compiler: elements are built by runtime calls (`solid-js/h`), as `ui.node` builds them
// in BMS, instead of cloning templates compiled from JSX. Functions are reactive, as in the compiled version.
import { batch, createMemo, createSelector, createSignal, For, Show } from "solid-js";
import h from "solid-js/h";
import { createStore } from "solid-js/store";
import { render } from "solid-js/web";
import { initialItems, register, SLOTS } from "./harness.js";

const [shown, setShown] = createSignal(false);
const [items, setItems] = createStore(initialItems());
const [hp, setHp] = createSignal({ current: 100, max: 100 });

function Slot(props) {
    return h(
        "div",
        {
            style: () => ({ background: props.selected ? "#ffcc00" : "#333333" }),
            onClick: () => props.onSelect(props.item.name),
        },
        h("span", "x", () => props.item.count),
    );
}

function Inventory() {
    const [selected, setSelected] = createSignal(null);
    const isSelected = createSelector(selected);
    return h(
        "div",
        { style: { display: "flex", "flex-direction": "column" } },
        h("span", "Inventory (", () => items.length, ")"),
        h(
            "div",
            h(For, { each: items }, (item) =>
                h(Slot, { item, selected: () => isSelected(item.name), onSelect: setSelected }),
            ),
        ),
        h(Show, { when: selected }, (name) => h("span", "details: ", name)),
    );
}

function HealthBar() {
    const low = createMemo(() => hp().current < hp().max * 0.25);
    return h(
        "div",
        { style: { display: "flex", "flex-direction": "column" } },
        h("div", {
            style: () => ({
                width: `${(hp().current / hp().max) * 100}%`,
                background: low() ? "#ff0000" : "#00ff00",
            }),
        }),
        h("span", () => hp().current, " / ", () => hp().max),
        h(Show, { when: low }, h("span", "LOW HP")),
    );
}

register({
    setup(scenario) {
        const app = document.getElementById("app");
        if (scenario === "health") {
            render(() => h(HealthBar), app);
        } else {
            render(() => h(Show, { when: shown }, () => h(Inventory)), app);
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
