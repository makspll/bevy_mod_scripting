# Script integration test harness

Tests are script files under `assets/tests/**`, driven by `.bmsscenario` files. Add a script (Lua and Rhai) plus a scenario rather than a Rust test when checking script-facing behaviour.

* Run: `cargo test --test script_tests --features lua54,rhai -- <name>`. Slow first build, run it in the background.
* Look at an existing scenario with both languages before writing a new one; copy its structure.
* Lua and Rhai differ in syntax and indexing; check a sibling test in the other language.
